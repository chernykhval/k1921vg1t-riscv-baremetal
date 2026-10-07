#[doc = "Register `SECCNT1` reader"]
pub type R = crate::R<Seccnt1Spec>;
#[doc = "Register `SECCNT1` writer"]
pub type W = crate::W<Seccnt1Spec>;
#[doc = "Field `VAL2` reader - "]
pub type Val2R = crate::FieldReader<u16>;
#[doc = "Field `VAL2` writer - "]
pub type Val2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VAL3` reader - "]
pub type Val3R = crate::FieldReader<u16>;
#[doc = "Field `VAL3` writer - "]
pub type Val3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn val2(&self) -> Val2R {
        Val2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn val3(&self) -> Val3R {
        Val3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn val2(&mut self) -> Val2W<'_, Seccnt1Spec> {
        Val2W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn val3(&mut self) -> Val3W<'_, Seccnt1Spec> {
        Val3W::new(self, 16)
    }
}
#[doc = "Security counter configuration 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seccnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seccnt1Spec;
impl crate::RegisterSpec for Seccnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccnt1::R`](R) reader structure"]
impl crate::Readable for Seccnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`seccnt1::W`](W) writer structure"]
impl crate::Writable for Seccnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECCNT1 to value 0xffff_ffff"]
impl crate::Resettable for Seccnt1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
