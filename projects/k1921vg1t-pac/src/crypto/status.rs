#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `READY` reader - "]
pub type ReadyR = crate::BitReader;
#[doc = "Field `KEYS_READY` reader - "]
pub type KeysReadyR = crate::BitReader;
#[doc = "Field `OUT_VALID` reader - "]
pub type OutValidR = crate::BitReader;
#[doc = "Field `IRQ_PENDING` reader - "]
pub type IrqPendingR = crate::BitReader;
#[doc = "Field `KEYS_STORED` reader - "]
pub type KeysStoredR = crate::FieldReader;
#[doc = "Field `DMA_ACTIVE` reader - "]
pub type DmaActiveR = crate::BitReader;
#[doc = "Field `AHB_ERROR` reader - "]
pub type AhbErrorR = crate::FieldReader;
#[doc = "Field `BAD_DESCR` reader - "]
pub type BadDescrR = crate::FieldReader;
#[doc = "Field `DMA_COUNTER` reader - "]
pub type DmaCounterR = crate::FieldReader<u16>;
#[doc = "Field `DMA_LAST` reader - "]
pub type DmaLastR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn keys_ready(&self) -> KeysReadyR {
        KeysReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn out_valid(&self) -> OutValidR {
        OutValidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn irq_pending(&self) -> IrqPendingR {
        IrqPendingR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn keys_stored(&self) -> KeysStoredR {
        KeysStoredR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_active(&self) -> DmaActiveR {
        DmaActiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn ahb_error(&self) -> AhbErrorR {
        AhbErrorR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn bad_descr(&self) -> BadDescrR {
        BadDescrR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn dma_counter(&self) -> DmaCounterR {
        DmaCounterR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dma_last(&self) -> DmaLastR {
        DmaLastR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Operation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
