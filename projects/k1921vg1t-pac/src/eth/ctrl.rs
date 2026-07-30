#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TXEN` reader - Transmit enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmit enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receive enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receive enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINT` reader - Transmitter interrupt"]
pub type TxintR = crate::BitReader;
#[doc = "Field `TXINT` writer - Transmitter interrupt"]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINT` reader - Receiver interrupt"]
pub type RxintR = crate::BitReader;
#[doc = "Field `RXINT` writer - Receiver interrupt"]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FD` reader - Full duplex"]
pub type FdR = crate::BitReader;
#[doc = "Field `FD` writer - Full duplex"]
pub type FdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR` reader - Promiscuous mode"]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Promiscuous mode"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Reset"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - Speed"]
pub type SpeedR = crate::BitReader;
#[doc = "Field `SPEED` writer - Speed"]
pub type SpeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GB` reader - Gigabit"]
pub type GbR = crate::BitReader;
#[doc = "Field `GB` writer - Gigabit"]
pub type GbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM` reader - Burstmode"]
pub type BmR = crate::BitReader;
#[doc = "Field `BM` writer - Burstmode"]
pub type BmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI` reader - PHY status change irq enable"]
pub type PiR = crate::BitReader;
#[doc = "Field `PI` writer - PHY status change irq enable"]
pub type PiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ME` reader - Multicast enable"]
pub type MeR = crate::BitReader;
#[doc = "Field `ME` writer - Multicast enable"]
pub type MeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC` reader - Multicast available"]
pub type McR = crate::BitReader;
#[doc = "Field `MDIOEN` reader - MDIO interrupt enabled"]
pub type MdioenR = crate::BitReader;
#[doc = "Field `GA` reader - Gigabit MAC available"]
pub type GaR = crate::BitReader;
#[doc = "Field `BS` reader - EDCL buffer size"]
pub type BsR = crate::FieldReader;
#[doc = "Field `EA` reader - EDCL available"]
pub type EaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter interrupt"]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver interrupt"]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Full duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FdR {
        FdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Promiscuous mode"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Gigabit"]
    #[inline(always)]
    pub fn gb(&self) -> GbR {
        GbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Burstmode"]
    #[inline(always)]
    pub fn bm(&self) -> BmR {
        BmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PHY status change irq enable"]
    #[inline(always)]
    pub fn pi(&self) -> PiR {
        PiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Multicast enable"]
    #[inline(always)]
    pub fn me(&self) -> MeR {
        MeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 25 - Multicast available"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MDIO interrupt enabled"]
    #[inline(always)]
    pub fn mdioen(&self) -> MdioenR {
        MdioenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Gigabit MAC available"]
    #[inline(always)]
    pub fn ga(&self) -> GaR {
        GaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - EDCL buffer size"]
    #[inline(always)]
    pub fn bs(&self) -> BsR {
        BsR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - EDCL available"]
    #[inline(always)]
    pub fn ea(&self) -> EaR {
        EaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, CtrlSpec> {
        TxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, CtrlSpec> {
        RxenW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter interrupt"]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, CtrlSpec> {
        TxintW::new(self, 2)
    }
    #[doc = "Bit 3 - Receiver interrupt"]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, CtrlSpec> {
        RxintW::new(self, 3)
    }
    #[doc = "Bit 4 - Full duplex"]
    #[inline(always)]
    pub fn fd(&mut self) -> FdW<'_, CtrlSpec> {
        FdW::new(self, 4)
    }
    #[doc = "Bit 5 - Promiscuous mode"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<'_, CtrlSpec> {
        PrW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<'_, CtrlSpec> {
        RstW::new(self, 6)
    }
    #[doc = "Bit 7 - Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, CtrlSpec> {
        SpeedW::new(self, 7)
    }
    #[doc = "Bit 8 - Gigabit"]
    #[inline(always)]
    pub fn gb(&mut self) -> GbW<'_, CtrlSpec> {
        GbW::new(self, 8)
    }
    #[doc = "Bit 9 - Burstmode"]
    #[inline(always)]
    pub fn bm(&mut self) -> BmW<'_, CtrlSpec> {
        BmW::new(self, 9)
    }
    #[doc = "Bit 10 - PHY status change irq enable"]
    #[inline(always)]
    pub fn pi(&mut self) -> PiW<'_, CtrlSpec> {
        PiW::new(self, 10)
    }
    #[doc = "Bit 11 - Multicast enable"]
    #[inline(always)]
    pub fn me(&mut self) -> MeW<'_, CtrlSpec> {
        MeW::new(self, 11)
    }
}
#[doc = "Ethernet control and capabilities register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
