#[doc = "Register `IRQEN` reader"]
pub type R = crate::R<IrqenSpec>;
#[doc = "Register `IRQEN` writer"]
pub type W = crate::W<IrqenSpec>;
#[doc = "Field `SEUEN` reader - Enable interrupt when single error detected"]
pub type SeuenR = crate::BitReader;
#[doc = "Field `SEUEN` writer - Enable interrupt when single error detected"]
pub type SeuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEUEN` reader - Enable interrupt when double errors detected"]
pub type DeuenR = crate::BitReader;
#[doc = "Field `DEUEN` writer - Enable interrupt when double errors detected"]
pub type DeuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable interrupt when single error detected"]
    #[inline(always)]
    pub fn seuen(&self) -> SeuenR {
        SeuenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt when double errors detected"]
    #[inline(always)]
    pub fn deuen(&self) -> DeuenR {
        DeuenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt when single error detected"]
    #[inline(always)]
    pub fn seuen(&mut self) -> SeuenW<'_, IrqenSpec> {
        SeuenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable interrupt when double errors detected"]
    #[inline(always)]
    pub fn deuen(&mut self) -> DeuenW<'_, IrqenSpec> {
        DeuenW::new(self, 1)
    }
}
#[doc = "SRAM ECC Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqenSpec;
impl crate::RegisterSpec for IrqenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqen::R`](R) reader structure"]
impl crate::Readable for IrqenSpec {}
#[doc = "`write(|w| ..)` method takes [`irqen::W`](W) writer structure"]
impl crate::Writable for IrqenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQEN to value 0"]
impl crate::Resettable for IrqenSpec {}
