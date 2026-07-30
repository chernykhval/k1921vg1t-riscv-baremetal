#[doc = "Register `D_CTRL` reader"]
pub type R = crate::R<DCtrlSpec>;
#[doc = "Register `D_CTRL` writer"]
pub type W = crate::W<DCtrlSpec>;
#[doc = "Field `CBN` reader - Command Bits Number"]
pub type CbnR = crate::FieldReader;
#[doc = "Field `CBN` writer - Command Bits Number"]
pub type CbnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DLBN` reader - Data Low Bits Number"]
pub type DlbnR = crate::FieldReader;
#[doc = "Field `DLBN` writer - Data Low Bits Number"]
pub type DlbnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DHBN` reader - Data High Bits Number"]
pub type DhbnR = crate::FieldReader;
#[doc = "Field `DHBN` writer - Data High Bits Number"]
pub type DhbnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "transmission Mode (0 - Triggered; 1 - Data repetition)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Triggered mode selected"]
    Triggered = 0,
    #[doc = "1: Data repetition mode selected"]
    Datarepetition = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - transmission Mode (0 - Triggered; 1 - Data repetition)"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Triggered,
            true => Mode::Datarepetition,
        }
    }
    #[doc = "Triggered mode selected"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == Mode::Triggered
    }
    #[doc = "Data repetition mode selected"]
    #[inline(always)]
    pub fn is_datarepetition(&self) -> bool {
        *self == Mode::Datarepetition
    }
}
#[doc = "Field `MODE` writer - transmission Mode (0 - Triggered; 1 - Data repetition)"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Triggered mode selected"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Triggered)
    }
    #[doc = "Data repetition mode selected"]
    #[inline(always)]
    pub fn datarepetition(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Datarepetition)
    }
}
#[doc = "Field `FSB` reader - First Selection Bit (data/time frame)"]
pub type FsbR = crate::BitReader;
#[doc = "Field `FSB` writer - First Selection Bit (data/time frame)"]
pub type FsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSB` reader - Second Selection Bit (data/time frame)"]
pub type SsbR = crate::BitReader;
#[doc = "Field `SSB` writer - Second Selection Bit (data/time frame)"]
pub type SsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPL` reader - Passive Phase Length (data/time frame)"]
pub type PplR = crate::FieldReader;
#[doc = "Field `PPL` writer - Passive Phase Length (data/time frame)"]
pub type PplW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TFN` reader - passive Time Frames Number"]
pub type TfnR = crate::FieldReader;
#[doc = "Field `TFN` writer - passive Time Frames Number"]
pub type TfnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Command Bits Number"]
    #[inline(always)]
    pub fn cbn(&self) -> CbnR {
        CbnR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - Data Low Bits Number"]
    #[inline(always)]
    pub fn dlbn(&self) -> DlbnR {
        DlbnR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Data High Bits Number"]
    #[inline(always)]
    pub fn dhbn(&self) -> DhbnR {
        DhbnR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - transmission Mode (0 - Triggered; 1 - Data repetition)"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - First Selection Bit (data/time frame)"]
    #[inline(always)]
    pub fn fsb(&self) -> FsbR {
        FsbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Second Selection Bit (data/time frame)"]
    #[inline(always)]
    pub fn ssb(&self) -> SsbR {
        SsbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Passive Phase Length (data/time frame)"]
    #[inline(always)]
    pub fn ppl(&self) -> PplR {
        PplR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - passive Time Frames Number"]
    #[inline(always)]
    pub fn tfn(&self) -> TfnR {
        TfnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command Bits Number"]
    #[inline(always)]
    pub fn cbn(&mut self) -> CbnW<'_, DCtrlSpec> {
        CbnW::new(self, 0)
    }
    #[doc = "Bits 6:10 - Data Low Bits Number"]
    #[inline(always)]
    pub fn dlbn(&mut self) -> DlbnW<'_, DCtrlSpec> {
        DlbnW::new(self, 6)
    }
    #[doc = "Bits 11:15 - Data High Bits Number"]
    #[inline(always)]
    pub fn dhbn(&mut self) -> DhbnW<'_, DCtrlSpec> {
        DhbnW::new(self, 11)
    }
    #[doc = "Bit 16 - transmission Mode (0 - Triggered; 1 - Data repetition)"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, DCtrlSpec> {
        ModeW::new(self, 16)
    }
    #[doc = "Bit 17 - First Selection Bit (data/time frame)"]
    #[inline(always)]
    pub fn fsb(&mut self) -> FsbW<'_, DCtrlSpec> {
        FsbW::new(self, 17)
    }
    #[doc = "Bit 18 - Second Selection Bit (data/time frame)"]
    #[inline(always)]
    pub fn ssb(&mut self) -> SsbW<'_, DCtrlSpec> {
        SsbW::new(self, 18)
    }
    #[doc = "Bits 19:23 - Passive Phase Length (data/time frame)"]
    #[inline(always)]
    pub fn ppl(&mut self) -> PplW<'_, DCtrlSpec> {
        PplW::new(self, 19)
    }
    #[doc = "Bits 24:27 - passive Time Frames Number"]
    #[inline(always)]
    pub fn tfn(&mut self) -> TfnW<'_, DCtrlSpec> {
        TfnW::new(self, 24)
    }
}
#[doc = "Downstream Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCtrlSpec;
impl crate::RegisterSpec for DCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_ctrl::R`](R) reader structure"]
impl crate::Readable for DCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`d_ctrl::W`](W) writer structure"]
impl crate::Writable for DCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_CTRL to value 0"]
impl crate::Resettable for DCtrlSpec {}
