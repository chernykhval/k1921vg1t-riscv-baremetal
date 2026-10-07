#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `DISABLE` reader - Disable DAC block"]
pub type DisableR = crate::BitReader;
#[doc = "Field `DISABLE` writer - Disable DAC block"]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Use external reference voltage source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtRef {
    #[doc = "0: Use internal reference volatage source"]
    InternalRef = 0,
    #[doc = "1: Use external 3.3 V reference volatage source"]
    ExternalRef = 1,
}
impl From<ExtRef> for bool {
    #[inline(always)]
    fn from(variant: ExtRef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_REF` reader - Use external reference voltage source"]
pub type ExtRefR = crate::BitReader<ExtRef>;
impl ExtRefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ExtRef {
        match self.bits {
            false => ExtRef::InternalRef,
            true => ExtRef::ExternalRef,
        }
    }
    #[doc = "Use internal reference volatage source"]
    #[inline(always)]
    pub fn is_internal_ref(&self) -> bool {
        *self == ExtRef::InternalRef
    }
    #[doc = "Use external 3.3 V reference volatage source"]
    #[inline(always)]
    pub fn is_external_ref(&self) -> bool {
        *self == ExtRef::ExternalRef
    }
}
#[doc = "Field `EXT_REF` writer - Use external reference voltage source"]
pub type ExtRefW<'a, REG> = crate::BitWriter<'a, REG, ExtRef>;
impl<'a, REG> ExtRefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use internal reference volatage source"]
    #[inline(always)]
    pub fn internal_ref(self) -> &'a mut crate::W<REG> {
        self.variant(ExtRef::InternalRef)
    }
    #[doc = "Use external 3.3 V reference volatage source"]
    #[inline(always)]
    pub fn external_ref(self) -> &'a mut crate::W<REG> {
        self.variant(ExtRef::ExternalRef)
    }
}
#[doc = "Field `BUF_EN` reader - Enable output buffer"]
pub type BufEnR = crate::BitReader;
#[doc = "Field `BUF_EN` writer - Enable output buffer"]
pub type BufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUNT` reader - Enable shunt resistor (test use only)"]
pub type ShuntR = crate::BitReader;
#[doc = "Field `SHUNT` writer - Enable shunt resistor (test use only)"]
pub type ShuntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select DAC trigger event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsel {
    #[doc = "0: software request by SWTRIG bit"]
    Swtrig = 0,
    #[doc = "1: hardware request from internal trigger generator"]
    Hwtrig = 1,
    #[doc = "2: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 2,
    #[doc = "3: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 3,
    #[doc = "4: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 4,
    #[doc = "5: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 5,
    #[doc = "6: GPIO A or GPIO B interrupt"]
    Gpioab = 6,
    #[doc = "7: GPIO C or GPIO D interrupt"]
    Gpiocd = 7,
    #[doc = "8: GPIO E or GPIO F interrupt"]
    Gpioef = 8,
    #[doc = "9: GPIO G interrupt"]
    Gpiog = 9,
    #[doc = "10: Combined all 16 A channels of PWMs"]
    Pwmagrp = 10,
    #[doc = "11: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 11,
    #[doc = "12: Reserved"]
    Reserved0 = 12,
    #[doc = "13: Reserved"]
    Reserved1 = 13,
    #[doc = "14: Reserved"]
    Reserved2 = 14,
    #[doc = "15: Reserved"]
    Reserved3 = 15,
}
impl From<Tsel> for u8 {
    #[inline(always)]
    fn from(variant: Tsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsel {
    type Ux = u8;
}
impl crate::IsEnum for Tsel {}
#[doc = "Field `TSEL` reader - Select DAC trigger event"]
pub type TselR = crate::FieldReader<Tsel>;
impl TselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsel {
        match self.bits {
            0 => Tsel::Swtrig,
            1 => Tsel::Hwtrig,
            2 => Tsel::Tmr0123,
            3 => Tsel::Tmr4567,
            4 => Tsel::Tmr89ab,
            5 => Tsel::Tmrcdef,
            6 => Tsel::Gpioab,
            7 => Tsel::Gpiocd,
            8 => Tsel::Gpioef,
            9 => Tsel::Gpiog,
            10 => Tsel::Pwmagrp,
            11 => Tsel::Pwmbgrp,
            12 => Tsel::Reserved0,
            13 => Tsel::Reserved1,
            14 => Tsel::Reserved2,
            15 => Tsel::Reserved3,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by SWTRIG bit"]
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == Tsel::Swtrig
    }
    #[doc = "hardware request from internal trigger generator"]
    #[inline(always)]
    pub fn is_hwtrig(&self) -> bool {
        *self == Tsel::Hwtrig
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Tsel::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Tsel::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Tsel::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Tsel::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Tsel::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Tsel::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Tsel::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Tsel::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Tsel::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Tsel::Pwmbgrp
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Tsel::Reserved0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Tsel::Reserved1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Tsel::Reserved2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Tsel::Reserved3
    }
}
#[doc = "Field `TSEL` writer - Select DAC trigger event"]
pub type TselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tsel, crate::Safe>;
impl<'a, REG> TselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by SWTRIG bit"]
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Swtrig)
    }
    #[doc = "hardware request from internal trigger generator"]
    #[inline(always)]
    pub fn hwtrig(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Hwtrig)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Pwmbgrp)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Reserved0)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Reserved1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Reserved2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel::Reserved3)
    }
}
#[doc = "DAC channel wave generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wave {
    #[doc = "0: Wave generation disabled"]
    None = 0,
    #[doc = "1: Noise wave generation enable"]
    Noise = 1,
    #[doc = "2: Triangular wave generation enable"]
    Triangular = 2,
    #[doc = "3: Pattern generator enable"]
    Pattern = 3,
}
impl From<Wave> for u8 {
    #[inline(always)]
    fn from(variant: Wave) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wave {
    type Ux = u8;
}
impl crate::IsEnum for Wave {}
#[doc = "Field `WAVE` reader - DAC channel wave generation enable"]
pub type WaveR = crate::FieldReader<Wave>;
impl WaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wave {
        match self.bits {
            0 => Wave::None,
            1 => Wave::Noise,
            2 => Wave::Triangular,
            3 => Wave::Pattern,
            _ => unreachable!(),
        }
    }
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Wave::None
    }
    #[doc = "Noise wave generation enable"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == Wave::Noise
    }
    #[doc = "Triangular wave generation enable"]
    #[inline(always)]
    pub fn is_triangular(&self) -> bool {
        *self == Wave::Triangular
    }
    #[doc = "Pattern generator enable"]
    #[inline(always)]
    pub fn is_pattern(&self) -> bool {
        *self == Wave::Pattern
    }
}
#[doc = "Field `WAVE` writer - DAC channel wave generation enable"]
pub type WaveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wave, crate::Safe>;
impl<'a, REG> WaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Wave::None)
    }
    #[doc = "Noise wave generation enable"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut crate::W<REG> {
        self.variant(Wave::Noise)
    }
    #[doc = "Triangular wave generation enable"]
    #[inline(always)]
    pub fn triangular(self) -> &'a mut crate::W<REG> {
        self.variant(Wave::Triangular)
    }
    #[doc = "Pattern generator enable"]
    #[inline(always)]
    pub fn pattern(self) -> &'a mut crate::W<REG> {
        self.variant(Wave::Pattern)
    }
}
#[doc = "Switch between independent and simultaneous DAC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datamode {
    #[doc = "0: Independent DAC mode"]
    Independent = 0,
    #[doc = "1: Simultaneous DAC mode"]
    Simultaneous = 1,
}
impl From<Datamode> for bool {
    #[inline(always)]
    fn from(variant: Datamode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAMODE` reader - Switch between independent and simultaneous DAC mode"]
pub type DatamodeR = crate::BitReader<Datamode>;
impl DatamodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datamode {
        match self.bits {
            false => Datamode::Independent,
            true => Datamode::Simultaneous,
        }
    }
    #[doc = "Independent DAC mode"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == Datamode::Independent
    }
    #[doc = "Simultaneous DAC mode"]
    #[inline(always)]
    pub fn is_simultaneous(&self) -> bool {
        *self == Datamode::Simultaneous
    }
}
#[doc = "Field `DATAMODE` writer - Switch between independent and simultaneous DAC mode"]
pub type DatamodeW<'a, REG> = crate::BitWriter<'a, REG, Datamode>;
impl<'a, REG> DatamodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Independent DAC mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(Datamode::Independent)
    }
    #[doc = "Simultaneous DAC mode"]
    #[inline(always)]
    pub fn simultaneous(self) -> &'a mut crate::W<REG> {
        self.variant(Datamode::Simultaneous)
    }
}
#[doc = "Signed DAC mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    #[doc = "0: Data treated as unsigned value"]
    Unsigned = 0,
    #[doc = "1: Data treated as signed values"]
    Signed = 1,
}
impl From<Sign> for bool {
    #[inline(always)]
    fn from(variant: Sign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGN` reader - Signed DAC mode enable"]
pub type SignR = crate::BitReader<Sign>;
impl SignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sign {
        match self.bits {
            false => Sign::Unsigned,
            true => Sign::Signed,
        }
    }
    #[doc = "Data treated as unsigned value"]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == Sign::Unsigned
    }
    #[doc = "Data treated as signed values"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == Sign::Signed
    }
}
#[doc = "Field `SIGN` writer - Signed DAC mode enable"]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG, Sign>;
impl<'a, REG> SignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data treated as unsigned value"]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(Sign::Unsigned)
    }
    #[doc = "Data treated as signed values"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut crate::W<REG> {
        self.variant(Sign::Signed)
    }
}
#[doc = "Field `MASK` reader - DAC channel mask/amplitude selector"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - DAC channel mask/amplitude selector"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Scale value for up/downscale input data in steps by power of 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scale {
    #[doc = "0: multiplication/division by 1"]
    Noshift = 0,
    #[doc = "1: multiplication/division by 2"]
    Shift1 = 1,
    #[doc = "2: multiplication/division by 4"]
    Shift2 = 2,
    #[doc = "3: multiplication/division by 8"]
    Shift3 = 3,
    #[doc = "4: multiplication/division by 16"]
    Shift4 = 4,
    #[doc = "5: multiplication/division by 32"]
    Shift5 = 5,
    #[doc = "6: multiplication/division by 64"]
    Shift6 = 6,
    #[doc = "7: multiplication/division by 128"]
    Shift7 = 7,
}
impl From<Scale> for u8 {
    #[inline(always)]
    fn from(variant: Scale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scale {
    type Ux = u8;
}
impl crate::IsEnum for Scale {}
#[doc = "Field `SCALE` reader - Scale value for up/downscale input data in steps by power of 2"]
pub type ScaleR = crate::FieldReader<Scale>;
impl ScaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scale {
        match self.bits {
            0 => Scale::Noshift,
            1 => Scale::Shift1,
            2 => Scale::Shift2,
            3 => Scale::Shift3,
            4 => Scale::Shift4,
            5 => Scale::Shift5,
            6 => Scale::Shift6,
            7 => Scale::Shift7,
            _ => unreachable!(),
        }
    }
    #[doc = "multiplication/division by 1"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == Scale::Noshift
    }
    #[doc = "multiplication/division by 2"]
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == Scale::Shift1
    }
    #[doc = "multiplication/division by 4"]
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == Scale::Shift2
    }
    #[doc = "multiplication/division by 8"]
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == Scale::Shift3
    }
    #[doc = "multiplication/division by 16"]
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == Scale::Shift4
    }
    #[doc = "multiplication/division by 32"]
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == Scale::Shift5
    }
    #[doc = "multiplication/division by 64"]
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == Scale::Shift6
    }
    #[doc = "multiplication/division by 128"]
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == Scale::Shift7
    }
}
#[doc = "Field `SCALE` writer - Scale value for up/downscale input data in steps by power of 2"]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 3, Scale, crate::Safe>;
impl<'a, REG> ScaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "multiplication/division by 1"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Noshift)
    }
    #[doc = "multiplication/division by 2"]
    #[inline(always)]
    pub fn shift1(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Shift1)
    }
    #[doc = "multiplication/division by 4"]
    #[inline(always)]
    pub fn shift2(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Shift2)
    }
    #[doc = "multiplication/division by 8"]
    #[inline(always)]
    pub fn shift3(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Shift3)
    }
    #[doc = "multiplication/division by 16"]
    #[inline(always)]
    pub fn shift4(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Shift4)
    }
    #[doc = "multiplication/division by 32"]
    #[inline(always)]
    pub fn shift5(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Shift5)
    }
    #[doc = "multiplication/division by 64"]
    #[inline(always)]
    pub fn shift6(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Shift6)
    }
    #[doc = "multiplication/division by 128"]
    #[inline(always)]
    pub fn shift7(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Shift7)
    }
}
#[doc = "Switch between up and downscale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Muldiv {
    #[doc = "0: division (shift SCALE positions to the right)"]
    Downscale = 0,
    #[doc = "1: multiplication (shift SCALE positions to the left)"]
    Upscale = 1,
}
impl From<Muldiv> for bool {
    #[inline(always)]
    fn from(variant: Muldiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULDIV` reader - Switch between up and downscale"]
pub type MuldivR = crate::BitReader<Muldiv>;
impl MuldivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Muldiv {
        match self.bits {
            false => Muldiv::Downscale,
            true => Muldiv::Upscale,
        }
    }
    #[doc = "division (shift SCALE positions to the right)"]
    #[inline(always)]
    pub fn is_downscale(&self) -> bool {
        *self == Muldiv::Downscale
    }
    #[doc = "multiplication (shift SCALE positions to the left)"]
    #[inline(always)]
    pub fn is_upscale(&self) -> bool {
        *self == Muldiv::Upscale
    }
}
#[doc = "Field `MULDIV` writer - Switch between up and downscale"]
pub type MuldivW<'a, REG> = crate::BitWriter<'a, REG, Muldiv>;
impl<'a, REG> MuldivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "division (shift SCALE positions to the right)"]
    #[inline(always)]
    pub fn downscale(self) -> &'a mut crate::W<REG> {
        self.variant(Muldiv::Downscale)
    }
    #[doc = "multiplication (shift SCALE positions to the left)"]
    #[inline(always)]
    pub fn upscale(self) -> &'a mut crate::W<REG> {
        self.variant(Muldiv::Upscale)
    }
}
#[doc = "Field `NEGATE` reader - Convert data to it's two's complement value"]
pub type NegateR = crate::BitReader;
#[doc = "Field `NEGATE` writer - Convert data to it's two's complement value"]
pub type NegateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET` reader - Offset for pattern generator"]
pub type OffsetR = crate::FieldReader;
#[doc = "Field `OFFSET` writer - Offset for pattern generator"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Disable DAC block"]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use external reference voltage source"]
    #[inline(always)]
    pub fn ext_ref(&self) -> ExtRefR {
        ExtRefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable output buffer"]
    #[inline(always)]
    pub fn buf_en(&self) -> BufEnR {
        BufEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable shunt resistor (test use only)"]
    #[inline(always)]
    pub fn shunt(&self) -> ShuntR {
        ShuntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Select DAC trigger event"]
    #[inline(always)]
    pub fn tsel(&self) -> TselR {
        TselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - DAC channel wave generation enable"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Switch between independent and simultaneous DAC mode"]
    #[inline(always)]
    pub fn datamode(&self) -> DatamodeR {
        DatamodeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Signed DAC mode enable"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - DAC channel mask/amplitude selector"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Scale value for up/downscale input data in steps by power of 2"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Switch between up and downscale"]
    #[inline(always)]
    pub fn muldiv(&self) -> MuldivR {
        MuldivR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Convert data to it's two's complement value"]
    #[inline(always)]
    pub fn negate(&self) -> NegateR {
        NegateR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Offset for pattern generator"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable DAC block"]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<'_, ControlSpec> {
        DisableW::new(self, 0)
    }
    #[doc = "Bit 1 - Use external reference voltage source"]
    #[inline(always)]
    pub fn ext_ref(&mut self) -> ExtRefW<'_, ControlSpec> {
        ExtRefW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable output buffer"]
    #[inline(always)]
    pub fn buf_en(&mut self) -> BufEnW<'_, ControlSpec> {
        BufEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable shunt resistor (test use only)"]
    #[inline(always)]
    pub fn shunt(&mut self) -> ShuntW<'_, ControlSpec> {
        ShuntW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Select DAC trigger event"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TselW<'_, ControlSpec> {
        TselW::new(self, 4)
    }
    #[doc = "Bits 8:9 - DAC channel wave generation enable"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<'_, ControlSpec> {
        WaveW::new(self, 8)
    }
    #[doc = "Bit 10 - Switch between independent and simultaneous DAC mode"]
    #[inline(always)]
    pub fn datamode(&mut self) -> DatamodeW<'_, ControlSpec> {
        DatamodeW::new(self, 10)
    }
    #[doc = "Bit 11 - Signed DAC mode enable"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, ControlSpec> {
        SignW::new(self, 11)
    }
    #[doc = "Bits 12:15 - DAC channel mask/amplitude selector"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, ControlSpec> {
        MaskW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Scale value for up/downscale input data in steps by power of 2"]
    #[inline(always)]
    pub fn scale(&mut self) -> ScaleW<'_, ControlSpec> {
        ScaleW::new(self, 16)
    }
    #[doc = "Bit 19 - Switch between up and downscale"]
    #[inline(always)]
    pub fn muldiv(&mut self) -> MuldivW<'_, ControlSpec> {
        MuldivW::new(self, 19)
    }
    #[doc = "Bit 20 - Convert data to it's two's complement value"]
    #[inline(always)]
    pub fn negate(&mut self) -> NegateW<'_, ControlSpec> {
        NegateW::new(self, 20)
    }
    #[doc = "Bits 24:31 - Offset for pattern generator"]
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<'_, ControlSpec> {
        OffsetW::new(self, 24)
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
#[doc = "`reset()` method sets CONTROL to value 0x01"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x01;
}
