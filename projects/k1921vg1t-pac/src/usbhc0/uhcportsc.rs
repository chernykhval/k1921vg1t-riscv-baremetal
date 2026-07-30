#[doc = "Register `UHCPORTSC` reader"]
pub type R = crate::R<UhcportscSpec>;
#[doc = "Register `UHCPORTSC` writer"]
pub type W = crate::W<UhcportscSpec>;
#[doc = "Field `CS` reader - Current Connect Status"]
pub type CsR = crate::BitReader;
#[doc = "Field `CSC` reader - Connect Status Change"]
pub type CscR = crate::BitReader;
#[doc = "Field `CSC` writer - Connect Status Change"]
pub type CscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PED` reader - Port Enabled/Disabled"]
pub type PedR = crate::BitReader;
#[doc = "Field `PED` writer - Port Enabled/Disabled"]
pub type PedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - Port Enable/Disable Change"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Port Enable/Disable Change"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS` reader - Line Status"]
pub type LsR = crate::FieldReader;
#[doc = "Field `RD` reader - Resume Detect"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Resume Detect"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSDA` reader - Low Speed Device Attached"]
pub type LsdaR = crate::BitReader;
#[doc = "Field `PR` reader - Port Reset"]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Port Reset"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA` reader - Over-current Active"]
pub type OaR = crate::BitReader;
#[doc = "Field `OC` reader - Over-current Change"]
pub type OcR = crate::BitReader;
#[doc = "Field `OC` writer - Over-current Change"]
pub type OcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Suspend"]
pub type SR = crate::BitReader;
#[doc = "Field `S` writer - Suspend"]
pub type SW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current Connect Status"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change"]
    #[inline(always)]
    pub fn csc(&self) -> CscR {
        CscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    pub fn ped(&self) -> PedR {
        PedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Line Status"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Resume Detect"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Speed Device Attached"]
    #[inline(always)]
    pub fn lsda(&self) -> LsdaR {
        LsdaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Reset"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Over-current Active"]
    #[inline(always)]
    pub fn oa(&self) -> OaR {
        OaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Over-current Change"]
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Suspend"]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change"]
    #[inline(always)]
    pub fn csc(&mut self) -> CscW<'_, UhcportscSpec> {
        CscW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    pub fn ped(&mut self) -> PedW<'_, UhcportscSpec> {
        PedW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, UhcportscSpec> {
        PeW::new(self, 3)
    }
    #[doc = "Bit 6 - Resume Detect"]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<'_, UhcportscSpec> {
        RdW::new(self, 6)
    }
    #[doc = "Bit 9 - Port Reset"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<'_, UhcportscSpec> {
        PrW::new(self, 9)
    }
    #[doc = "Bit 11 - Over-current Change"]
    #[inline(always)]
    pub fn oc(&mut self) -> OcW<'_, UhcportscSpec> {
        OcW::new(self, 11)
    }
    #[doc = "Bit 12 - Suspend"]
    #[inline(always)]
    pub fn s(&mut self) -> SW<'_, UhcportscSpec> {
        SW::new(self, 12)
    }
}
#[doc = "USBHC UHC Port Status and Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhcportsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhcportsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhcportscSpec;
impl crate::RegisterSpec for UhcportscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhcportsc::R`](R) reader structure"]
impl crate::Readable for UhcportscSpec {}
#[doc = "`write(|w| ..)` method takes [`uhcportsc::W`](W) writer structure"]
impl crate::Writable for UhcportscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCPORTSC to value 0"]
impl crate::Resettable for UhcportscSpec {}
