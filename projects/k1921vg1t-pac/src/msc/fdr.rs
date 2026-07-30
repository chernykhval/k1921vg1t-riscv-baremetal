#[doc = "Register `FDR` reader"]
pub type R = crate::R<FdrSpec>;
#[doc = "Register `FDR` writer"]
pub type W = crate::W<FdrSpec>;
#[doc = "Field `STEP` reader - Divider start value in the normal mode / Step of divider in the fractional mode"]
pub type StepR = crate::FieldReader<u16>;
#[doc = "Field `STEP` writer - Divider start value in the normal mode / Step of divider in the fractional mode"]
pub type StepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Mode setting of the frequency divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Divider is turned off. Fmsc isn't generated"]
    TurnOff = 0,
    #[doc = "1: Normal divider mod is selected"]
    Normal = 1,
    #[doc = "2: Fractional divider mode is selected"]
    Fractional = 2,
    #[doc = "3: Bypass mode is selected (Fmsc = Fapb / 2)"]
    Bypass = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Mode setting of the frequency divider"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::TurnOff,
            1 => Mode::Normal,
            2 => Mode::Fractional,
            3 => Mode::Bypass,
            _ => unreachable!(),
        }
    }
    #[doc = "Divider is turned off. Fmsc isn't generated"]
    #[inline(always)]
    pub fn is_turn_off(&self) -> bool {
        *self == Mode::TurnOff
    }
    #[doc = "Normal divider mod is selected"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Mode::Normal
    }
    #[doc = "Fractional divider mode is selected"]
    #[inline(always)]
    pub fn is_fractional(&self) -> bool {
        *self == Mode::Fractional
    }
    #[doc = "Bypass mode is selected (Fmsc = Fapb / 2)"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Mode::Bypass
    }
}
#[doc = "Field `MODE` writer - Mode setting of the frequency divider"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divider is turned off. Fmsc isn't generated"]
    #[inline(always)]
    pub fn turn_off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::TurnOff)
    }
    #[doc = "Normal divider mod is selected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Normal)
    }
    #[doc = "Fractional divider mode is selected"]
    #[inline(always)]
    pub fn fractional(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Fractional)
    }
    #[doc = "Bypass mode is selected (Fmsc = Fapb / 2)"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Bypass)
    }
}
#[doc = "Field `RESULT` reader - Count frequency divider"]
pub type ResultR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Divider start value in the normal mode / Step of divider in the fractional mode"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - Mode setting of the frequency divider"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Count frequency divider"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Divider start value in the normal mode / Step of divider in the fractional mode"]
    #[inline(always)]
    pub fn step(&mut self) -> StepW<'_, FdrSpec> {
        StepW::new(self, 0)
    }
    #[doc = "Bits 14:15 - Mode setting of the frequency divider"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, FdrSpec> {
        ModeW::new(self, 14)
    }
}
#[doc = "Fractional divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdrSpec;
impl crate::RegisterSpec for FdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FdrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FdrSpec {}
