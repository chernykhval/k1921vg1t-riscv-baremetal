#[doc = "Register `SEU_ADDR` reader"]
pub type R = crate::R<SeuAddrSpec>;
#[doc = "Register `SEU_ADDR` writer"]
pub type W = crate::W<SeuAddrSpec>;
#[doc = "Field `VAL` reader - Last latched address when single error occurred"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Last latched address when single error occurred"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last latched address when single error occurred"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last latched address when single error occurred"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, SeuAddrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "SRAM Single Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`seu_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seu_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeuAddrSpec;
impl crate::RegisterSpec for SeuAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seu_addr::R`](R) reader structure"]
impl crate::Readable for SeuAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`seu_addr::W`](W) writer structure"]
impl crate::Writable for SeuAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEU_ADDR to value 0"]
impl crate::Resettable for SeuAddrSpec {}
