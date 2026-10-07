#[doc = "Register `Reserved` reader"]
pub type R = crate::R<ReservedSpec>;
#[doc = "Register `Reserved` writer"]
pub type W = crate::W<ReservedSpec>;
#[doc = "Field `SCLK_DIV` reader - SPI SCLK divider. SCLK period = (SCLK_DIV + 1) * 1"]
pub type SclkDivR = crate::FieldReader;
#[doc = "Field `SCLK_DIV` writer - SPI SCLK divider. SCLK period = (SCLK_DIV + 1) * 1"]
pub type SclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NOT_GAIN` reader - Not gain output: 1 - 1x (Vout = Vref * DATA / 4096); 0 - 2x (Vout = 2 * Vref * DATA / 4096);"]
pub type NotGainR = crate::BitReader;
#[doc = "Field `NOT_GAIN` writer - Not gain output: 1 - 1x (Vout = Vref * DATA / 4096); 0 - 2x (Vout = 2 * Vref * DATA / 4096);"]
pub type NotGainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_VREF` reader - VREF Input Buffer Control: 1 - buffered (Vref = 0.04..Vdd-0.04); 0 - unbuffered (Vref = 0..Vdd)"]
pub type BufVrefR = crate::BitReader;
#[doc = "Field `BUF_VREF` writer - VREF Input Buffer Control: 1 - buffered (Vref = 0.04..Vdd-0.04); 0 - unbuffered (Vref = 0..Vdd)"]
pub type BufVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY` reader - Ready to update DATA"]
pub type ReadyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - SPI SCLK divider. SCLK period = (SCLK_DIV + 1) * 1"]
    #[inline(always)]
    pub fn sclk_div(&self) -> SclkDivR {
        SclkDivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Not gain output: 1 - 1x (Vout = Vref * DATA / 4096); 0 - 2x (Vout = 2 * Vref * DATA / 4096);"]
    #[inline(always)]
    pub fn not_gain(&self) -> NotGainR {
        NotGainR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VREF Input Buffer Control: 1 - buffered (Vref = 0.04..Vdd-0.04); 0 - unbuffered (Vref = 0..Vdd)"]
    #[inline(always)]
    pub fn buf_vref(&self) -> BufVrefR {
        BufVrefR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Ready to update DATA"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SPI SCLK divider. SCLK period = (SCLK_DIV + 1) * 1"]
    #[inline(always)]
    pub fn sclk_div(&mut self) -> SclkDivW<'_, ReservedSpec> {
        SclkDivW::new(self, 0)
    }
    #[doc = "Bit 8 - Not gain output: 1 - 1x (Vout = Vref * DATA / 4096); 0 - 2x (Vout = 2 * Vref * DATA / 4096);"]
    #[inline(always)]
    pub fn not_gain(&mut self) -> NotGainW<'_, ReservedSpec> {
        NotGainW::new(self, 8)
    }
    #[doc = "Bit 9 - VREF Input Buffer Control: 1 - buffered (Vref = 0.04..Vdd-0.04); 0 - unbuffered (Vref = 0..Vdd)"]
    #[inline(always)]
    pub fn buf_vref(&mut self) -> BufVrefW<'_, ReservedSpec> {
        BufVrefW::new(self, 9)
    }
}
#[doc = "MCP4921 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReservedSpec;
impl crate::RegisterSpec for ReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved::R`](R) reader structure"]
impl crate::Readable for ReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`reserved::W`](W) writer structure"]
impl crate::Writable for ReservedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Reserved to value 0x0001_0102"]
impl crate::Resettable for ReservedSpec {
    const RESET_VALUE: u32 = 0x0001_0102;
}
