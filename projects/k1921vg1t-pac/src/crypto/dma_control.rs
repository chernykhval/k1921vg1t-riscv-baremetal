#[doc = "Register `DMA_CONTROL` reader"]
pub type R = crate::R<DmaControlSpec>;
#[doc = "Register `DMA_CONTROL` writer"]
pub type W = crate::W<DmaControlSpec>;
#[doc = "Field `START` writer - "]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTES_SWAP` reader - "]
pub type BytesSwapR = crate::BitReader;
#[doc = "Field `BYTES_SWAP` writer - "]
pub type BytesSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORDS_SWAP` reader - "]
pub type WordsSwapR = crate::BitReader;
#[doc = "Field `WORDS_SWAP` writer - "]
pub type WordsSwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bytes_swap(&self) -> BytesSwapR {
        BytesSwapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn words_swap(&self) -> WordsSwapR {
        WordsSwapR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, DmaControlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bytes_swap(&mut self) -> BytesSwapW<'_, DmaControlSpec> {
        BytesSwapW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn words_swap(&mut self) -> WordsSwapW<'_, DmaControlSpec> {
        WordsSwapW::new(self, 2)
    }
}
#[doc = "DMA operations control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaControlSpec;
impl crate::RegisterSpec for DmaControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_control::R`](R) reader structure"]
impl crate::Readable for DmaControlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_control::W`](W) writer structure"]
impl crate::Writable for DmaControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CONTROL to value 0"]
impl crate::Resettable for DmaControlSpec {}
