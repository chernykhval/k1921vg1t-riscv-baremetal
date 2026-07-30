#[doc = "Register `CLC` reader"]
pub type R = crate::R<ClcSpec>;
#[doc = "Register `CLC` writer"]
pub type W = crate::W<ClcSpec>;
#[doc = "Field `DISR` reader - OFF module CAN"]
pub type DisrR = crate::BitReader;
#[doc = "Field `DISR` writer - OFF module CAN"]
pub type DisrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISS` reader - State CAN"]
pub type DissR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - OFF module CAN"]
    #[inline(always)]
    pub fn disr(&self) -> DisrR {
        DisrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - State CAN"]
    #[inline(always)]
    pub fn diss(&self) -> DissR {
        DissR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OFF module CAN"]
    #[inline(always)]
    pub fn disr(&mut self) -> DisrW<'_, ClcSpec> {
        DisrW::new(self, 0)
    }
}
#[doc = "Frequency control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClcSpec;
impl crate::RegisterSpec for ClcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clc::R`](R) reader structure"]
impl crate::Readable for ClcSpec {}
#[doc = "`write(|w| ..)` method takes [`clc::W`](W) writer structure"]
impl crate::Writable for ClcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLC to value 0x03"]
impl crate::Resettable for ClcSpec {
    const RESET_VALUE: u32 = 0x03;
}
