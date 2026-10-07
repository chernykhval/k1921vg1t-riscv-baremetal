#[doc = "Register `CH_START` writer"]
pub type W = crate::W<ChStartSpec>;
#[doc = "Field `CH_START` writer - Start channel"]
pub type ChStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start channel"]
    #[inline(always)]
    pub fn ch_start(&mut self) -> ChStartW<'_, ChStartSpec> {
        ChStartW::new(self, 0)
    }
}
#[doc = "Channel start register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChStartSpec;
impl crate::RegisterSpec for ChStartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_start::W`](W) writer structure"]
impl crate::Writable for ChStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_START to value 0"]
impl crate::Resettable for ChStartSpec {}
