#[doc = "Register `TIMING` reader"]
pub type R = crate::R<TimingSpec>;
#[doc = "Register `TIMING` writer"]
pub type W = crate::W<TimingSpec>;
#[doc = "Field `T_RAS` reader - SDRAM active to prechardge, specified in clocks"]
pub type TRasR = crate::FieldReader;
#[doc = "Field `T_RAS` writer - SDRAM active to prechardge, specified in clocks"]
pub type TRasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_RP` reader - SDRAM precharge command period, specified in clocks"]
pub type TRpR = crate::FieldReader;
#[doc = "Field `T_RP` writer - SDRAM precharge command period, specified in clocks"]
pub type TRpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_RCD` reader - SDRAM active to read or write delay, specified in clocks"]
pub type TRcdR = crate::FieldReader;
#[doc = "Field `T_RCD` writer - SDRAM active to read or write delay, specified in clocks"]
pub type TRcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_RC_AR` reader - SDRAM active to active/auto-refresh command period, specified in clocks"]
pub type TRcArR = crate::FieldReader;
#[doc = "Field `T_RC_AR` writer - SDRAM active to active/auto-refresh command period, specified in clocks"]
pub type TRcArW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_WR` reader - SDRAM write recovery time, specified in clocks"]
pub type TWrR = crate::FieldReader;
#[doc = "Field `T_WR` writer - SDRAM write recovery time, specified in clocks"]
pub type TWrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SDRAM active to prechardge, specified in clocks"]
    #[inline(always)]
    pub fn t_ras(&self) -> TRasR {
        TRasR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SDRAM precharge command period, specified in clocks"]
    #[inline(always)]
    pub fn t_rp(&self) -> TRpR {
        TRpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SDRAM active to read or write delay, specified in clocks"]
    #[inline(always)]
    pub fn t_rcd(&self) -> TRcdR {
        TRcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SDRAM active to active/auto-refresh command period, specified in clocks"]
    #[inline(always)]
    pub fn t_rc_ar(&self) -> TRcArR {
        TRcArR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SDRAM write recovery time, specified in clocks"]
    #[inline(always)]
    pub fn t_wr(&self) -> TWrR {
        TWrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SDRAM active to prechardge, specified in clocks"]
    #[inline(always)]
    pub fn t_ras(&mut self) -> TRasW<'_, TimingSpec> {
        TRasW::new(self, 0)
    }
    #[doc = "Bits 4:7 - SDRAM precharge command period, specified in clocks"]
    #[inline(always)]
    pub fn t_rp(&mut self) -> TRpW<'_, TimingSpec> {
        TRpW::new(self, 4)
    }
    #[doc = "Bits 8:11 - SDRAM active to read or write delay, specified in clocks"]
    #[inline(always)]
    pub fn t_rcd(&mut self) -> TRcdW<'_, TimingSpec> {
        TRcdW::new(self, 8)
    }
    #[doc = "Bits 12:15 - SDRAM active to active/auto-refresh command period, specified in clocks"]
    #[inline(always)]
    pub fn t_rc_ar(&mut self) -> TRcArW<'_, TimingSpec> {
        TRcArW::new(self, 12)
    }
    #[doc = "Bits 16:19 - SDRAM write recovery time, specified in clocks"]
    #[inline(always)]
    pub fn t_wr(&mut self) -> TWrW<'_, TimingSpec> {
        TWrW::new(self, 16)
    }
}
#[doc = "SDRAM Timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimingSpec;
impl crate::RegisterSpec for TimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing::R`](R) reader structure"]
impl crate::Readable for TimingSpec {}
#[doc = "`write(|w| ..)` method takes [`timing::W`](W) writer structure"]
impl crate::Writable for TimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMING to value 0x0001_7224"]
impl crate::Resettable for TimingSpec {
    const RESET_VALUE: u32 = 0x0001_7224;
}
