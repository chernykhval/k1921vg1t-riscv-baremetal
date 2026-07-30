#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `A1IE` reader - Alarm 1 interrupt enable"]
pub type A1ieR = crate::BitReader;
#[doc = "Field `A1IE` writer - Alarm 1 interrupt enable"]
pub type A1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A2IE` reader - Alarm 2 interrupt enable"]
pub type A2ieR = crate::BitReader;
#[doc = "Field `A2IE` writer - Alarm 2 interrupt enable"]
pub type A2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - Power switched to battery interrupt enable"]
pub type BieR = crate::BitReader;
#[doc = "Field `BIE` writer - Power switched to battery interrupt enable"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPIE` reader - Switch to normal power supply interrupt enable"]
pub type NpieR = crate::BitReader;
#[doc = "Field `NPIE` writer - Switch to normal power supply interrupt enable"]
pub type NpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIE` reader - Timer overflow interrupt enable"]
pub type OieR = crate::BitReader;
#[doc = "Field `OIE` writer - Timer overflow interrupt enable"]
pub type OieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRC` reader - Disable RC generator"]
pub type DrcR = crate::BitReader;
#[doc = "Field `DRC` writer - Disable RC generator"]
pub type DrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Use 32kHz instead of 1Hz clock signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcm {
    #[doc = "0: The time register is incremented each second"]
    NormalMode = 0,
    #[doc = "1: 32 kHz clock is used to increment the time register"]
    FastMode = 1,
}
impl From<Fcm> for bool {
    #[inline(always)]
    fn from(variant: Fcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCM` reader - Use 32kHz instead of 1Hz clock signal"]
pub type FcmR = crate::BitReader<Fcm>;
impl FcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcm {
        match self.bits {
            false => Fcm::NormalMode,
            true => Fcm::FastMode,
        }
    }
    #[doc = "The time register is incremented each second"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Fcm::NormalMode
    }
    #[doc = "32 kHz clock is used to increment the time register"]
    #[inline(always)]
    pub fn is_fast_mode(&self) -> bool {
        *self == Fcm::FastMode
    }
}
#[doc = "Field `FCM` writer - Use 32kHz instead of 1Hz clock signal"]
pub type FcmW<'a, REG> = crate::BitWriter<'a, REG, Fcm>;
impl<'a, REG> FcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The time register is incremented each second"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Fcm::NormalMode)
    }
    #[doc = "32 kHz clock is used to increment the time register"]
    #[inline(always)]
    pub fn fast_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Fcm::FastMode)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm 1 interrupt enable"]
    #[inline(always)]
    pub fn a1ie(&self) -> A1ieR {
        A1ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm 2 interrupt enable"]
    #[inline(always)]
    pub fn a2ie(&self) -> A2ieR {
        A2ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power switched to battery interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Switch to normal power supply interrupt enable"]
    #[inline(always)]
    pub fn npie(&self) -> NpieR {
        NpieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer overflow interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OieR {
        OieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable RC generator"]
    #[inline(always)]
    pub fn drc(&self) -> DrcR {
        DrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Use 32kHz instead of 1Hz clock signal"]
    #[inline(always)]
    pub fn fcm(&self) -> FcmR {
        FcmR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 1 interrupt enable"]
    #[inline(always)]
    pub fn a1ie(&mut self) -> A1ieW<'_, ControlSpec> {
        A1ieW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm 2 interrupt enable"]
    #[inline(always)]
    pub fn a2ie(&mut self) -> A2ieW<'_, ControlSpec> {
        A2ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Power switched to battery interrupt enable"]
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<'_, ControlSpec> {
        BieW::new(self, 2)
    }
    #[doc = "Bit 3 - Switch to normal power supply interrupt enable"]
    #[inline(always)]
    pub fn npie(&mut self) -> NpieW<'_, ControlSpec> {
        NpieW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer overflow interrupt enable"]
    #[inline(always)]
    pub fn oie(&mut self) -> OieW<'_, ControlSpec> {
        OieW::new(self, 4)
    }
    #[doc = "Bit 8 - Disable RC generator"]
    #[inline(always)]
    pub fn drc(&mut self) -> DrcW<'_, ControlSpec> {
        DrcW::new(self, 8)
    }
    #[doc = "Bit 9 - Use 32kHz instead of 1Hz clock signal"]
    #[inline(always)]
    pub fn fcm(&mut self) -> FcmW<'_, ControlSpec> {
        FcmW::new(self, 9)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
