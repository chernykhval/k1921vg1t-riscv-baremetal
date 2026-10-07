#[doc = "Register `DATA_SWAP` reader"]
pub type R = crate::R<DataSwapSpec>;
#[doc = "Register `DATA_SWAP` writer"]
pub type W = crate::W<DataSwapSpec>;
#[doc = "Field `SWAP` reader - Swaps data between negative and positive clocked data input"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Swaps data between negative and positive clocked data input"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Swaps data between negative and positive clocked data input"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Swaps data between negative and positive clocked data input"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, DataSwapSpec> {
        SwapW::new(self, 0)
    }
}
#[doc = "RGMII receiver data swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSwapSpec;
impl crate::RegisterSpec for DataSwapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_swap::R`](R) reader structure"]
impl crate::Readable for DataSwapSpec {}
#[doc = "`write(|w| ..)` method takes [`data_swap::W`](W) writer structure"]
impl crate::Writable for DataSwapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_SWAP to value 0"]
impl crate::Resettable for DataSwapSpec {}
