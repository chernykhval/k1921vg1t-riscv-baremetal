#[doc = "Register `NEG_DELAY` reader"]
pub type R = crate::R<NegDelaySpec>;
#[doc = "Register `NEG_DELAY` writer"]
pub type W = crate::W<NegDelaySpec>;
#[doc = "Field `DELAY` reader - Number of RX clock cycles to delay input data"]
pub type DelayR = crate::FieldReader;
#[doc = "Field `DELAY` writer - Number of RX clock cycles to delay input data"]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Number of RX clock cycles to delay input data"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of RX clock cycles to delay input data"]
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<'_, NegDelaySpec> {
        DelayW::new(self, 0)
    }
}
#[doc = "RGMII positive clock data receiver delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`neg_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`neg_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NegDelaySpec;
impl crate::RegisterSpec for NegDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`neg_delay::R`](R) reader structure"]
impl crate::Readable for NegDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`neg_delay::W`](W) writer structure"]
impl crate::Writable for NegDelaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NEG_DELAY to value 0"]
impl crate::Resettable for NegDelaySpec {}
