#[doc = "Register `STATIC1` reader"]
pub type R = crate::R<Static1Spec>;
#[doc = "Register `STATIC1` writer"]
pub type W = crate::W<Static1Spec>;
#[doc = "Field `WR_BURST_MAX` reader - Maximum number of bytes of an AXI write burst"]
pub type WrBurstMaxR = crate::FieldReader;
#[doc = "Field `WR_BURST_MAX` writer - Maximum number of bytes of an AXI write burst"]
pub type WrBurstMaxW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WR_TOKENS` reader - Number of AXI write commands to issue before the channel releases the AXI command bus"]
pub type WrTokensR = crate::FieldReader;
#[doc = "Field `WR_TOKENS` writer - Number of AXI write commands to issue before the channel releases the AXI command bus"]
pub type WrTokensW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WR_OUTS_MAX` reader - Number of maximum outstanding AXI write commands"]
pub type WrOutsMaxR = crate::FieldReader;
#[doc = "Field `WR_OUTS_MAX` writer - Number of maximum outstanding AXI write commands"]
pub type WrOutsMaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WR_OUTSTCFG` reader - Channel cfg: Use AXI write command when FIFO is full - Not implemented"]
pub type WrOutstcfgR = crate::BitReader;
#[doc = "Field `WR_INCR` reader - Increment next burst address"]
pub type WrIncrR = crate::BitReader;
#[doc = "Field `WR_INCR` writer - Increment next burst address"]
pub type WrIncrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Maximum number of bytes of an AXI write burst"]
    #[inline(always)]
    pub fn wr_burst_max(&self) -> WrBurstMaxR {
        WrBurstMaxR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Number of AXI write commands to issue before the channel releases the AXI command bus"]
    #[inline(always)]
    pub fn wr_tokens(&self) -> WrTokensR {
        WrTokensR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of maximum outstanding AXI write commands"]
    #[inline(always)]
    pub fn wr_outs_max(&self) -> WrOutsMaxR {
        WrOutsMaxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Channel cfg: Use AXI write command when FIFO is full - Not implemented"]
    #[inline(always)]
    pub fn wr_outstcfg(&self) -> WrOutstcfgR {
        WrOutstcfgR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Increment next burst address"]
    #[inline(always)]
    pub fn wr_incr(&self) -> WrIncrR {
        WrIncrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Maximum number of bytes of an AXI write burst"]
    #[inline(always)]
    pub fn wr_burst_max(&mut self) -> WrBurstMaxW<'_, Static1Spec> {
        WrBurstMaxW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Number of AXI write commands to issue before the channel releases the AXI command bus"]
    #[inline(always)]
    pub fn wr_tokens(&mut self) -> WrTokensW<'_, Static1Spec> {
        WrTokensW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of maximum outstanding AXI write commands"]
    #[inline(always)]
    pub fn wr_outs_max(&mut self) -> WrOutsMaxW<'_, Static1Spec> {
        WrOutsMaxW::new(self, 24)
    }
    #[doc = "Bit 31 - Increment next burst address"]
    #[inline(always)]
    pub fn wr_incr(&mut self) -> WrIncrW<'_, Static1Spec> {
        WrIncrW::new(self, 31)
    }
}
#[doc = "Static register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`static1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Static1Spec;
impl crate::RegisterSpec for Static1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`static1::R`](R) reader structure"]
impl crate::Readable for Static1Spec {}
#[doc = "`write(|w| ..)` method takes [`static1::W`](W) writer structure"]
impl crate::Writable for Static1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATIC1 to value 0x8001_0000"]
impl crate::Resettable for Static1Spec {
    const RESET_VALUE: u32 = 0x8001_0000;
}
