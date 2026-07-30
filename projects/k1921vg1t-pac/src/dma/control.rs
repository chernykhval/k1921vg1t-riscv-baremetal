#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `JOINT_MODE` reader - Joint mode select"]
pub type JointModeR = crate::BitReader;
#[doc = "Field `JOINT_MODE` writer - Joint mode select"]
pub type JointModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Joint mode select"]
    #[inline(always)]
    pub fn joint_mode(&self) -> JointModeR {
        JointModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Joint mode select"]
    #[inline(always)]
    pub fn joint_mode(&mut self) -> JointModeW<'_, ControlSpec> {
        JointModeW::new(self, 0)
    }
}
#[doc = "DMA Core Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {}
