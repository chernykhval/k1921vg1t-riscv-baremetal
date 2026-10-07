#[doc = "Register `BIT_SWAP` reader"]
pub type R = crate::R<BitSwapSpec>;
#[doc = "Register `BIT_SWAP` writer"]
pub type W = crate::W<BitSwapSpec>;
#[doc = "Field `SWAP` reader - Change data order on RxD RGMII to GMII output"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Change data order on RxD RGMII to GMII output"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Change data order on RxD RGMII to GMII output"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Change data order on RxD RGMII to GMII output"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, BitSwapSpec> {
        SwapW::new(self, 0)
    }
}
#[doc = "RGMII receiver sample order swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`bit_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BitSwapSpec;
impl crate::RegisterSpec for BitSwapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bit_swap::R`](R) reader structure"]
impl crate::Readable for BitSwapSpec {}
#[doc = "`write(|w| ..)` method takes [`bit_swap::W`](W) writer structure"]
impl crate::Writable for BitSwapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIT_SWAP to value 0"]
impl crate::Resettable for BitSwapSpec {}
