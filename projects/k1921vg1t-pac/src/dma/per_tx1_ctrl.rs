#[doc = "Register `PER_TX1_CTRL` reader"]
pub type R = crate::R<PerTx1CtrlSpec>;
#[doc = "Register `PER_TX1_CTRL` writer"]
pub type W = crate::W<PerTx1CtrlSpec>;
#[doc = "Field `TXREQ` reader - "]
pub type TxreqR = crate::FieldReader<u32>;
#[doc = "Field `TXREQ` writer - "]
pub type TxreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn txreq(&self) -> TxreqR {
        TxreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn txreq(&mut self) -> TxreqW<'_, PerTx1CtrlSpec> {
        TxreqW::new(self, 0)
    }
}
#[doc = "Direct control of peripheral TX request (register 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_tx1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_tx1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerTx1CtrlSpec;
impl crate::RegisterSpec for PerTx1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per_tx1_ctrl::R`](R) reader structure"]
impl crate::Readable for PerTx1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`per_tx1_ctrl::W`](W) writer structure"]
impl crate::Writable for PerTx1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PER_TX1_CTRL to value 0"]
impl crate::Resettable for PerTx1CtrlSpec {}
