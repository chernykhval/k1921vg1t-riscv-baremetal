#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `SWRES` reader - Software Reset and Reset Status"]
pub type SwresR = crate::BitReader;
#[doc = "Field `SWRES` writer - Software Reset and Reset Status"]
pub type SwresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA` reader - Enable CAN Channel"]
pub type EnaR = crate::BitReader;
#[doc = "Field `ENA` writer - Enable CAN Channel"]
pub type EnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - Test mode (not used)"]
pub type TestR = crate::BitReader;
#[doc = "Field `TEST` writer - Test mode (not used)"]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LISTEN` reader - Listen Mode"]
pub type ListenR = crate::BitReader;
#[doc = "Field `LISTEN` writer - Listen Mode"]
pub type ListenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TTC Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Synttc {
    #[doc = "0: Capture TTC Timer by SOF bit"]
    CapSof = 0,
    #[doc = "1: Capture TTC Timer by EOF bit"]
    CapEof = 1,
}
impl From<Synttc> for bool {
    #[inline(always)]
    fn from(variant: Synttc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNTTC` reader - TTC Capture Mode"]
pub type SynttcR = crate::BitReader<Synttc>;
impl SynttcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Synttc {
        match self.bits {
            false => Synttc::CapSof,
            true => Synttc::CapEof,
        }
    }
    #[doc = "Capture TTC Timer by SOF bit"]
    #[inline(always)]
    pub fn is_cap_sof(&self) -> bool {
        *self == Synttc::CapSof
    }
    #[doc = "Capture TTC Timer by EOF bit"]
    #[inline(always)]
    pub fn is_cap_eof(&self) -> bool {
        *self == Synttc::CapEof
    }
}
#[doc = "Field `SYNTTC` writer - TTC Capture Mode"]
pub type SynttcW<'a, REG> = crate::BitWriter<'a, REG, Synttc>;
impl<'a, REG> SynttcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture TTC Timer by SOF bit"]
    #[inline(always)]
    pub fn cap_sof(self) -> &'a mut crate::W<REG> {
        self.variant(Synttc::CapSof)
    }
    #[doc = "Capture TTC Timer by EOF bit"]
    #[inline(always)]
    pub fn cap_eof(self) -> &'a mut crate::W<REG> {
        self.variant(Synttc::CapEof)
    }
}
#[doc = "Field `TTC` reader - TTC Mode Enable"]
pub type TtcR = crate::BitReader;
#[doc = "Field `TTC` writer - TTC Mode Enable"]
pub type TtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRQ` reader - Request for overrun frame"]
pub type OvrqR = crate::BitReader;
#[doc = "Field `OVRQ` writer - Request for overrun frame"]
pub type OvrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRQ` reader - Break request"]
pub type AbrqR = crate::BitReader;
#[doc = "Field `ABRQ` writer - Break request"]
pub type AbrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset and Reset Status"]
    #[inline(always)]
    pub fn swres(&self) -> SwresR {
        SwresR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable CAN Channel"]
    #[inline(always)]
    pub fn ena(&self) -> EnaR {
        EnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test mode (not used)"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Listen Mode"]
    #[inline(always)]
    pub fn listen(&self) -> ListenR {
        ListenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TTC Capture Mode"]
    #[inline(always)]
    pub fn synttc(&self) -> SynttcR {
        SynttcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TTC Mode Enable"]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Request for overrun frame"]
    #[inline(always)]
    pub fn ovrq(&self) -> OvrqR {
        OvrqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break request"]
    #[inline(always)]
    pub fn abrq(&self) -> AbrqR {
        AbrqR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset and Reset Status"]
    #[inline(always)]
    pub fn swres(&mut self) -> SwresW<'_, ControlSpec> {
        SwresW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable CAN Channel"]
    #[inline(always)]
    pub fn ena(&mut self) -> EnaW<'_, ControlSpec> {
        EnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Test mode (not used)"]
    #[inline(always)]
    pub fn test(&mut self) -> TestW<'_, ControlSpec> {
        TestW::new(self, 2)
    }
    #[doc = "Bit 3 - Listen Mode"]
    #[inline(always)]
    pub fn listen(&mut self) -> ListenW<'_, ControlSpec> {
        ListenW::new(self, 3)
    }
    #[doc = "Bit 4 - TTC Capture Mode"]
    #[inline(always)]
    pub fn synttc(&mut self) -> SynttcW<'_, ControlSpec> {
        SynttcW::new(self, 4)
    }
    #[doc = "Bit 5 - TTC Mode Enable"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TtcW<'_, ControlSpec> {
        TtcW::new(self, 5)
    }
    #[doc = "Bit 6 - Request for overrun frame"]
    #[inline(always)]
    pub fn ovrq(&mut self) -> OvrqW<'_, ControlSpec> {
        OvrqW::new(self, 6)
    }
    #[doc = "Bit 7 - Break request"]
    #[inline(always)]
    pub fn abrq(&mut self) -> AbrqW<'_, ControlSpec> {
        AbrqW::new(self, 7)
    }
}
#[doc = "CAN General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {}
