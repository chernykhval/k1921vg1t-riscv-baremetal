#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `OVF` reader - Capture Overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Capture Overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT` reader - Output"]
pub type OutR = crate::BitReader;
#[doc = "Field `OUT` writer - Output"]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCI` reader - Capture / Compare Input"]
pub type CciR = crate::BitReader;
#[doc = "Field `OUTMODE` reader - Output Mode"]
pub type OutmodeR = crate::FieldReader;
#[doc = "Field `OUTMODE` writer - Output Mode"]
pub type OutmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAP` reader - Capture Mode"]
pub type CapR = crate::BitReader;
#[doc = "Field `CAP` writer - Capture Mode"]
pub type CapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCCI` reader - Synchronized Capture / Compare Input"]
pub type ScciR = crate::BitReader;
#[doc = "Field `CCISEL` reader - Capture / Compare Input Select"]
pub type CciselR = crate::FieldReader;
#[doc = "Field `CCISEL` writer - Capture / Compare Input Select"]
pub type CciselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAPMODE` reader - Capture Mode"]
pub type CapmodeR = crate::FieldReader;
#[doc = "Field `CAPMODE` writer - Capture Mode"]
pub type CapmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Capture Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture / Compare Input"]
    #[inline(always)]
    pub fn cci(&self) -> CciR {
        CciR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Output Mode"]
    #[inline(always)]
    pub fn outmode(&self) -> OutmodeR {
        OutmodeR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Capture Mode"]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronized Capture / Compare Input"]
    #[inline(always)]
    pub fn scci(&self) -> ScciR {
        ScciR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Capture / Compare Input Select"]
    #[inline(always)]
    pub fn ccisel(&self) -> CciselR {
        CciselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Capture Mode"]
    #[inline(always)]
    pub fn capmode(&self) -> CapmodeR {
        CapmodeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Capture Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<'_, CtrlSpec> {
        OvfW::new(self, 1)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, CtrlSpec> {
        OutW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Output Mode"]
    #[inline(always)]
    pub fn outmode(&mut self) -> OutmodeW<'_, CtrlSpec> {
        OutmodeW::new(self, 5)
    }
    #[doc = "Bit 8 - Capture Mode"]
    #[inline(always)]
    pub fn cap(&mut self) -> CapW<'_, CtrlSpec> {
        CapW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Capture / Compare Input Select"]
    #[inline(always)]
    pub fn ccisel(&mut self) -> CciselW<'_, CtrlSpec> {
        CciselW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Capture Mode"]
    #[inline(always)]
    pub fn capmode(&mut self) -> CapmodeW<'_, CtrlSpec> {
        CapmodeW::new(self, 14)
    }
}
#[doc = "Capture / Compare Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x20"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x20;
}
