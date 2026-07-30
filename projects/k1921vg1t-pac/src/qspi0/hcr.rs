#[doc = "Register `HCR` reader"]
pub type R = crate::R<HcrSpec>;
#[doc = "Register `HCR` writer"]
pub type W = crate::W<HcrSpec>;
#[doc = "Field `DEN` reader - Device enable"]
pub type DenR = crate::BitReader;
#[doc = "Field `DEN` writer - Device enable"]
pub type DenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRS` reader - Device reset ( SW reset )"]
pub type DrsR = crate::BitReader;
#[doc = "Field `DRS` writer - Device reset ( SW reset )"]
pub type DrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFCLR` reader - Tx FIFO clear"]
pub type TxfclrR = crate::BitReader;
#[doc = "Field `TXFCLR` writer - Tx FIFO clear"]
pub type TxfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFCLR` reader - Rx FIFO clear"]
pub type RxfclrR = crate::BitReader;
#[doc = "Field `RXFCLR` writer - Rx FIFO clear"]
pub type RxfclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Device enable"]
    #[inline(always)]
    pub fn den(&self) -> DenR {
        DenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device reset ( SW reset )"]
    #[inline(always)]
    pub fn drs(&self) -> DrsR {
        DrsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx FIFO clear"]
    #[inline(always)]
    pub fn txfclr(&self) -> TxfclrR {
        TxfclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO clear"]
    #[inline(always)]
    pub fn rxfclr(&self) -> RxfclrR {
        RxfclrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device enable"]
    #[inline(always)]
    pub fn den(&mut self) -> DenW<'_, HcrSpec> {
        DenW::new(self, 0)
    }
    #[doc = "Bit 1 - Device reset ( SW reset )"]
    #[inline(always)]
    pub fn drs(&mut self) -> DrsW<'_, HcrSpec> {
        DrsW::new(self, 1)
    }
    #[doc = "Bit 2 - Tx FIFO clear"]
    #[inline(always)]
    pub fn txfclr(&mut self) -> TxfclrW<'_, HcrSpec> {
        TxfclrW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO clear"]
    #[inline(always)]
    pub fn rxfclr(&mut self) -> RxfclrW<'_, HcrSpec> {
        RxfclrW::new(self, 3)
    }
}
#[doc = "Host Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcrSpec;
impl crate::RegisterSpec for HcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcr::R`](R) reader structure"]
impl crate::Readable for HcrSpec {}
#[doc = "`write(|w| ..)` method takes [`hcr::W`](W) writer structure"]
impl crate::Writable for HcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCR to value 0"]
impl crate::Resettable for HcrSpec {}
