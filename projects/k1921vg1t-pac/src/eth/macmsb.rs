#[doc = "Register `MACMSB` reader"]
pub type R = crate::R<MacmsbSpec>;
#[doc = "Register `MACMSB` writer"]
pub type W = crate::W<MacmsbSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, MacmsbSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address high bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`macmsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacmsbSpec;
impl crate::RegisterSpec for MacmsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmsb::R`](R) reader structure"]
impl crate::Readable for MacmsbSpec {}
#[doc = "`write(|w| ..)` method takes [`macmsb::W`](W) writer structure"]
impl crate::Writable for MacmsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACMSB to value 0"]
impl crate::Resettable for MacmsbSpec {}
