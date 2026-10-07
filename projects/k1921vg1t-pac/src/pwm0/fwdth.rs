#[doc = "Register `FWDTH` reader"]
pub type R = crate::R<FwdthSpec>;
#[doc = "Register `FWDTH` writer"]
pub type W = crate::W<FwdthSpec>;
#[doc = "Field `VAL` reader - Pulse filter width selection"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Pulse filter width selection"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pulse filter width selection"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pulse filter width selection"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FwdthSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Filter Width select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fwdth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwdth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwdthSpec;
impl crate::RegisterSpec for FwdthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwdth::R`](R) reader structure"]
impl crate::Readable for FwdthSpec {}
#[doc = "`write(|w| ..)` method takes [`fwdth::W`](W) writer structure"]
impl crate::Writable for FwdthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FWDTH to value 0"]
impl crate::Resettable for FwdthSpec {}
