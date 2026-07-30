#[doc = "Register `HPRIMOB` reader"]
pub type R = crate::R<HprimobSpec>;
#[doc = "Register `HPRIMOB` writer"]
pub type W = crate::W<HprimobSpec>;
#[doc = "Field `HPMOB` reader - Highest Priority MOb Number"]
pub type HpmobR = crate::FieldReader;
#[doc = "Field `HPMOB` writer - Highest Priority MOb Number"]
pub type HpmobW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 4:8 - Highest Priority MOb Number"]
    #[inline(always)]
    pub fn hpmob(&self) -> HpmobR {
        HpmobR::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:8 - Highest Priority MOb Number"]
    #[inline(always)]
    pub fn hpmob(&mut self) -> HpmobW<'_, HprimobSpec> {
        HpmobW::new(self, 4)
    }
}
#[doc = "CAN Highest Priority MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hprimob::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprimob::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HprimobSpec;
impl crate::RegisterSpec for HprimobSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprimob::R`](R) reader structure"]
impl crate::Readable for HprimobSpec {}
#[doc = "`write(|w| ..)` method takes [`hprimob::W`](W) writer structure"]
impl crate::Writable for HprimobSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPRIMOB to value 0x01f0"]
impl crate::Resettable for HprimobSpec {
    const RESET_VALUE: u32 = 0x01f0;
}
