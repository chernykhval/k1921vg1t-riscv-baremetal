#[doc = "Register `I2SSTATE` reader"]
pub type R = crate::R<I2sstateSpec>;
#[doc = "Field `TXFIFO_STATE` reader - Transmiter FIFO status"]
pub type TxfifoStateR = crate::BitReader;
#[doc = "Field `RXFIFO_STATE` reader - Receiver FIFO status"]
pub type RxfifoStateR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmiter FIFO status"]
    #[inline(always)]
    pub fn txfifo_state(&self) -> TxfifoStateR {
        TxfifoStateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver FIFO status"]
    #[inline(always)]
    pub fn rxfifo_state(&self) -> RxfifoStateR {
        RxfifoStateR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "I2S State register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sstate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sstateSpec;
impl crate::RegisterSpec for I2sstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sstate::R`](R) reader structure"]
impl crate::Readable for I2sstateSpec {}
#[doc = "`reset()` method sets I2SSTATE to value 0"]
impl crate::Resettable for I2sstateSpec {}
