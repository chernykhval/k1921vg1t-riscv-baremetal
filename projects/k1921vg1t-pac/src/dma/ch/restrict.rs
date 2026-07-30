#[doc = "Register `RESTRICT` reader"]
pub type R = crate::R<RestrictSpec>;
#[doc = "Field `RD_ALLOWFIFO` reader - Read start address does not restrict burst size"]
pub type RdAllowfifoR = crate::BitReader;
#[doc = "Field `WR_ALLOWFIFO` reader - Write start address does not restrict burst size"]
pub type WrAllowfifoR = crate::BitReader;
#[doc = "Field `ALLOW_FIFO` reader - Burst size can equal data buffer size"]
pub type AllowFifoR = crate::BitReader;
#[doc = "Field `ALLOW_FBURST` reader - Maximum bursts of 16 strobes can be used"]
pub type AllowFburstR = crate::BitReader;
#[doc = "Field `ALLOW_JBURST` reader - Joint bursts are currently active"]
pub type AllowJburstR = crate::BitReader;
#[doc = "Field `RD_OUTSTAND` reader - Read outstanding is currently active"]
pub type RdOutstandR = crate::BitReader;
#[doc = "Field `WR_OUTSTAND` reader - Write outstanding is currently active"]
pub type WrOutstandR = crate::BitReader;
#[doc = "Field `NONALIGN` reader - Block configuration is not aligned"]
pub type NonalignR = crate::BitReader;
#[doc = "Field `SIMPLE_STAT` reader - Configuration is aligned and perepherals are not used"]
pub type SimpleStatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read start address does not restrict burst size"]
    #[inline(always)]
    pub fn rd_allowfifo(&self) -> RdAllowfifoR {
        RdAllowfifoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write start address does not restrict burst size"]
    #[inline(always)]
    pub fn wr_allowfifo(&self) -> WrAllowfifoR {
        WrAllowfifoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Burst size can equal data buffer size"]
    #[inline(always)]
    pub fn allow_fifo(&self) -> AllowFifoR {
        AllowFifoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Maximum bursts of 16 strobes can be used"]
    #[inline(always)]
    pub fn allow_fburst(&self) -> AllowFburstR {
        AllowFburstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Joint bursts are currently active"]
    #[inline(always)]
    pub fn allow_jburst(&self) -> AllowJburstR {
        AllowJburstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read outstanding is currently active"]
    #[inline(always)]
    pub fn rd_outstand(&self) -> RdOutstandR {
        RdOutstandR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write outstanding is currently active"]
    #[inline(always)]
    pub fn wr_outstand(&self) -> WrOutstandR {
        WrOutstandR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Block configuration is not aligned"]
    #[inline(always)]
    pub fn nonalign(&self) -> NonalignR {
        NonalignR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configuration is aligned and perepherals are not used"]
    #[inline(always)]
    pub fn simple_stat(&self) -> SimpleStatR {
        SimpleStatR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Channels restrictions status register\n\nYou can [`read`](crate::Reg::read) this register and get [`restrict::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RestrictSpec;
impl crate::RegisterSpec for RestrictSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`restrict::R`](R) reader structure"]
impl crate::Readable for RestrictSpec {}
#[doc = "`reset()` method sets RESTRICT to value 0x07"]
impl crate::Resettable for RestrictSpec {
    const RESET_VALUE: u32 = 0x07;
}
