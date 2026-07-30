#[doc = "Register `DEU_ADDR` reader"]
pub type R = crate::R<DeuAddrSpec>;
#[doc = "Register `DEU_ADDR` writer"]
pub type W = crate::W<DeuAddrSpec>;
#[doc = "Field `VAL` reader - Last latched address when double error occurred"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Last latched address when double error occurred"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last latched address when double error occurred"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last latched address when double error occurred"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DeuAddrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "SRAM Double Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`deu_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deu_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeuAddrSpec;
impl crate::RegisterSpec for DeuAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deu_addr::R`](R) reader structure"]
impl crate::Readable for DeuAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`deu_addr::W`](W) writer structure"]
impl crate::Writable for DeuAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEU_ADDR to value 0"]
impl crate::Resettable for DeuAddrSpec {}
