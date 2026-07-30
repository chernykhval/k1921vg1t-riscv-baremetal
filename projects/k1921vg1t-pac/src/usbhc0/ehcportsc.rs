#[doc = "Register `EHCPORTSC` reader"]
pub type R = crate::R<EhcportscSpec>;
#[doc = "Register `EHCPORTSC` writer"]
pub type W = crate::W<EhcportscSpec>;
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
#[doc = "Field `OA` reader - Over-current Active"]
pub type OaR = crate::BitReader;
#[doc = "Field `OC` reader - Over-current Change"]
pub type OcR = crate::BitReader;
#[doc = "Field `OC` writer - Over-current Change"]
pub type OcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR` reader - Force Port Resume"]
pub type FprR = crate::BitReader;
#[doc = "Field `FPR` writer - Force Port Resume"]
pub type FprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Suspend"]
pub type SR = crate::BitReader;
#[doc = "Field `S` writer - Suspend"]
pub type SW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR` reader - Port Reset"]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Port Reset"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Line Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ls {
    #[doc = "0: Not LS devoce, perform EHCI reset"]
    Se0 = 0,
    #[doc = "1: Not LS device, perform EHCI reset"]
    Jstate = 1,
    #[doc = "2: LS device, release ownership of port"]
    Kstate = 2,
    #[doc = "3: Not LS device, perform EHCI reset"]
    Undefined = 3,
}
impl From<Ls> for u8 {
    #[inline(always)]
    fn from(variant: Ls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ls {
    type Ux = u8;
}
impl crate::IsEnum for Ls {}
#[doc = "Field `LS` reader - Line Status"]
pub type LsR = crate::FieldReader<Ls>;
impl LsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ls {
        match self.bits {
            0 => Ls::Se0,
            1 => Ls::Jstate,
            2 => Ls::Kstate,
            3 => Ls::Undefined,
            _ => unreachable!(),
        }
    }
    #[doc = "Not LS devoce, perform EHCI reset"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == Ls::Se0
    }
    #[doc = "Not LS device, perform EHCI reset"]
    #[inline(always)]
    pub fn is_jstate(&self) -> bool {
        *self == Ls::Jstate
    }
    #[doc = "LS device, release ownership of port"]
    #[inline(always)]
    pub fn is_kstate(&self) -> bool {
        *self == Ls::Kstate
    }
    #[doc = "Not LS device, perform EHCI reset"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        *self == Ls::Undefined
    }
}
#[doc = "Field `PP` reader - Port Power"]
pub type PpR = crate::BitReader;
#[doc = "Field `PP` writer - Port Power"]
pub type PpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PO` reader - Port Owner"]
pub type PoR = crate::BitReader;
#[doc = "Field `PO` writer - Port Owner"]
pub type PoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIC` reader - Port Test Control"]
pub type PicR = crate::FieldReader;
#[doc = "Field `PIC` writer - Port Test Control"]
pub type PicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PTC` reader - Port Test Control"]
pub type PtcR = crate::FieldReader;
#[doc = "Field `PTC` writer - Port Test Control"]
pub type PtcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WKCNNT_E` reader - Wake on Connect Enable"]
pub type WkcnntER = crate::BitReader;
#[doc = "Field `WKCNNT_E` writer - Wake on Connect Enable"]
pub type WkcnntEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKDSCNNT_E` reader - Wake on Disconnect Enable"]
pub type WkdscnntER = crate::BitReader;
#[doc = "Field `WKDSCNNT_E` writer - Wake on Disconnect Enable"]
pub type WkdscnntEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKOC_E` reader - Wake on Over-current Enable"]
pub type WkocER = crate::BitReader;
#[doc = "Field `WKOC_E` writer - Wake on Over-current Enable"]
pub type WkocEW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - Over-current Active"]
    #[inline(always)]
    pub fn oa(&self) -> OaR {
        OaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Over-current Change"]
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn fpr(&self) -> FprR {
        FprR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Line Status"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn pp(&self) -> PpR {
        PpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Owner"]
    #[inline(always)]
    pub fn po(&self) -> PoR {
        PoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Port Test Control"]
    #[inline(always)]
    pub fn pic(&self) -> PicR {
        PicR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline(always)]
    pub fn ptc(&self) -> PtcR {
        PtcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Wake on Connect Enable"]
    #[inline(always)]
    pub fn wkcnnt_e(&self) -> WkcnntER {
        WkcnntER::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable"]
    #[inline(always)]
    pub fn wkdscnnt_e(&self) -> WkdscnntER {
        WkdscnntER::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wake on Over-current Enable"]
    #[inline(always)]
    pub fn wkoc_e(&self) -> WkocER {
        WkocER::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change"]
    #[inline(always)]
    pub fn csc(&mut self) -> CscW<'_, EhcportscSpec> {
        CscW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    pub fn ped(&mut self) -> PedW<'_, EhcportscSpec> {
        PedW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, EhcportscSpec> {
        PeW::new(self, 3)
    }
    #[doc = "Bit 5 - Over-current Change"]
    #[inline(always)]
    pub fn oc(&mut self) -> OcW<'_, EhcportscSpec> {
        OcW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn fpr(&mut self) -> FprW<'_, EhcportscSpec> {
        FprW::new(self, 6)
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline(always)]
    pub fn s(&mut self) -> SW<'_, EhcportscSpec> {
        SW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<'_, EhcportscSpec> {
        PrW::new(self, 8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn pp(&mut self) -> PpW<'_, EhcportscSpec> {
        PpW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Owner"]
    #[inline(always)]
    pub fn po(&mut self) -> PoW<'_, EhcportscSpec> {
        PoW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Port Test Control"]
    #[inline(always)]
    pub fn pic(&mut self) -> PicW<'_, EhcportscSpec> {
        PicW::new(self, 14)
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline(always)]
    pub fn ptc(&mut self) -> PtcW<'_, EhcportscSpec> {
        PtcW::new(self, 16)
    }
    #[doc = "Bit 20 - Wake on Connect Enable"]
    #[inline(always)]
    pub fn wkcnnt_e(&mut self) -> WkcnntEW<'_, EhcportscSpec> {
        WkcnntEW::new(self, 20)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable"]
    #[inline(always)]
    pub fn wkdscnnt_e(&mut self) -> WkdscnntEW<'_, EhcportscSpec> {
        WkdscnntEW::new(self, 21)
    }
    #[doc = "Bit 22 - Wake on Over-current Enable"]
    #[inline(always)]
    pub fn wkoc_e(&mut self) -> WkocEW<'_, EhcportscSpec> {
        WkocEW::new(self, 22)
    }
}
#[doc = "USBHC EHC Port Status and Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehcportsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehcportsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EhcportscSpec;
impl crate::RegisterSpec for EhcportscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehcportsc::R`](R) reader structure"]
impl crate::Readable for EhcportscSpec {}
#[doc = "`write(|w| ..)` method takes [`ehcportsc::W`](W) writer structure"]
impl crate::Writable for EhcportscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCPORTSC to value 0x2000"]
impl crate::Resettable for EhcportscSpec {
    const RESET_VALUE: u32 = 0x2000;
}
