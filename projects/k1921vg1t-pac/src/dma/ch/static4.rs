#[doc = "Register `STATIC4` reader"]
pub type R = crate::R<Static4Spec>;
#[doc = "Register `STATIC4` writer"]
pub type W = crate::W<Static4Spec>;
#[doc = "Field `RD_PER_NUM` reader - Number of peripheral to read from"]
pub type RdPerNumR = crate::FieldReader;
#[doc = "Field `RD_PER_NUM` writer - Number of peripheral to read from"]
pub type RdPerNumW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RD_PER_DELAY` reader - Peripheral latency for AXI read operation (must be set more than 1 for work with perepherals)"]
pub type RdPerDelayR = crate::FieldReader;
#[doc = "Field `RD_PER_DELAY` writer - Peripheral latency for AXI read operation (must be set more than 1 for work with perepherals)"]
pub type RdPerDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RD_PER_BLOCK` reader - Peripheral control works in block mode"]
pub type RdPerBlockR = crate::BitReader;
#[doc = "Field `RD_PER_BLOCK` writer - Peripheral control works in block mode"]
pub type RdPerBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_PER_NUM` reader - Number of peripheral to write from"]
pub type WrPerNumR = crate::FieldReader;
#[doc = "Field `WR_PER_NUM` writer - Number of peripheral to write from"]
pub type WrPerNumW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WR_PER_DELAY` reader - Peripheral latency for AXI write operation (must be set more than 1 for work with perepherals)"]
pub type WrPerDelayR = crate::FieldReader;
#[doc = "Field `WR_PER_DELAY` writer - Peripheral latency for AXI write operation (must be set more than 1 for work with perepherals)"]
pub type WrPerDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WR_PER_BLOCK` reader - Peripheral control works in block mode"]
pub type WrPerBlockR = crate::BitReader;
#[doc = "Field `WR_PER_BLOCK` writer - Peripheral control works in block mode"]
pub type WrPerBlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Number of peripheral to read from"]
    #[inline(always)]
    pub fn rd_per_num(&self) -> RdPerNumR {
        RdPerNumR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Peripheral latency for AXI read operation (must be set more than 1 for work with perepherals)"]
    #[inline(always)]
    pub fn rd_per_delay(&self) -> RdPerDelayR {
        RdPerDelayR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Peripheral control works in block mode"]
    #[inline(always)]
    pub fn rd_per_block(&self) -> RdPerBlockR {
        RdPerBlockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Number of peripheral to write from"]
    #[inline(always)]
    pub fn wr_per_num(&self) -> WrPerNumR {
        WrPerNumR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:26 - Peripheral latency for AXI write operation (must be set more than 1 for work with perepherals)"]
    #[inline(always)]
    pub fn wr_per_delay(&self) -> WrPerDelayR {
        WrPerDelayR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Peripheral control works in block mode"]
    #[inline(always)]
    pub fn wr_per_block(&self) -> WrPerBlockR {
        WrPerBlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number of peripheral to read from"]
    #[inline(always)]
    pub fn rd_per_num(&mut self) -> RdPerNumW<'_, Static4Spec> {
        RdPerNumW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Peripheral latency for AXI read operation (must be set more than 1 for work with perepherals)"]
    #[inline(always)]
    pub fn rd_per_delay(&mut self) -> RdPerDelayW<'_, Static4Spec> {
        RdPerDelayW::new(self, 8)
    }
    #[doc = "Bit 15 - Peripheral control works in block mode"]
    #[inline(always)]
    pub fn rd_per_block(&mut self) -> RdPerBlockW<'_, Static4Spec> {
        RdPerBlockW::new(self, 15)
    }
    #[doc = "Bits 16:22 - Number of peripheral to write from"]
    #[inline(always)]
    pub fn wr_per_num(&mut self) -> WrPerNumW<'_, Static4Spec> {
        WrPerNumW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Peripheral latency for AXI write operation (must be set more than 1 for work with perepherals)"]
    #[inline(always)]
    pub fn wr_per_delay(&mut self) -> WrPerDelayW<'_, Static4Spec> {
        WrPerDelayW::new(self, 24)
    }
    #[doc = "Bit 31 - Peripheral control works in block mode"]
    #[inline(always)]
    pub fn wr_per_block(&mut self) -> WrPerBlockW<'_, Static4Spec> {
        WrPerBlockW::new(self, 31)
    }
}
#[doc = "Static register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`static4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Static4Spec;
impl crate::RegisterSpec for Static4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`static4::R`](R) reader structure"]
impl crate::Readable for Static4Spec {}
#[doc = "`write(|w| ..)` method takes [`static4::W`](W) writer structure"]
impl crate::Writable for Static4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATIC4 to value 0x0100_0100"]
impl crate::Resettable for Static4Spec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
