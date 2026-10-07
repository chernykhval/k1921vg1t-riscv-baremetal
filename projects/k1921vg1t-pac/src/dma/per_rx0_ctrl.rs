#[doc = "Register `PER_RX0_CTRL` reader"]
pub type R = crate::R<PerRx0CtrlSpec>;
#[doc = "Register `PER_RX0_CTRL` writer"]
pub type W = crate::W<PerRx0CtrlSpec>;
#[doc = "Field `RXREQ` reader - "]
pub type RxreqR = crate::FieldReader<u32>;
#[doc = "Field `RXREQ` writer - "]
pub type RxreqW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn rxreq(&self) -> RxreqR {
        RxreqR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn rxreq(&mut self) -> RxreqW<'_, PerRx0CtrlSpec> {
        RxreqW::new(self, 1)
    }
}
#[doc = "Direct control of peripheral RX request (register 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_rx0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_rx0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerRx0CtrlSpec;
impl crate::RegisterSpec for PerRx0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_rx0_ctrl::R`](R) reader structure"]
impl crate::Readable for PerRx0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`per_rx0_ctrl::W`](W) writer structure"]
impl crate::Writable for PerRx0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PER_RX0_CTRL to value 0"]
impl crate::Resettable for PerRx0CtrlSpec {}
