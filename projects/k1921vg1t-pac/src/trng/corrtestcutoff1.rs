#[doc = "Register `CORRTESTCUTOFF1` reader"]
pub type R = crate::R<Corrtestcutoff1Spec>;
#[doc = "Register `CORRTESTCUTOFF1` writer"]
pub type W = crate::W<Corrtestcutoff1Spec>;
#[doc = "Field `TWOCUTOFF` reader - Correlation test cut-off value for time shift/delay of +/-2 samples"]
pub type TwocutoffR = crate::FieldReader;
#[doc = "Field `TWOCUTOFF` writer - Correlation test cut-off value for time shift/delay of +/-2 samples"]
pub type TwocutoffW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `THREECUTOFF` reader - Correlation test cut-off value for time shift/delay of +/-3 samples"]
pub type ThreecutoffR = crate::FieldReader;
#[doc = "Field `THREECUTOFF` writer - Correlation test cut-off value for time shift/delay of +/-3 samples"]
pub type ThreecutoffW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Correlation test cut-off value for time shift/delay of +/-2 samples"]
    #[inline(always)]
    pub fn twocutoff(&self) -> TwocutoffR {
        TwocutoffR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Correlation test cut-off value for time shift/delay of +/-3 samples"]
    #[inline(always)]
    pub fn threecutoff(&self) -> ThreecutoffR {
        ThreecutoffR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Correlation test cut-off value for time shift/delay of +/-2 samples"]
    #[inline(always)]
    pub fn twocutoff(&mut self) -> TwocutoffW<'_, Corrtestcutoff1Spec> {
        TwocutoffW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Correlation test cut-off value for time shift/delay of +/-3 samples"]
    #[inline(always)]
    pub fn threecutoff(&mut self) -> ThreecutoffW<'_, Corrtestcutoff1Spec> {
        ThreecutoffW::new(self, 16)
    }
}
#[doc = "CorrTestCutoff register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`corrtestcutoff1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corrtestcutoff1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Corrtestcutoff1Spec;
impl crate::RegisterSpec for Corrtestcutoff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`corrtestcutoff1::R`](R) reader structure"]
impl crate::Readable for Corrtestcutoff1Spec {}
#[doc = "`write(|w| ..)` method takes [`corrtestcutoff1::W`](W) writer structure"]
impl crate::Writable for Corrtestcutoff1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORRTESTCUTOFF1 to value 0x007f_007f"]
impl crate::Resettable for Corrtestcutoff1Spec {
    const RESET_VALUE: u32 = 0x007f_007f;
}
