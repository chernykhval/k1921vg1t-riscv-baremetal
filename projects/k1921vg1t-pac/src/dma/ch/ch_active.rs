#[doc = "Register `CH_ACTIVE` reader"]
pub type R = crate::R<ChActiveSpec>;
#[doc = "Register `CH_ACTIVE` writer"]
pub type W = crate::W<ChActiveSpec>;
#[doc = "Field `CH_ACTIVE` reader - Channel is currently active"]
pub type ChActiveR = crate::BitReader;
#[doc = "Field `CH_ACTIVE` writer - Channel is currently active"]
pub type ChActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel is currently active"]
    #[inline(always)]
    pub fn ch_active(&self) -> ChActiveR {
        ChActiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel is currently active"]
    #[inline(always)]
    pub fn ch_active(&mut self) -> ChActiveW<'_, ChActiveSpec> {
        ChActiveW::new(self, 0)
    }
}
#[doc = "Channel active register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_active::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_active::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChActiveSpec;
impl crate::RegisterSpec for ChActiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_active::R`](R) reader structure"]
impl crate::Readable for ChActiveSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_active::W`](W) writer structure"]
impl crate::Writable for ChActiveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_ACTIVE to value 0x01"]
impl crate::Resettable for ChActiveSpec {
    const RESET_VALUE: u32 = 0x01;
}
