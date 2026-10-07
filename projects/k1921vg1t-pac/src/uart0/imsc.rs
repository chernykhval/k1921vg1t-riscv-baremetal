#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RIMIM` reader - nUARTRI modem interrupt mask"]
pub type RimimR = crate::BitReader;
#[doc = "Field `RIMIM` writer - nUARTRI modem interrupt mask"]
pub type RimimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIM` reader - nUARTCTS modem interrupt mask"]
pub type CtsmimR = crate::BitReader;
#[doc = "Field `CTSMIM` writer - nUARTCTS modem interrupt mask"]
pub type CtsmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMIM` reader - nUARTDCD modem interrupt mask"]
pub type DcdmimR = crate::BitReader;
#[doc = "Field `DCDMIM` writer - nUARTDCD modem interrupt mask"]
pub type DcdmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMIM` reader - nUARTDSR modem interrupt mask"]
pub type DsrmimR = crate::BitReader;
#[doc = "Field `DSRMIM` writer - nUARTDSR modem interrupt mask"]
pub type DsrmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - Receive interrupt mask"]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - Receive interrupt mask"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - Transmit interrupt mask"]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - Transmit interrupt mask"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask"]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERIM` reader - Framing error interrupt mask"]
pub type FerimR = crate::BitReader;
#[doc = "Field `FERIM` writer - Framing error interrupt mask"]
pub type FerimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIM` reader - Parity error interrupt mask"]
pub type PerimR = crate::BitReader;
#[doc = "Field `PERIM` writer - Parity error interrupt mask"]
pub type PerimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERIM` reader - Break error interrupt mask"]
pub type BerimR = crate::BitReader;
#[doc = "Field `BERIM` writer - Break error interrupt mask"]
pub type BerimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OERIM` reader - Overrun error interrupt mask"]
pub type OerimR = crate::BitReader;
#[doc = "Field `OERIM` writer - Overrun error interrupt mask"]
pub type OerimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIM` reader - Transmit done interrupt mask"]
pub type TdimR = crate::BitReader;
#[doc = "Field `TDIM` writer - Transmit done interrupt mask"]
pub type TdimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&self) -> RimimR {
        RimimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask"]
    #[inline(always)]
    pub fn ctsmim(&self) -> CtsmimR {
        CtsmimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DcdmimR {
        DcdmimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DsrmimR {
        DsrmimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn ferim(&self) -> FerimR {
        FerimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn perim(&self) -> PerimR {
        PerimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn berim(&self) -> BerimR {
        BerimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oerim(&self) -> OerimR {
        OerimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit done interrupt mask"]
    #[inline(always)]
    pub fn tdim(&self) -> TdimR {
        TdimR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask"]
    #[inline(always)]
    pub fn rimim(&mut self) -> RimimW<'_, ImscSpec> {
        RimimW::new(self, 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask"]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CtsmimW<'_, ImscSpec> {
        CtsmimW::new(self, 1)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask"]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DcdmimW<'_, ImscSpec> {
        DcdmimW::new(self, 2)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask"]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DsrmimW<'_, ImscSpec> {
        DsrmimW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive interrupt mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RximW<'_, ImscSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit interrupt mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TximW<'_, ImscSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RtimW<'_, ImscSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - Framing error interrupt mask"]
    #[inline(always)]
    pub fn ferim(&mut self) -> FerimW<'_, ImscSpec> {
        FerimW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt mask"]
    #[inline(always)]
    pub fn perim(&mut self) -> PerimW<'_, ImscSpec> {
        PerimW::new(self, 8)
    }
    #[doc = "Bit 9 - Break error interrupt mask"]
    #[inline(always)]
    pub fn berim(&mut self) -> BerimW<'_, ImscSpec> {
        BerimW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask"]
    #[inline(always)]
    pub fn oerim(&mut self) -> OerimW<'_, ImscSpec> {
        OerimW::new(self, 10)
    }
    #[doc = "Bit 11 - Transmit done interrupt mask"]
    #[inline(always)]
    pub fn tdim(&mut self) -> TdimW<'_, ImscSpec> {
        TdimW::new(self, 11)
    }
}
#[doc = "Interrupt Mask Set/Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscSpec;
impl crate::RegisterSpec for ImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for ImscSpec {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for ImscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for ImscSpec {}
