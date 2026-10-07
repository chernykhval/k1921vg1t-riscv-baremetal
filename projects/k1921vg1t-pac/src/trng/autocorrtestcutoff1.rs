#[doc = "Register `AUTOCORRTESTCUTOFF1` reader"]
pub type R = crate::R<Autocorrtestcutoff1Spec>;
#[doc = "Register `AUTOCORRTESTCUTOFF1` writer"]
pub type W = crate::W<Autocorrtestcutoff1Spec>;
#[doc = "Field `TWOCUTOFF` reader - Auto-correlation test cut-off value for delay +2 samples"]
pub type TwocutoffR = crate::FieldReader;
#[doc = "Field `TWOCUTOFF` writer - Auto-correlation test cut-off value for delay +2 samples"]
pub type TwocutoffW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `THREECUTOFF` reader - Auto-correlation test cut-off value for delay +3 samples"]
pub type ThreecutoffR = crate::FieldReader;
#[doc = "Field `THREECUTOFF` writer - Auto-correlation test cut-off value for delay +3 samples"]
pub type ThreecutoffW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Auto-correlation test cut-off value for delay +2 samples"]
    #[inline(always)]
    pub fn twocutoff(&self) -> TwocutoffR {
        TwocutoffR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Auto-correlation test cut-off value for delay +3 samples"]
    #[inline(always)]
    pub fn threecutoff(&self) -> ThreecutoffR {
        ThreecutoffR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Auto-correlation test cut-off value for delay +2 samples"]
    #[inline(always)]
    pub fn twocutoff(&mut self) -> TwocutoffW<'_, Autocorrtestcutoff1Spec> {
        TwocutoffW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Auto-correlation test cut-off value for delay +3 samples"]
    #[inline(always)]
    pub fn threecutoff(&mut self) -> ThreecutoffW<'_, Autocorrtestcutoff1Spec> {
        ThreecutoffW::new(self, 16)
    }
}
#[doc = "AutoCorrTestCutoff register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorrtestcutoff1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorrtestcutoff1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Autocorrtestcutoff1Spec;
impl crate::RegisterSpec for Autocorrtestcutoff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocorrtestcutoff1::R`](R) reader structure"]
impl crate::Readable for Autocorrtestcutoff1Spec {}
#[doc = "`write(|w| ..)` method takes [`autocorrtestcutoff1::W`](W) writer structure"]
impl crate::Writable for Autocorrtestcutoff1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTOCORRTESTCUTOFF1 to value 0x007f_007f"]
impl crate::Resettable for Autocorrtestcutoff1Spec {
    const RESET_VALUE: u32 = 0x007f_007f;
}
