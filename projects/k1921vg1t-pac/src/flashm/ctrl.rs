#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FBEN` reader - Fetch buffer Enable"]
pub type FbenR = crate::BitReader;
#[doc = "Field `FBEN` writer - Fetch buffer Enable"]
pub type FbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQEN` reader - Interrupt enable bit"]
pub type IrqenR = crate::BitReader;
#[doc = "Field `IRQEN` writer - Interrupt enable bit"]
pub type IrqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBFLUSH` writer - Flush internal buffer"]
pub type FbflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAT` reader - Flash latency"]
pub type LatR = crate::FieldReader;
#[doc = "Field `LAT` writer - Flash latency"]
pub type LatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Fetch buffer Enable"]
    #[inline(always)]
    pub fn fben(&self) -> FbenR {
        FbenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable bit"]
    #[inline(always)]
    pub fn irqen(&self) -> IrqenR {
        IrqenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Flash latency"]
    #[inline(always)]
    pub fn lat(&self) -> LatR {
        LatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fetch buffer Enable"]
    #[inline(always)]
    pub fn fben(&mut self) -> FbenW<'_, CtrlSpec> {
        FbenW::new(self, 0)
    }
    #[doc = "Bit 4 - Interrupt enable bit"]
    #[inline(always)]
    pub fn irqen(&mut self) -> IrqenW<'_, CtrlSpec> {
        IrqenW::new(self, 4)
    }
    #[doc = "Bit 8 - Flush internal buffer"]
    #[inline(always)]
    pub fn fbflush(&mut self) -> FbflushW<'_, CtrlSpec> {
        FbflushW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Flash latency"]
    #[inline(always)]
    pub fn lat(&mut self) -> LatW<'_, CtrlSpec> {
        LatW::new(self, 16)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
