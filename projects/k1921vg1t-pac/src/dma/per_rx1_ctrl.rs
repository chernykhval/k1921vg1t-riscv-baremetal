#[doc = "Register `PER_RX1_CTRL` reader"]
pub type R = crate::R<PerRx1CtrlSpec>;
#[doc = "Register `PER_RX1_CTRL` writer"]
pub type W = crate::W<PerRx1CtrlSpec>;
#[doc = "Field `RXREQ` reader - "]
pub type RxreqR = crate::FieldReader<u32>;
#[doc = "Field `RXREQ` writer - "]
pub type RxreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rxreq(&self) -> RxreqR {
        RxreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rxreq(&mut self) -> RxreqW<'_, PerRx1CtrlSpec> {
        RxreqW::new(self, 0)
    }
}
#[doc = "Direct control of peripheral RX request (register 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_rx1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_rx1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerRx1CtrlSpec;
impl crate::RegisterSpec for PerRx1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_rx1_ctrl::R`](R) reader structure"]
impl crate::Readable for PerRx1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`per_rx1_ctrl::W`](W) writer structure"]
impl crate::Writable for PerRx1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PER_RX1_CTRL to value 0"]
impl crate::Resettable for PerRx1CtrlSpec {}
