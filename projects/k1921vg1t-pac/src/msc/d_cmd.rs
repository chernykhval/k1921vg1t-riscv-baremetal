#[doc = "Register `D_CMD` reader"]
pub type R = crate::R<DCmdSpec>;
#[doc = "Register `D_CMD` writer"]
pub type W = crate::W<DCmdSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DCmdSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Downstream Command bits\n\nYou can [`read`](crate::Reg::read) this register and get [`d_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCmdSpec;
impl crate::RegisterSpec for DCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_cmd::R`](R) reader structure"]
impl crate::Readable for DCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`d_cmd::W`](W) writer structure"]
impl crate::Writable for DCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_CMD to value 0"]
impl crate::Resettable for DCmdSpec {}
