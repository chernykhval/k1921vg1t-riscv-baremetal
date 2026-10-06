#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

// UART0 output, 9600 8N1 from the 27 MHz HSE crystal (same as rust_uart).
fn uart0_init(p: &pac::Peripherals) {
    p.rcu.hsecfg().modify(|_, w| w.en().set_bit());
    for _ in 0..100_000 {
        unsafe { core::arch::asm!("nop") };
    }
    p.rcu.cgcfgahb().modify(|_, w| w.gpioben().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.gpioben().set_bit());
    p.rcu.cgcfgapb2().modify(|_, w| w.uart0en().set_bit());
    p.rcu.rstdisapb2().modify(|_, w| w.uart0en().set_bit());
    p.gpiob.altfuncnum0().modify(|_, w| w.pin0().af3().pin1().af3());
    p.gpiob.altfuncset().write(|w| w.pin0().set_bit().pin1().set_bit());
    p.rcu.uartcfg(0).value().modify(|_, w| {
        w.clksel().srcclk();
        w.clken().set_bit();
        w.rstdis().set_bit()
    });
    p.uart0.ibrd().write(|w| unsafe { w.divint().bits(175) });
    p.uart0.fbrd().write(|w| unsafe { w.divfrac().bits(50) });
    p.uart0.lcrh().modify(|_, w| w.wlen()._8bit().fen().set_bit());
    p.uart0.cr().modify(|_, w| w.txe().set_bit().rxe().set_bit().uarten().set_bit());
}

fn uart0_send_byte(p: &pac::Peripherals, byte: u8) {
    while p.uart0.fr().read().txff().bit_is_set() {}
    p.uart0.dr().write(|w| unsafe { w.data().bits(byte) });
}

fn uart0_send_str(p: &pac::Peripherals, s: &str) {
    for b in s.bytes() {
        uart0_send_byte(p, b);
    }
}

// Hexadecimal output of a u32 (8 digits).
fn uart0_send_hex32(p: &pac::Peripherals, n: u32) {
    let hex = b"0123456789ABCDEF";
    for shift in (0..8).rev() {
        let nibble = ((n >> (shift * 4)) & 0xF) as usize;
        uart0_send_byte(p, hex[nibble]);
    }
    uart0_send_byte(p, b' ');
}

// Print a 128-bit block (4 words) in hex.
fn print_block(p: &pac::Peripherals, label: &str, block: &[u32; 4]) {
    uart0_send_str(p, label);
    for &w in block.iter() {
        uart0_send_hex32(p, w);
    }
    uart0_send_str(p, "\r\n");
}

// Encrypt or decrypt one 128-bit block with the CRYPTO block in ECB mode.
// ALGORITHM values from the SVD: 0 = AES-128, 1 = AES-256, 3 = GOST Kuznyechik.
// Key and data words are loaded as is, without byte swapping (as in the NIIET driver);
// verified against the GOST R 34.12-2015 test vector.
fn crypto_process(
    p: &pac::Peripherals,
    algorithm: u8,
    decrypt: bool,
    key: &[u32; 8],       // 256-bit key
    data_in: &[u32; 4],   // 128-bit block
) -> [u32; 4] {
    p.rcu.cgcfgahb().modify(|_, w| w.cryptoen().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.cryptoen().set_bit());

    for i in 0..8 {
        p.crypto.key(i).value().write(|w| unsafe { w.val().bits(key[i]) });
    }
    for i in 0..4 {
        p.crypto.text_in(i).value().write(|w| unsafe { w.val().bits(data_in[i]) });
    }

    p.crypto.control().write(|w| {
        w.self_update().clear_bit();
        unsafe { w.mode().bits(0); } // 0 = ECB
        unsafe { w.algorithm().bits(algorithm); }
        if decrypt {
            w.direction().set_bit(); // 1 = decrypt
        } else {
            w.direction().clear_bit(); // 0 = encrypt
        }
        w.update_key().clear_bit();
        w.start().set_bit()
    });

    // Wait for the key schedule, then for the result.
    while p.crypto.status().read().keys_ready().bit_is_clear() {}
    while p.crypto.status().read().ready().bit_is_clear() {}

    let mut out = [0u32; 4];
    for i in 0..4 {
        out[i] = p.crypto.text_out(i).value().read().val().bits();
    }
    out
}

const KUZNECHIK: u8 = 3;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };
    uart0_init(&p);

    // Official Kuznyechik test vector from GOST R 34.12-2015.
    let key: [u32; 8] = [
        0x8899AABB, 0xCCDDEEFF, 0x00112233, 0x44556677,
        0xFEDCBA98, 0x76543210, 0x01234567, 0x89ABCDEF,
    ];
    // Plaintext 1122334455667700ffeeddccbbaa9988
    let plaintext: [u32; 4] = [0x11223344, 0x55667700, 0xFFEEDDCC, 0xBBAA9988];
    // Expected ciphertext 7f679d90bebc24305a468d42b9d4edcd
    let expected: [u32; 4] = [0x7F679D90, 0xBEBC2430, 0x5A468D42, 0xB9D4EDCD];

    print_block(&p, "Plaintext:  ", &plaintext);

    let cipher = crypto_process(&p, KUZNECHIK, false, &key, &plaintext);
    print_block(&p, "Ciphertext: ", &cipher);
    print_block(&p, "Expected:   ", &expected);

    if cipher == expected {
        uart0_send_str(&p, "MATCH: sovpalo s GOST test-vector\r\n");
    } else {
        uart0_send_str(&p, "MISMATCH: ne sovpalo s GOST\r\n");
    }

    let decrypted = crypto_process(&p, KUZNECHIK, true, &key, &cipher);
    print_block(&p, "Decrypted:  ", &decrypted);

    if decrypted == plaintext {
        uart0_send_str(&p, "OK: decrypted == plaintext\r\n");
    } else {
        uart0_send_str(&p, "FAIL round-trip\r\n");
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
