#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DmacrSpec>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DmacrSpec>;
#[doc = "Field `RXDMAE` reader - DMA enable bit at reception"]
pub type RxdmaeR = crate::BitReader;
#[doc = "Field `RXDMAE` writer - DMA enable bit at reception"]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - DMA enable bit transmission"]
pub type TxdmaeR = crate::BitReader;
#[doc = "Field `TXDMAE` writer - DMA enable bit transmission"]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARXSTOP` reader - Automatic clear DMA RX request generation when channel ends all transactions"]
pub type DmarxstopR = crate::BitReader;
#[doc = "Field `DMARXSTOP` writer - Automatic clear DMA RX request generation when channel ends all transactions"]
pub type DmarxstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATXSTOP` reader - Automatic clear DMA TX request generation when channel ends all transactions"]
pub type DmatxstopR = crate::BitReader;
#[doc = "Field `DMATXSTOP` writer - Automatic clear DMA TX request generation when channel ends all transactions"]
pub type DmatxstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA enable bit at reception"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA enable bit transmission"]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic clear DMA RX request generation when channel ends all transactions"]
    #[inline(always)]
    pub fn dmarxstop(&self) -> DmarxstopR {
        DmarxstopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Automatic clear DMA TX request generation when channel ends all transactions"]
    #[inline(always)]
    pub fn dmatxstop(&self) -> DmatxstopR {
        DmatxstopR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable bit at reception"]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RxdmaeW<'_, DmacrSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA enable bit transmission"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TxdmaeW<'_, DmacrSpec> {
        TxdmaeW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic clear DMA RX request generation when channel ends all transactions"]
    #[inline(always)]
    pub fn dmarxstop(&mut self) -> DmarxstopW<'_, DmacrSpec> {
        DmarxstopW::new(self, 2)
    }
    #[doc = "Bit 3 - Automatic clear DMA TX request generation when channel ends all transactions"]
    #[inline(always)]
    pub fn dmatxstop(&mut self) -> DmatxstopW<'_, DmacrSpec> {
        DmatxstopW::new(self, 3)
    }
}
#[doc = "Control register DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrSpec;
impl crate::RegisterSpec for DmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DmacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DmacrSpec {}
