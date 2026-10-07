#[doc = "Register `UHCCMDSTS` reader"]
pub type R = crate::R<UhccmdstsSpec>;
#[doc = "Register `UHCCMDSTS` writer"]
pub type W = crate::W<UhccmdstsSpec>;
#[doc = "Field `RS` reader - Run/Stop"]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - Run/Stop"]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCRESET` reader - Host Controller Reset"]
pub type HcresetR = crate::BitReader;
#[doc = "Field `HCRESET` writer - Host Controller Reset"]
pub type HcresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRESET` reader - Global Reset"]
pub type GresetR = crate::BitReader;
#[doc = "Field `GRESET` writer - Global Reset"]
pub type GresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EGSM` reader - Enter Global Suspend Mode"]
pub type EgsmR = crate::BitReader;
#[doc = "Field `EGSM` writer - Enter Global Suspend Mode"]
pub type EgsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGR` reader - Force Global Resume"]
pub type FgrR = crate::BitReader;
#[doc = "Field `FGR` writer - Force Global Resume"]
pub type FgrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDBG` reader - Software Debug"]
pub type SwdbgR = crate::BitReader;
#[doc = "Field `SWDBG` writer - Software Debug"]
pub type SwdbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CF` reader - Confiduire Flag"]
pub type CfR = crate::BitReader;
#[doc = "Field `CF` writer - Confiduire Flag"]
pub type CfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXP` reader - Maximum Packet Size"]
pub type MaxpR = crate::BitReader;
#[doc = "Field `MAXP` writer - Maximum Packet Size"]
pub type MaxpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBINT` reader - USB Interrupt"]
pub type UsbintR = crate::BitReader;
#[doc = "Field `USBINT` writer - USB Interrupt"]
pub type UsbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBERRINT` reader - USB Error Interrupt"]
pub type UsberrintR = crate::BitReader;
#[doc = "Field `USBERRINT` writer - USB Error Interrupt"]
pub type UsberrintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Resume Detect"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Resume Detect"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSE` reader - Host System Error"]
pub type HseR = crate::BitReader;
#[doc = "Field `HSE` writer - Host System Error"]
pub type HseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPE` reader - Host Controller Process Error"]
pub type HcpeR = crate::BitReader;
#[doc = "Field `HCPE` writer - Host Controller Process Error"]
pub type HcpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCH` reader - HCHalted"]
pub type HchR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Run/Stop"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset"]
    #[inline(always)]
    pub fn hcreset(&self) -> HcresetR {
        HcresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Reset"]
    #[inline(always)]
    pub fn greset(&self) -> GresetR {
        GresetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enter Global Suspend Mode"]
    #[inline(always)]
    pub fn egsm(&self) -> EgsmR {
        EgsmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Global Resume"]
    #[inline(always)]
    pub fn fgr(&self) -> FgrR {
        FgrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Debug"]
    #[inline(always)]
    pub fn swdbg(&self) -> SwdbgR {
        SwdbgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Confiduire Flag"]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Maximum Packet Size"]
    #[inline(always)]
    pub fn maxp(&self) -> MaxpR {
        MaxpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - USB Interrupt"]
    #[inline(always)]
    pub fn usbint(&self) -> UsbintR {
        UsbintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB Error Interrupt"]
    #[inline(always)]
    pub fn usberrint(&self) -> UsberrintR {
        UsberrintR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Resume Detect"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Host System Error"]
    #[inline(always)]
    pub fn hse(&self) -> HseR {
        HseR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Host Controller Process Error"]
    #[inline(always)]
    pub fn hcpe(&self) -> HcpeR {
        HcpeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HCHalted"]
    #[inline(always)]
    pub fn hch(&self) -> HchR {
        HchR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop"]
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<'_, UhccmdstsSpec> {
        RsW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Controller Reset"]
    #[inline(always)]
    pub fn hcreset(&mut self) -> HcresetW<'_, UhccmdstsSpec> {
        HcresetW::new(self, 1)
    }
    #[doc = "Bit 2 - Global Reset"]
    #[inline(always)]
    pub fn greset(&mut self) -> GresetW<'_, UhccmdstsSpec> {
        GresetW::new(self, 2)
    }
    #[doc = "Bit 3 - Enter Global Suspend Mode"]
    #[inline(always)]
    pub fn egsm(&mut self) -> EgsmW<'_, UhccmdstsSpec> {
        EgsmW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Global Resume"]
    #[inline(always)]
    pub fn fgr(&mut self) -> FgrW<'_, UhccmdstsSpec> {
        FgrW::new(self, 4)
    }
    #[doc = "Bit 5 - Software Debug"]
    #[inline(always)]
    pub fn swdbg(&mut self) -> SwdbgW<'_, UhccmdstsSpec> {
        SwdbgW::new(self, 5)
    }
    #[doc = "Bit 6 - Confiduire Flag"]
    #[inline(always)]
    pub fn cf(&mut self) -> CfW<'_, UhccmdstsSpec> {
        CfW::new(self, 6)
    }
    #[doc = "Bit 7 - Maximum Packet Size"]
    #[inline(always)]
    pub fn maxp(&mut self) -> MaxpW<'_, UhccmdstsSpec> {
        MaxpW::new(self, 7)
    }
    #[doc = "Bit 16 - USB Interrupt"]
    #[inline(always)]
    pub fn usbint(&mut self) -> UsbintW<'_, UhccmdstsSpec> {
        UsbintW::new(self, 16)
    }
    #[doc = "Bit 17 - USB Error Interrupt"]
    #[inline(always)]
    pub fn usberrint(&mut self) -> UsberrintW<'_, UhccmdstsSpec> {
        UsberrintW::new(self, 17)
    }
    #[doc = "Bit 18 - Resume Detect"]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<'_, UhccmdstsSpec> {
        RdW::new(self, 18)
    }
    #[doc = "Bit 19 - Host System Error"]
    #[inline(always)]
    pub fn hse(&mut self) -> HseW<'_, UhccmdstsSpec> {
        HseW::new(self, 19)
    }
    #[doc = "Bit 20 - Host Controller Process Error"]
    #[inline(always)]
    pub fn hcpe(&mut self) -> HcpeW<'_, UhccmdstsSpec> {
        HcpeW::new(self, 20)
    }
}
#[doc = "USBHC UHC Command and Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhccmdsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhccmdsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhccmdstsSpec;
impl crate::RegisterSpec for UhccmdstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhccmdsts::R`](R) reader structure"]
impl crate::Readable for UhccmdstsSpec {}
#[doc = "`write(|w| ..)` method takes [`uhccmdsts::W`](W) writer structure"]
impl crate::Writable for UhccmdstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCCMDSTS to value 0x0020_0000"]
impl crate::Resettable for UhccmdstsSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
