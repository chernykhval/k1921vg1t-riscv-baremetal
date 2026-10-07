#[doc = "Register `LOAD` reader"]
pub type R = crate::R<LoadSpec>;
#[doc = "Register `LOAD` writer"]
pub type W = crate::W<LoadSpec>;
#[doc = "Field `VAL` reader - Begin value counter"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Begin value counter"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Begin value counter"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Begin value counter"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, LoadSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Watchdog Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`load::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadSpec;
impl crate::RegisterSpec for LoadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load::R`](R) reader structure"]
impl crate::Readable for LoadSpec {}
#[doc = "`write(|w| ..)` method takes [`load::W`](W) writer structure"]
impl crate::Writable for LoadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOAD to value 0xffff_ffff"]
impl crate::Resettable for LoadSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
