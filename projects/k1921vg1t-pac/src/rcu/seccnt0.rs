#[doc = "Register `SECCNT0` reader"]
pub type R = crate::R<Seccnt0Spec>;
#[doc = "Register `SECCNT0` writer"]
pub type W = crate::W<Seccnt0Spec>;
#[doc = "Field `VAL0` reader - "]
pub type Val0R = crate::FieldReader<u16>;
#[doc = "Field `VAL1` reader - "]
pub type Val1R = crate::FieldReader<u16>;
#[doc = "Field `VAL1` writer - "]
pub type Val1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn val0(&self) -> Val0R {
        Val0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn val1(&self) -> Val1R {
        Val1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn val1(&mut self) -> Val1W<'_, Seccnt0Spec> {
        Val1W::new(self, 16)
    }
}
#[doc = "Security counter configuration 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seccnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seccnt0Spec;
impl crate::RegisterSpec for Seccnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccnt0::R`](R) reader structure"]
impl crate::Readable for Seccnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`seccnt0::W`](W) writer structure"]
impl crate::Writable for Seccnt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECCNT0 to value 0xffff_0000"]
impl crate::Resettable for Seccnt0Spec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
