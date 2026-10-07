#[doc = "Register `FIFOSTAT` reader"]
pub type R = crate::R<FifostatSpec>;
#[doc = "Field `RD_GAP` reader - Remaining space for channels FIFO for read data"]
pub type RdGapR = crate::FieldReader;
#[doc = "Field `WR_FULLNESS` reader - Occupied space in channels FIFO by write data"]
pub type WrFullnessR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Remaining space for channels FIFO for read data"]
    #[inline(always)]
    pub fn rd_gap(&self) -> RdGapR {
        RdGapR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Occupied space in channels FIFO by write data"]
    #[inline(always)]
    pub fn wr_fullness(&self) -> WrFullnessR {
        WrFullnessR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "FIFO fullness status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifostat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifostatSpec;
impl crate::RegisterSpec for FifostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifostat::R`](R) reader structure"]
impl crate::Readable for FifostatSpec {}
#[doc = "`reset()` method sets FIFOSTAT to value 0x20"]
impl crate::Resettable for FifostatSpec {
    const RESET_VALUE: u32 = 0x20;
}
