#[doc = "Register `CORRTESTCUTOFF0` reader"]
pub type R = crate::R<Corrtestcutoff0Spec>;
#[doc = "Register `CORRTESTCUTOFF0` writer"]
pub type W = crate::W<Corrtestcutoff0Spec>;
#[doc = "Field `ZEROCUTOFF` reader - Correlation test cut-off value for time shift/delay of 0 samples"]
pub type ZerocutoffR = crate::FieldReader;
#[doc = "Field `ZEROCUTOFF` writer - Correlation test cut-off value for time shift/delay of 0 samples"]
pub type ZerocutoffW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ONECUTOFF` reader - Correlation test cut-off value for time shift/delay of +/-1 sample"]
pub type OnecutoffR = crate::FieldReader;
#[doc = "Field `ONECUTOFF` writer - Correlation test cut-off value for time shift/delay of +/-1 sample"]
pub type OnecutoffW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Correlation test cut-off value for time shift/delay of 0 samples"]
    #[inline(always)]
    pub fn zerocutoff(&self) -> ZerocutoffR {
        ZerocutoffR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Correlation test cut-off value for time shift/delay of +/-1 sample"]
    #[inline(always)]
    pub fn onecutoff(&self) -> OnecutoffR {
        OnecutoffR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Correlation test cut-off value for time shift/delay of 0 samples"]
    #[inline(always)]
    pub fn zerocutoff(&mut self) -> ZerocutoffW<'_, Corrtestcutoff0Spec> {
        ZerocutoffW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Correlation test cut-off value for time shift/delay of +/-1 sample"]
    #[inline(always)]
    pub fn onecutoff(&mut self) -> OnecutoffW<'_, Corrtestcutoff0Spec> {
        OnecutoffW::new(self, 16)
    }
}
#[doc = "CorrTestCutoff register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`corrtestcutoff0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corrtestcutoff0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Corrtestcutoff0Spec;
impl crate::RegisterSpec for Corrtestcutoff0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`corrtestcutoff0::R`](R) reader structure"]
impl crate::Readable for Corrtestcutoff0Spec {}
#[doc = "`write(|w| ..)` method takes [`corrtestcutoff0::W`](W) writer structure"]
impl crate::Writable for Corrtestcutoff0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORRTESTCUTOFF0 to value 0x007f_007f"]
impl crate::Resettable for Corrtestcutoff0Spec {
    const RESET_VALUE: u32 = 0x007f_007f;
}
