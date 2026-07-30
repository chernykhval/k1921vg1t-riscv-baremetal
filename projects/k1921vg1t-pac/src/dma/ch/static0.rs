#[doc = "Register `STATIC0` reader"]
pub type R = crate::R<Static0Spec>;
#[doc = "Register `STATIC0` writer"]
pub type W = crate::W<Static0Spec>;
#[doc = "Field `RD_BURST_MAX` reader - Maximum number of bytes of an AXI read burst"]
pub type RdBurstMaxR = crate::FieldReader;
#[doc = "Field `RD_BURST_MAX` writer - Maximum number of bytes of an AXI read burst"]
pub type RdBurstMaxW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RD_TOKENS` reader - Number of AXI read commands to issue before the channel releases the AXI command bus"]
pub type RdTokensR = crate::FieldReader;
#[doc = "Field `RD_TOKENS` writer - Number of AXI read commands to issue before the channel releases the AXI command bus"]
pub type RdTokensW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD_OUTS_MAX` reader - Number of maximum outstanding AXI read commands"]
pub type RdOutsMaxR = crate::FieldReader;
#[doc = "Field `RD_OUTS_MAX` writer - Number of maximum outstanding AXI read commands"]
pub type RdOutsMaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RD_OUTSTCFG` reader - Channel cfg: Use AXI read command when FIFO is full - Not implemented"]
pub type RdOutstcfgR = crate::BitReader;
#[doc = "Field `RD_INCR` reader - Increment next burst address"]
pub type RdIncrR = crate::BitReader;
#[doc = "Field `RD_INCR` writer - Increment next burst address"]
pub type RdIncrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Maximum number of bytes of an AXI read burst"]
    #[inline(always)]
    pub fn rd_burst_max(&self) -> RdBurstMaxR {
        RdBurstMaxR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Number of AXI read commands to issue before the channel releases the AXI command bus"]
    #[inline(always)]
    pub fn rd_tokens(&self) -> RdTokensR {
        RdTokensR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of maximum outstanding AXI read commands"]
    #[inline(always)]
    pub fn rd_outs_max(&self) -> RdOutsMaxR {
        RdOutsMaxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Channel cfg: Use AXI read command when FIFO is full - Not implemented"]
    #[inline(always)]
    pub fn rd_outstcfg(&self) -> RdOutstcfgR {
        RdOutstcfgR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Increment next burst address"]
    #[inline(always)]
    pub fn rd_incr(&self) -> RdIncrR {
        RdIncrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Maximum number of bytes of an AXI read burst"]
    #[inline(always)]
    pub fn rd_burst_max(&mut self) -> RdBurstMaxW<'_, Static0Spec> {
        RdBurstMaxW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Number of AXI read commands to issue before the channel releases the AXI command bus"]
    #[inline(always)]
    pub fn rd_tokens(&mut self) -> RdTokensW<'_, Static0Spec> {
        RdTokensW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of maximum outstanding AXI read commands"]
    #[inline(always)]
    pub fn rd_outs_max(&mut self) -> RdOutsMaxW<'_, Static0Spec> {
        RdOutsMaxW::new(self, 24)
    }
    #[doc = "Bit 31 - Increment next burst address"]
    #[inline(always)]
    pub fn rd_incr(&mut self) -> RdIncrW<'_, Static0Spec> {
        RdIncrW::new(self, 31)
    }
}
#[doc = "Static register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`static0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Static0Spec;
impl crate::RegisterSpec for Static0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`static0::R`](R) reader structure"]
impl crate::Readable for Static0Spec {}
#[doc = "`write(|w| ..)` method takes [`static0::W`](W) writer structure"]
impl crate::Writable for Static0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATIC0 to value 0x8001_0000"]
impl crate::Resettable for Static0Spec {
    const RESET_VALUE: u32 = 0x8001_0000;
}
