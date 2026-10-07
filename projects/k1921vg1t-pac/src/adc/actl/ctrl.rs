#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ADCEN` reader - Enable ADC module"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - Enable ADC module"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRDY` reader - ADC ready for conversions"]
pub type AdcrdyR = crate::BitReader;
#[doc = "Field `ADCENLDO` reader - Enable ADC LDO"]
pub type AdcenldoR = crate::BitReader;
#[doc = "Field `ADCENLDO` writer - Enable ADC LDO"]
pub type AdcenldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSTART` reader - Start ADC module"]
pub type AdcstartR = crate::BitReader;
#[doc = "Field `ADCSTART` writer - Start ADC module"]
pub type AdcstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC resolution select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selres {
    #[doc = "0: 6 bit mode"]
    _6bit = 0,
    #[doc = "1: 8 bit mode"]
    _8bit = 1,
    #[doc = "2: 10 bit mode"]
    _10bit = 2,
    #[doc = "3: 12 bit mode"]
    _12bit = 3,
}
impl From<Selres> for u8 {
    #[inline(always)]
    fn from(variant: Selres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selres {
    type Ux = u8;
}
impl crate::IsEnum for Selres {}
#[doc = "Field `SELRES` reader - ADC resolution select"]
pub type SelresR = crate::FieldReader<Selres>;
impl SelresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selres {
        match self.bits {
            0 => Selres::_6bit,
            1 => Selres::_8bit,
            2 => Selres::_10bit,
            3 => Selres::_12bit,
            _ => unreachable!(),
        }
    }
    #[doc = "6 bit mode"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Selres::_6bit
    }
    #[doc = "8 bit mode"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Selres::_8bit
    }
    #[doc = "10 bit mode"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Selres::_10bit
    }
    #[doc = "12 bit mode"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Selres::_12bit
    }
}
#[doc = "Field `SELRES` writer - ADC resolution select"]
pub type SelresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Selres, crate::Safe>;
impl<'a, REG> SelresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "6 bit mode"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Selres::_6bit)
    }
    #[doc = "8 bit mode"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Selres::_8bit)
    }
    #[doc = "10 bit mode"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Selres::_10bit)
    }
    #[doc = "12 bit mode"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Selres::_12bit)
    }
}
#[doc = "Field `CALEN` reader - Enable ADC internal calibration"]
pub type CalenR = crate::BitReader;
#[doc = "Field `CALEN` writer - Enable ADC internal calibration"]
pub type CalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALLOAD` reader - Load value to ADC internal calibration scheme"]
pub type CalloadR = crate::BitReader;
#[doc = "Field `CALLOAD` writer - Load value to ADC internal calibration scheme"]
pub type CalloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSTART` reader - Start ADC internal calibration"]
pub type CalstartR = crate::BitReader;
#[doc = "Field `CALSTART` writer - Start ADC internal calibration"]
pub type CalstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALBUSY` reader - ADC internal calibration is in progress"]
pub type CalbusyR = crate::BitReader;
#[doc = "Field `RANGELDO` reader - Select LDO range"]
pub type RangeldoR = crate::BitReader;
#[doc = "Field `RANGELDO` writer - Select LDO range"]
pub type RangeldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISLVL` reader - Disable level-shifters"]
pub type DislvlR = crate::BitReader;
#[doc = "Field `DISLVL` writer - Disable level-shifters"]
pub type DislvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALIN` reader - ADC calibration input value"]
pub type CalinR = crate::FieldReader;
#[doc = "Field `CALIN` writer - ADC calibration input value"]
pub type CalinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CALOUT` reader - ADC calibration output value"]
pub type CaloutR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable ADC module"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC ready for conversions"]
    #[inline(always)]
    pub fn adcrdy(&self) -> AdcrdyR {
        AdcrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable ADC LDO"]
    #[inline(always)]
    pub fn adcenldo(&self) -> AdcenldoR {
        AdcenldoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start ADC module"]
    #[inline(always)]
    pub fn adcstart(&self) -> AdcstartR {
        AdcstartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ADC resolution select"]
    #[inline(always)]
    pub fn selres(&self) -> SelresR {
        SelresR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable ADC internal calibration"]
    #[inline(always)]
    pub fn calen(&self) -> CalenR {
        CalenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Load value to ADC internal calibration scheme"]
    #[inline(always)]
    pub fn calload(&self) -> CalloadR {
        CalloadR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start ADC internal calibration"]
    #[inline(always)]
    pub fn calstart(&self) -> CalstartR {
        CalstartR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC internal calibration is in progress"]
    #[inline(always)]
    pub fn calbusy(&self) -> CalbusyR {
        CalbusyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select LDO range"]
    #[inline(always)]
    pub fn rangeldo(&self) -> RangeldoR {
        RangeldoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable level-shifters"]
    #[inline(always)]
    pub fn dislvl(&self) -> DislvlR {
        DislvlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:22 - ADC calibration input value"]
    #[inline(always)]
    pub fn calin(&self) -> CalinR {
        CalinR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - ADC calibration output value"]
    #[inline(always)]
    pub fn calout(&self) -> CaloutR {
        CaloutR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ADC module"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<'_, CtrlSpec> {
        AdcenW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable ADC LDO"]
    #[inline(always)]
    pub fn adcenldo(&mut self) -> AdcenldoW<'_, CtrlSpec> {
        AdcenldoW::new(self, 2)
    }
    #[doc = "Bit 3 - Start ADC module"]
    #[inline(always)]
    pub fn adcstart(&mut self) -> AdcstartW<'_, CtrlSpec> {
        AdcstartW::new(self, 3)
    }
    #[doc = "Bits 4:5 - ADC resolution select"]
    #[inline(always)]
    pub fn selres(&mut self) -> SelresW<'_, CtrlSpec> {
        SelresW::new(self, 4)
    }
    #[doc = "Bit 8 - Enable ADC internal calibration"]
    #[inline(always)]
    pub fn calen(&mut self) -> CalenW<'_, CtrlSpec> {
        CalenW::new(self, 8)
    }
    #[doc = "Bit 9 - Load value to ADC internal calibration scheme"]
    #[inline(always)]
    pub fn calload(&mut self) -> CalloadW<'_, CtrlSpec> {
        CalloadW::new(self, 9)
    }
    #[doc = "Bit 10 - Start ADC internal calibration"]
    #[inline(always)]
    pub fn calstart(&mut self) -> CalstartW<'_, CtrlSpec> {
        CalstartW::new(self, 10)
    }
    #[doc = "Bit 12 - Select LDO range"]
    #[inline(always)]
    pub fn rangeldo(&mut self) -> RangeldoW<'_, CtrlSpec> {
        RangeldoW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable level-shifters"]
    #[inline(always)]
    pub fn dislvl(&mut self) -> DislvlW<'_, CtrlSpec> {
        DislvlW::new(self, 13)
    }
    #[doc = "Bits 16:22 - ADC calibration input value"]
    #[inline(always)]
    pub fn calin(&mut self) -> CalinW<'_, CtrlSpec> {
        CalinW::new(self, 16)
    }
}
#[doc = "ADC module control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x3000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x3000;
}
