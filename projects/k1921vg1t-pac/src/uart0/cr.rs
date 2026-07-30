#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `UARTEN` reader - UART enable"]
pub type UartenR = crate::BitReader;
#[doc = "Field `UARTEN` writer - UART enable"]
pub type UartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIREN` reader - Enable IrDA SIR ENDEC"]
pub type SirenR = crate::BitReader;
#[doc = "Field `SIREN` writer - Enable IrDA SIR ENDEC"]
pub type SirenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRLP` reader - Enable SIR low-power IrDA mode"]
pub type SirlpR = crate::BitReader;
#[doc = "Field `SIRLP` writer - Enable SIR low-power IrDA mode"]
pub type SirlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBE` reader - Loopback enable"]
pub type LbeR = crate::BitReader;
#[doc = "Field `LBE` writer - Loopback enable"]
pub type LbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - Transmit enable"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - Transmit enable"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - Receive enable"]
pub type RxeR = crate::BitReader;
#[doc = "Field `RXE` writer - Receive enable"]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR` reader - Data transmit ready"]
pub type DtrR = crate::BitReader;
#[doc = "Field `DTR` writer - Data transmit ready"]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - Request to send"]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - Request to send"]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - RTS hardware flow control enable"]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - RTS hardware flow control enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTS hardware flow control enable"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS hardware flow control enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART enable"]
    #[inline(always)]
    pub fn uarten(&self) -> UartenR {
        UartenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable IrDA SIR ENDEC"]
    #[inline(always)]
    pub fn siren(&self) -> SirenR {
        SirenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable SIR low-power IrDA mode"]
    #[inline(always)]
    pub fn sirlp(&self) -> SirlpR {
        SirlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Loopback enable"]
    #[inline(always)]
    pub fn lbe(&self) -> LbeR {
        LbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit enable"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive enable"]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmit ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Request to send"]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART enable"]
    #[inline(always)]
    pub fn uarten(&mut self) -> UartenW<'_, CrSpec> {
        UartenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable IrDA SIR ENDEC"]
    #[inline(always)]
    pub fn siren(&mut self) -> SirenW<'_, CrSpec> {
        SirenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable SIR low-power IrDA mode"]
    #[inline(always)]
    pub fn sirlp(&mut self) -> SirlpW<'_, CrSpec> {
        SirlpW::new(self, 2)
    }
    #[doc = "Bit 7 - Loopback enable"]
    #[inline(always)]
    pub fn lbe(&mut self) -> LbeW<'_, CrSpec> {
        LbeW::new(self, 7)
    }
    #[doc = "Bit 8 - Transmit enable"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, CrSpec> {
        TxeW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive enable"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<'_, CrSpec> {
        RxeW::new(self, 9)
    }
    #[doc = "Bit 10 - Data transmit ready"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<'_, CrSpec> {
        DtrW::new(self, 10)
    }
    #[doc = "Bit 11 - Request to send"]
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<'_, CrSpec> {
        RtsW::new(self, 11)
    }
    #[doc = "Bit 14 - RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<'_, CrSpec> {
        RtsenW::new(self, 14)
    }
    #[doc = "Bit 15 - CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, CrSpec> {
        CtsenW::new(self, 15)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x0300"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x0300;
}
