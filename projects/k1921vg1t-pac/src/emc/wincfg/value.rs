#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "8/16 bit mode. When 1 - 16 bit mode enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: 8 bit mode"]
    _8bit = 0,
    #[doc = "1: 16 bit mode"]
    _16bit = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - 8/16 bit mode. When 1 - 16 bit mode enabled"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::_8bit,
            true => Mode::_16bit,
        }
    }
    #[doc = "8 bit mode"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Mode::_8bit
    }
    #[doc = "16 bit mode"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Mode::_16bit
    }
}
#[doc = "Field `MODE` writer - 8/16 bit mode. When 1 - 16 bit mode enabled"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8 bit mode"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_8bit)
    }
    #[doc = "16 bit mode"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_16bit)
    }
}
#[doc = "Field `RDCYC` reader - Additional Read cycles"]
pub type RdcycR = crate::FieldReader;
#[doc = "Field `RDCYC` writer - Additional Read cycles"]
pub type RdcycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRCYC` reader - Additional Write cycles"]
pub type WrcycR = crate::FieldReader;
#[doc = "Field `WRCYC` writer - Additional Write cycles"]
pub type WrcycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TACYC` reader - Turnaround cycles"]
pub type TacycR = crate::FieldReader;
#[doc = "Field `TACYC` writer - Turnaround cycles"]
pub type TacycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 8/16 bit mode. When 1 - 16 bit mode enabled"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Additional Read cycles"]
    #[inline(always)]
    pub fn rdcyc(&self) -> RdcycR {
        RdcycR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Additional Write cycles"]
    #[inline(always)]
    pub fn wrcyc(&self) -> WrcycR {
        WrcycR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Turnaround cycles"]
    #[inline(always)]
    pub fn tacyc(&self) -> TacycR {
        TacycR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 8/16 bit mode. When 1 - 16 bit mode enabled"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ValueSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Additional Read cycles"]
    #[inline(always)]
    pub fn rdcyc(&mut self) -> RdcycW<'_, ValueSpec> {
        RdcycW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Additional Write cycles"]
    #[inline(always)]
    pub fn wrcyc(&mut self) -> WrcycW<'_, ValueSpec> {
        WrcycW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Turnaround cycles"]
    #[inline(always)]
    pub fn tacyc(&mut self) -> TacycW<'_, ValueSpec> {
        TacycW::new(self, 24)
    }
}
#[doc = "Window Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALUE to value 0x0707_0701"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0x0707_0701;
}
