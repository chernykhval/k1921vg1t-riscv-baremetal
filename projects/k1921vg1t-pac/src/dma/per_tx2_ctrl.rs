#[doc = "Register `PER_TX2_CTRL` reader"]
pub type R = crate::R<PerTx2CtrlSpec>;
#[doc = "Register `PER_TX2_CTRL` writer"]
pub type W = crate::W<PerTx2CtrlSpec>;
#[doc = "Field `TXREQ` reader - "]
pub type TxreqR = crate::FieldReader;
#[doc = "Field `TXREQ` writer - "]
pub type TxreqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn txreq(&self) -> TxreqR {
        TxreqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn txreq(&mut self) -> TxreqW<'_, PerTx2CtrlSpec> {
        TxreqW::new(self, 0)
    }
}
#[doc = "Direct control of peripheral TX request (register 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_tx2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_tx2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerTx2CtrlSpec;
impl crate::RegisterSpec for PerTx2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_tx2_ctrl::R`](R) reader structure"]
impl crate::Readable for PerTx2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`per_tx2_ctrl::W`](W) writer structure"]
impl crate::Writable for PerTx2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PER_TX2_CTRL to value 0"]
impl crate::Resettable for PerTx2CtrlSpec {}
