#[doc = "Register `POS_DELAY` reader"]
pub type R = crate::R<PosDelaySpec>;
#[doc = "Register `POS_DELAY` writer"]
pub type W = crate::W<PosDelaySpec>;
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
    pub fn delay(&mut self) -> DelayW<'_, PosDelaySpec> {
        DelayW::new(self, 0)
    }
}
#[doc = "RGMII negative clock data receiver delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`pos_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pos_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PosDelaySpec;
impl crate::RegisterSpec for PosDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pos_delay::R`](R) reader structure"]
impl crate::Readable for PosDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`pos_delay::W`](W) writer structure"]
impl crate::Writable for PosDelaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POS_DELAY to value 0"]
impl crate::Resettable for PosDelaySpec {}
