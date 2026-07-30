#[doc = "Register `I2SSTATECLR` writer"]
pub type W = crate::W<I2sstateclrSpec>;
#[doc = "Field `TXFIFO_STCLR` writer - Transmiter FIFO status clear"]
pub type TxfifoStclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_STCLR` writer - Receiver FIFO status clear"]
pub type RxfifoStclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmiter FIFO status clear"]
    #[inline(always)]
    pub fn txfifo_stclr(&mut self) -> TxfifoStclrW<'_, I2sstateclrSpec> {
        TxfifoStclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver FIFO status clear"]
    #[inline(always)]
    pub fn rxfifo_stclr(&mut self) -> RxfifoStclrW<'_, I2sstateclrSpec> {
        RxfifoStclrW::new(self, 1)
    }
}
#[doc = "I2S State Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sstateclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sstateclrSpec;
impl crate::RegisterSpec for I2sstateclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2sstateclr::W`](W) writer structure"]
impl crate::Writable for I2sstateclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SSTATECLR to value 0"]
impl crate::Resettable for I2sstateclrSpec {}
