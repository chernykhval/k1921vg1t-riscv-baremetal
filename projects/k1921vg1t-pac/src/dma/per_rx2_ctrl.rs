#[doc = "Register `PER_RX2_CTRL` reader"]
pub type R = crate::R<PerRx2CtrlSpec>;
#[doc = "Register `PER_RX2_CTRL` writer"]
pub type W = crate::W<PerRx2CtrlSpec>;
#[doc = "Field `RXREQ` reader - "]
pub type RxreqR = crate::FieldReader;
#[doc = "Field `RXREQ` writer - "]
pub type RxreqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rxreq(&self) -> RxreqR {
        RxreqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rxreq(&mut self) -> RxreqW<'_, PerRx2CtrlSpec> {
        RxreqW::new(self, 0)
    }
}
#[doc = "Direct control of peripheral RX request (register 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_rx2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_rx2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerRx2CtrlSpec;
impl crate::RegisterSpec for PerRx2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_rx2_ctrl::R`](R) reader structure"]
impl crate::Readable for PerRx2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`per_rx2_ctrl::W`](W) writer structure"]
impl crate::Writable for PerRx2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PER_RX2_CTRL to value 0"]
impl crate::Resettable for PerRx2CtrlSpec {}
