#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

// ============================================================================
//  ЦЕРЕМОНИЯ №1 — UART (готовый). Вывод в терминал COM4/COM5, 9600 8-N-1.
// ============================================================================
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

// печать 32-битного числа в шестнадцатеричном виде (8 цифр)
fn uart0_send_hex32(p: &pac::Peripherals, n: u32) {
    let hex = b"0123456789ABCDEF";
    for shift in (0..8).rev() {
        let nibble = ((n >> (shift * 4)) & 0xF) as usize;
        uart0_send_byte(p, hex[nibble]);
    }
    uart0_send_byte(p, b' ');
}

// печать блока из 4 слов (128 бит) в hex
fn print_block(p: &pac::Peripherals, label: &str, block: &[u32; 4]) {
    uart0_send_str(p, label);
    for &w in block.iter() {
        uart0_send_hex32(p, w);
    }
    uart0_send_str(p, "\r\n");
}

// ============================================================================
//  ЦЕРЕМОНИЯ №2 — блок CRYPTO. Шифрование/дешифрование ОДНОГО блока (128 бит)
//  в режиме ECB. algorithm: 0=AES-128, 1=AES-256, 2=Магма, 3=Кузнечик.
//  Слова ключа/данных грузим КАК ЕСТЬ, без разворота байт (как в драйвере НИИЭТ);
//  проверено эталонным вектором ГОСТ 34.12-2015 — совпадает побитно.
// ============================================================================
fn crypto_process(
    p: &pac::Peripherals,
    algorithm: u8,
    decrypt: bool,
    key: &[u32; 8],       // ключ 256 бит = 8 слов
    data_in: &[u32; 4],   // блок данных 128 бит = 4 слова
) -> [u32; 4] {
    // включить блок CRYPTO
    p.rcu.cgcfgahb().modify(|_, w| w.cryptoen().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.cryptoen().set_bit());

    // загрузить ключ (8 слов, как в штатном драйвере НИИЭТ — без разворота байт)
    for i in 0..8 {
        p.crypto.key(i).value().write(|w| unsafe { w.val().bits(key[i]) });
    }
    // загрузить входные данные (4 слова)
    for i in 0..4 {
        p.crypto.text_in(i).value().write(|w| unsafe { w.val().bits(data_in[i]) });
    }

    // настроить и запустить: ECB, выбранный алгоритм, шифр/дешифр, START
    p.crypto.control().write(|w| {
        w.self_update().clear_bit();
        unsafe { w.mode().bits(0); } // 0 = ECB
        unsafe { w.algorithm().bits(algorithm); }
        if decrypt {
            w.direction().set_bit(); // 1 = дешифрование
        } else {
            w.direction().clear_bit(); // 0 = шифрование
        }
        w.update_key().clear_bit();
        w.start().set_bit()
    });

    // ждать готовности ключей, затем результата
    while p.crypto.status().read().keys_ready().bit_is_clear() {}
    while p.crypto.status().read().ready().bit_is_clear() {}

    // прочитать результат (4 слова, без разворота байт)
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

    // ГОСТ 34.12-2015 «Кузнечик» — ОФИЦИАЛЬНЫЙ тестовый вектор из стандарта.
    // ключ 256 бит (8 слов) — из стандарта
    let key: [u32; 8] = [
        0x8899AABB, 0xCCDDEEFF, 0x00112233, 0x44556677,
        0xFEDCBA98, 0x76543210, 0x01234567, 0x89ABCDEF,
    ];
    // открытый текст (блок 128 бит) — из стандарта: 1122334455667700ffeeddccbbaa9988
    let plaintext: [u32; 4] = [0x11223344, 0x55667700, 0xFFEEDDCC, 0xBBAA9988];
    // эталонный шифртекст из стандарта: 7f679d90bebc24305a468d42b9d4edcd
    let expected: [u32; 4] = [0x7F679D90, 0xBEBC2430, 0x5A468D42, 0xB9D4EDCD];

    print_block(&p, "Plaintext:  ", &plaintext);

    let cipher = crypto_process(&p, KUZNECHIK, false, &key, &plaintext);  // зашифровать
    print_block(&p, "Ciphertext: ", &cipher);
    print_block(&p, "Expected:   ", &expected);

    // главная проверка: совпал ли аппаратный результат с эталоном ГОСТ
    if cipher == expected {
        uart0_send_str(&p, "MATCH: sovpalo s GOST test-vector\r\n");
    } else {
        uart0_send_str(&p, "MISMATCH: ne sovpalo s GOST\r\n");
    }

    let decrypted = crypto_process(&p, KUZNECHIK, true, &key, &cipher);   // расшифровать
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
