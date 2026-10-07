#[doc = "Register `EHCINTR` reader"]
pub type R = crate::R<EhcintrSpec>;
#[doc = "Register `EHCINTR` writer"]
pub type W = crate::W<EhcintrSpec>;
#[doc = "Field `USBINT` reader - USB Interrupt Enable"]
pub type UsbintR = crate::BitReader;
#[doc = "Field `USBINT` writer - USB Interrupt Enable"]
pub type UsbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBERRINT` reader - USB Error Interrupt Enable"]
pub type UsberrintR = crate::BitReader;
#[doc = "Field `USBERRINT` writer - USB Error Interrupt Enable"]
pub type UsberrintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCD` reader - Port Change Detect Enable"]
pub type PcdR = crate::BitReader;
#[doc = "Field `PCD` writer - Port Change Detect Enable"]
pub type PcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLR` reader - Frame List Rollover Enable"]
pub type FlrR = crate::BitReader;
#[doc = "Field `FLR` writer - Frame List Rollover Enable"]
pub type FlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSE` reader - Host System Error Enable"]
pub type HseR = crate::BitReader;
#[doc = "Field `HSE` writer - Host System Error Enable"]
pub type HseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IAA` reader - Interrupt on Async Advance Enable"]
pub type IaaR = crate::BitReader;
#[doc = "Field `IAA` writer - Interrupt on Async Advance Enable"]
pub type IaaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB Interrupt Enable"]
    #[inline(always)]
    pub fn usbint(&self) -> UsbintR {
        UsbintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable"]
    #[inline(always)]
    pub fn usberrint(&self) -> UsberrintR {
        UsberrintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Detect Enable"]
    #[inline(always)]
    pub fn pcd(&self) -> PcdR {
        PcdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Enable"]
    #[inline(always)]
    pub fn flr(&self) -> FlrR {
        FlrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host System Error Enable"]
    #[inline(always)]
    pub fn hse(&self) -> HseR {
        HseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance Enable"]
    #[inline(always)]
    pub fn iaa(&self) -> IaaR {
        IaaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt Enable"]
    #[inline(always)]
    pub fn usbint(&mut self) -> UsbintW<'_, EhcintrSpec> {
        UsbintW::new(self, 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable"]
    #[inline(always)]
    pub fn usberrint(&mut self) -> UsberrintW<'_, EhcintrSpec> {
        UsberrintW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Change Detect Enable"]
    #[inline(always)]
    pub fn pcd(&mut self) -> PcdW<'_, EhcintrSpec> {
        PcdW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame List Rollover Enable"]
    #[inline(always)]
    pub fn flr(&mut self) -> FlrW<'_, EhcintrSpec> {
        FlrW::new(self, 3)
    }
    #[doc = "Bit 4 - Host System Error Enable"]
    #[inline(always)]
    pub fn hse(&mut self) -> HseW<'_, EhcintrSpec> {
        HseW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance Enable"]
    #[inline(always)]
    pub fn iaa(&mut self) -> IaaW<'_, EhcintrSpec> {
        IaaW::new(self, 5)
    }
}
#[doc = "USBHC EHC Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehcintr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehcintr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EhcintrSpec;
impl crate::RegisterSpec for EhcintrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehcintr::R`](R) reader structure"]
impl crate::Readable for EhcintrSpec {}
#[doc = "`write(|w| ..)` method takes [`ehcintr::W`](W) writer structure"]
impl crate::Writable for EhcintrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCINTR to value 0"]
impl crate::Resettable for EhcintrSpec {}
