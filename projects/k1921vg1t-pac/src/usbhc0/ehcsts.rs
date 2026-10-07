#[doc = "Register `EHCSTS` reader"]
pub type R = crate::R<EhcstsSpec>;
#[doc = "Register `EHCSTS` writer"]
pub type W = crate::W<EhcstsSpec>;
#[doc = "Field `USBINT` reader - USB Interrupt"]
pub type UsbintR = crate::BitReader;
#[doc = "Field `USBINT` writer - USB Interrupt"]
pub type UsbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBERRINT` reader - USB Error Interrupt"]
pub type UsberrintR = crate::BitReader;
#[doc = "Field `USBERRINT` writer - USB Error Interrupt"]
pub type UsberrintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCD` reader - Port Change Detect"]
pub type PcdR = crate::BitReader;
#[doc = "Field `PCD` writer - Port Change Detect"]
pub type PcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLR` reader - Frame List Rollover"]
pub type FlrR = crate::BitReader;
#[doc = "Field `FLR` writer - Frame List Rollover"]
pub type FlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSE` reader - Host System Error"]
pub type HseR = crate::BitReader;
#[doc = "Field `HSE` writer - Host System Error"]
pub type HseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IAA` reader - Interrupt on Async Advance"]
pub type IaaR = crate::BitReader;
#[doc = "Field `IAA` writer - Interrupt on Async Advance"]
pub type IaaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCH` reader - HCHalted"]
pub type HchR = crate::BitReader;
#[doc = "Field `R` reader - Reclamation"]
pub type RR = crate::BitReader;
#[doc = "Field `PSS` reader - Periodic Schedule Status"]
pub type PssR = crate::BitReader;
#[doc = "Field `ASS` reader - Asynchronous Schedule Status"]
pub type AssR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB Interrupt"]
    #[inline(always)]
    pub fn usbint(&self) -> UsbintR {
        UsbintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt"]
    #[inline(always)]
    pub fn usberrint(&self) -> UsberrintR {
        UsberrintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Detect"]
    #[inline(always)]
    pub fn pcd(&self) -> PcdR {
        PcdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover"]
    #[inline(always)]
    pub fn flr(&self) -> FlrR {
        FlrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host System Error"]
    #[inline(always)]
    pub fn hse(&self) -> HseR {
        HseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance"]
    #[inline(always)]
    pub fn iaa(&self) -> IaaR {
        IaaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - HCHalted"]
    #[inline(always)]
    pub fn hch(&self) -> HchR {
        HchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reclamation"]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Periodic Schedule Status"]
    #[inline(always)]
    pub fn pss(&self) -> PssR {
        PssR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous Schedule Status"]
    #[inline(always)]
    pub fn ass(&self) -> AssR {
        AssR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt"]
    #[inline(always)]
    pub fn usbint(&mut self) -> UsbintW<'_, EhcstsSpec> {
        UsbintW::new(self, 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt"]
    #[inline(always)]
    pub fn usberrint(&mut self) -> UsberrintW<'_, EhcstsSpec> {
        UsberrintW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Change Detect"]
    #[inline(always)]
    pub fn pcd(&mut self) -> PcdW<'_, EhcstsSpec> {
        PcdW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame List Rollover"]
    #[inline(always)]
    pub fn flr(&mut self) -> FlrW<'_, EhcstsSpec> {
        FlrW::new(self, 3)
    }
    #[doc = "Bit 4 - Host System Error"]
    #[inline(always)]
    pub fn hse(&mut self) -> HseW<'_, EhcstsSpec> {
        HseW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance"]
    #[inline(always)]
    pub fn iaa(&mut self) -> IaaW<'_, EhcstsSpec> {
        IaaW::new(self, 5)
    }
}
#[doc = "USBHC EHC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehcsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehcsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EhcstsSpec;
impl crate::RegisterSpec for EhcstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehcsts::R`](R) reader structure"]
impl crate::Readable for EhcstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ehcsts::W`](W) writer structure"]
impl crate::Writable for EhcstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCSTS to value 0x1000"]
impl crate::Resettable for EhcstsSpec {
    const RESET_VALUE: u32 = 0x1000;
}
