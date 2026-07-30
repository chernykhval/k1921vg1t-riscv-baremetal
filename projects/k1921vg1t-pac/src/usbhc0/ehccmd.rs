#[doc = "Register `EHCCMD` reader"]
pub type R = crate::R<EhccmdSpec>;
#[doc = "Register `EHCCMD` writer"]
pub type W = crate::W<EhccmdSpec>;
#[doc = "Field `RUNSTOP` reader - Run/Stop"]
pub type RunstopR = crate::BitReader;
#[doc = "Field `RUNSTOP` writer - Run/Stop"]
pub type RunstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCRESET` reader - Host Controller Reset"]
pub type HcresetR = crate::BitReader;
#[doc = "Field `HCRESET` writer - Host Controller Reset"]
pub type HcresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLSIZE` reader - Frame List Size"]
pub type FlsizeR = crate::FieldReader;
#[doc = "Field `FLSIZE` writer - Frame List Size"]
pub type FlsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERSCH_EN` reader - Periodic Schedule Enable"]
pub type PerschEnR = crate::BitReader;
#[doc = "Field `PERSCH_EN` writer - Periodic Schedule Enable"]
pub type PerschEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASSCH_EN` reader - Asynchronous Schedule Enable"]
pub type AsschEnR = crate::BitReader;
#[doc = "Field `ASSCH_EN` writer - Asynchronous Schedule Enable"]
pub type AsschEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNC_ADV_DB` reader - Interrupt on Async Advance Doorbell"]
pub type AsyncAdvDbR = crate::BitReader;
#[doc = "Field `ASYNC_ADV_DB` writer - Interrupt on Async Advance Doorbell"]
pub type AsyncAdvDbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASPM_CNT` reader - Asynchronouos Schedule Park Mode Count"]
pub type AspmCntR = crate::FieldReader;
#[doc = "Field `ASPM_CNT` writer - Asynchronouos Schedule Park Mode Count"]
pub type AspmCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ASPM_EN` reader - Asynchronous Schedule Park Mode Enable"]
pub type AspmEnR = crate::BitReader;
#[doc = "Field `ASPM_EN` writer - Asynchronous Schedule Park Mode Enable"]
pub type AspmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTHRESCTRL` reader - Interrupt Threshold Control"]
pub type IntthresctrlR = crate::FieldReader;
#[doc = "Field `INTTHRESCTRL` writer - Interrupt Threshold Control"]
pub type IntthresctrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Run/Stop"]
    #[inline(always)]
    pub fn runstop(&self) -> RunstopR {
        RunstopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset"]
    #[inline(always)]
    pub fn hcreset(&self) -> HcresetR {
        HcresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Frame List Size"]
    #[inline(always)]
    pub fn flsize(&self) -> FlsizeR {
        FlsizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline(always)]
    pub fn persch_en(&self) -> PerschEnR {
        PerschEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline(always)]
    pub fn assch_en(&self) -> AsschEnR {
        AsschEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell"]
    #[inline(always)]
    pub fn async_adv_db(&self) -> AsyncAdvDbR {
        AsyncAdvDbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Asynchronouos Schedule Park Mode Count"]
    #[inline(always)]
    pub fn aspm_cnt(&self) -> AspmCntR {
        AspmCntR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable"]
    #[inline(always)]
    pub fn aspm_en(&self) -> AspmEnR {
        AspmEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline(always)]
    pub fn intthresctrl(&self) -> IntthresctrlR {
        IntthresctrlR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop"]
    #[inline(always)]
    pub fn runstop(&mut self) -> RunstopW<'_, EhccmdSpec> {
        RunstopW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Controller Reset"]
    #[inline(always)]
    pub fn hcreset(&mut self) -> HcresetW<'_, EhccmdSpec> {
        HcresetW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Frame List Size"]
    #[inline(always)]
    pub fn flsize(&mut self) -> FlsizeW<'_, EhccmdSpec> {
        FlsizeW::new(self, 2)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline(always)]
    pub fn persch_en(&mut self) -> PerschEnW<'_, EhccmdSpec> {
        PerschEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline(always)]
    pub fn assch_en(&mut self) -> AsschEnW<'_, EhccmdSpec> {
        AsschEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell"]
    #[inline(always)]
    pub fn async_adv_db(&mut self) -> AsyncAdvDbW<'_, EhccmdSpec> {
        AsyncAdvDbW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Asynchronouos Schedule Park Mode Count"]
    #[inline(always)]
    pub fn aspm_cnt(&mut self) -> AspmCntW<'_, EhccmdSpec> {
        AspmCntW::new(self, 8)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable"]
    #[inline(always)]
    pub fn aspm_en(&mut self) -> AspmEnW<'_, EhccmdSpec> {
        AspmEnW::new(self, 11)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline(always)]
    pub fn intthresctrl(&mut self) -> IntthresctrlW<'_, EhccmdSpec> {
        IntthresctrlW::new(self, 16)
    }
}
#[doc = "USBHC EHC Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehccmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehccmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EhccmdSpec;
impl crate::RegisterSpec for EhccmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehccmd::R`](R) reader structure"]
impl crate::Readable for EhccmdSpec {}
#[doc = "`write(|w| ..)` method takes [`ehccmd::W`](W) writer structure"]
impl crate::Writable for EhccmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCCMD to value 0x0008_0b00"]
impl crate::Resettable for EhccmdSpec {
    const RESET_VALUE: u32 = 0x0008_0b00;
}
