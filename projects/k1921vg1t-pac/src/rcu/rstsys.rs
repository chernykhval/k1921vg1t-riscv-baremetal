#[doc = "Register `RSTSYS` writer"]
pub type W = crate::W<RstsysSpec>;
#[doc = "Field `RSTEN` writer - Reset enable"]
pub type RstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - KEY 'A55A' in hexadecimal for unblocking"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bit 0 - Reset enable"]
    #[inline(always)]
    pub fn rsten(&mut self) -> RstenW<'_, RstsysSpec> {
        RstenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - KEY 'A55A' in hexadecimal for unblocking"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, RstsysSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "System Reset Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsys::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstsysSpec;
impl crate::RegisterSpec for RstsysSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstsys::W`](W) writer structure"]
impl crate::Writable for RstsysSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTSYS to value 0"]
impl crate::Resettable for RstsysSpec {}
