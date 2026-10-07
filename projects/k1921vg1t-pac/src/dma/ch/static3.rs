#[doc = "Register `STATIC3` reader"]
pub type R = crate::R<Static3Spec>;
#[doc = "Register `STATIC3` writer"]
pub type W = crate::W<Static3Spec>;
#[doc = "Field `RD_WAIT_LIM` reader - Minimum number of cycles to wait after a channel is released and before its starts read command"]
pub type RdWaitLimR = crate::FieldReader<u16>;
#[doc = "Field `RD_WAIT_LIM` writer - Minimum number of cycles to wait after a channel is released and before its starts read command"]
pub type RdWaitLimW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WR_WAIT_LIM` reader - Minimum number of cycles to wait after a channel is released and before its starts write command"]
pub type WrWaitLimR = crate::FieldReader<u16>;
#[doc = "Field `WR_WAIT_LIM` writer - Minimum number of cycles to wait after a channel is released and before its starts write command"]
pub type WrWaitLimW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Minimum number of cycles to wait after a channel is released and before its starts read command"]
    #[inline(always)]
    pub fn rd_wait_lim(&self) -> RdWaitLimR {
        RdWaitLimR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Minimum number of cycles to wait after a channel is released and before its starts write command"]
    #[inline(always)]
    pub fn wr_wait_lim(&self) -> WrWaitLimR {
        WrWaitLimR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Minimum number of cycles to wait after a channel is released and before its starts read command"]
    #[inline(always)]
    pub fn rd_wait_lim(&mut self) -> RdWaitLimW<'_, Static3Spec> {
        RdWaitLimW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Minimum number of cycles to wait after a channel is released and before its starts write command"]
    #[inline(always)]
    pub fn wr_wait_lim(&mut self) -> WrWaitLimW<'_, Static3Spec> {
        WrWaitLimW::new(self, 16)
    }
}
#[doc = "Static register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`static3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Static3Spec;
impl crate::RegisterSpec for Static3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`static3::R`](R) reader structure"]
impl crate::Readable for Static3Spec {}
#[doc = "`write(|w| ..)` method takes [`static3::W`](W) writer structure"]
impl crate::Writable for Static3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATIC3 to value 0"]
impl crate::Resettable for Static3Spec {}
