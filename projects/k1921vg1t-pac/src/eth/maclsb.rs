#[doc = "Register `MACLSB` reader"]
pub type R = crate::R<MaclsbSpec>;
#[doc = "Register `MACLSB` writer"]
pub type W = crate::W<MaclsbSpec>;
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
    pub fn val(&mut self) -> ValW<'_, MaclsbSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address low bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`maclsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaclsbSpec;
impl crate::RegisterSpec for MaclsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maclsb::R`](R) reader structure"]
impl crate::Readable for MaclsbSpec {}
#[doc = "`write(|w| ..)` method takes [`maclsb::W`](W) writer structure"]
impl crate::Writable for MaclsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACLSB to value 0"]
impl crate::Resettable for MaclsbSpec {}
