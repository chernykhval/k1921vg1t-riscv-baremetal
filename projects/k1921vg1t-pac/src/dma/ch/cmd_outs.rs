#[doc = "Register `CMD_OUTS` reader"]
pub type R = crate::R<CmdOutsSpec>;
#[doc = "Field `RD_CMD_OUTS` reader - Number of channels pending AXI read commands"]
pub type RdCmdOutsR = crate::FieldReader;
#[doc = "Field `WR_CMD_OUTS` reader - Number of channels pending AXI write commands"]
pub type WrCmdOutsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Number of channels pending AXI read commands"]
    #[inline(always)]
    pub fn rd_cmd_outs(&self) -> RdCmdOutsR {
        RdCmdOutsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of channels pending AXI write commands"]
    #[inline(always)]
    pub fn wr_cmd_outs(&self) -> WrCmdOutsR {
        WrCmdOutsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Outstanding commands status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_outs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdOutsSpec;
impl crate::RegisterSpec for CmdOutsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_outs::R`](R) reader structure"]
impl crate::Readable for CmdOutsSpec {}
#[doc = "`reset()` method sets CMD_OUTS to value 0"]
impl crate::Resettable for CmdOutsSpec {}
