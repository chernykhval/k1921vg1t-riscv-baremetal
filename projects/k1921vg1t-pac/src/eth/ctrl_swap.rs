#[doc = "Register `CTRL_SWAP` reader"]
pub type R = crate::R<CtrlSwapSpec>;
#[doc = "Register `CTRL_SWAP` writer"]
pub type W = crate::W<CtrlSwapSpec>;
#[doc = "Field `SWAP` reader - Swaps control signals lines RX_DV and RXC"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Swaps control signals lines RX_DV and RXC"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Swaps control signals lines RX_DV and RXC"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Swaps control signals lines RX_DV and RXC"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, CtrlSwapSpec> {
        SwapW::new(self, 0)
    }
}
#[doc = "RGMII receiver control signal swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSwapSpec;
impl crate::RegisterSpec for CtrlSwapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_swap::R`](R) reader structure"]
impl crate::Readable for CtrlSwapSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_swap::W`](W) writer structure"]
impl crate::Writable for CtrlSwapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL_SWAP to value 0"]
impl crate::Resettable for CtrlSwapSpec {}
