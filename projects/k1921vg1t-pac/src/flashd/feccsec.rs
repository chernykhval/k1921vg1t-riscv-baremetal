#[doc = "Register `FECCSEC` reader"]
pub type R = crate::R<FeccsecSpec>;
#[doc = "Register `FECCSEC` writer"]
pub type W = crate::W<FeccsecSpec>;
#[doc = "Field `VAL` reader - Single error times occurred"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Single error times occurred"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single error times occurred"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Single error times occurred"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FeccsecSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Flash ECC Single Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeccsecSpec;
impl crate::RegisterSpec for FeccsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feccsec::R`](R) reader structure"]
impl crate::Readable for FeccsecSpec {}
#[doc = "`write(|w| ..)` method takes [`feccsec::W`](W) writer structure"]
impl crate::Writable for FeccsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FECCSEC to value 0"]
impl crate::Resettable for FeccsecSpec {}
