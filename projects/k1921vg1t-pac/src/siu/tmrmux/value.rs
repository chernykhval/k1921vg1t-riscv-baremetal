#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Select signal for TMRxx CCIA 0 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccia0 {
    #[doc = "0: External pin"]
    Tmrio0 = 0,
    #[doc = "1: External pin"]
    Tmrio1 = 1,
    #[doc = "2: External pin"]
    Tmrio2 = 2,
    #[doc = "3: External pin"]
    Tmrio3 = 3,
    #[doc = "4: GPIOA or GPIOB adc soc impulse"]
    Gpioab = 4,
    #[doc = "5: GPIOC or GPIOD adc soc impulse"]
    Gpiocd = 5,
    #[doc = "6: GPIOE or GPIOF adc soc impulse"]
    Gpioef = 6,
    #[doc = "7: GPIOG adc soc impulse"]
    Gpiog = 7,
    #[doc = "8: PWM0-3 adc soc channel A impulse"]
    Pwm0_3a = 8,
    #[doc = "9: PWM4-7 adc soc channel A impulse"]
    Pwm4_7a = 9,
    #[doc = "10: PWM8-11 adc soc channel A impulse"]
    Pwm8_11a = 10,
    #[doc = "11: PWM12-15 adc soc channel A impulse"]
    Pwm12_15a = 11,
    #[doc = "12: PWM0-3 adc soc channel B impulse"]
    Pwm0_3b = 12,
    #[doc = "13: PWM4-7 adc soc channel B impulse"]
    Pwm4_7b = 13,
    #[doc = "14: PWM8-11 adc soc channel B impulse"]
    Pwm8_11b = 14,
    #[doc = "15: PWM12-15 adc soc channel B impulse"]
    Pwm12_15b = 15,
}
impl From<Ccia0> for u8 {
    #[inline(always)]
    fn from(variant: Ccia0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccia0 {
    type Ux = u8;
}
impl crate::IsEnum for Ccia0 {}
#[doc = "Field `CCIA_0` reader - Select signal for TMRxx CCIA 0 input"]
pub type Ccia0R = crate::FieldReader<Ccia0>;
impl Ccia0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccia0 {
        match self.bits {
            0 => Ccia0::Tmrio0,
            1 => Ccia0::Tmrio1,
            2 => Ccia0::Tmrio2,
            3 => Ccia0::Tmrio3,
            4 => Ccia0::Gpioab,
            5 => Ccia0::Gpiocd,
            6 => Ccia0::Gpioef,
            7 => Ccia0::Gpiog,
            8 => Ccia0::Pwm0_3a,
            9 => Ccia0::Pwm4_7a,
            10 => Ccia0::Pwm8_11a,
            11 => Ccia0::Pwm12_15a,
            12 => Ccia0::Pwm0_3b,
            13 => Ccia0::Pwm4_7b,
            14 => Ccia0::Pwm8_11b,
            15 => Ccia0::Pwm12_15b,
            _ => unreachable!(),
        }
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio0(&self) -> bool {
        *self == Ccia0::Tmrio0
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio1(&self) -> bool {
        *self == Ccia0::Tmrio1
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio2(&self) -> bool {
        *self == Ccia0::Tmrio2
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio3(&self) -> bool {
        *self == Ccia0::Tmrio3
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Ccia0::Gpioab
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Ccia0::Gpiocd
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Ccia0::Gpioef
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Ccia0::Gpiog
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm0_3a(&self) -> bool {
        *self == Ccia0::Pwm0_3a
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm4_7a(&self) -> bool {
        *self == Ccia0::Pwm4_7a
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm8_11a(&self) -> bool {
        *self == Ccia0::Pwm8_11a
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm12_15a(&self) -> bool {
        *self == Ccia0::Pwm12_15a
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm0_3b(&self) -> bool {
        *self == Ccia0::Pwm0_3b
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm4_7b(&self) -> bool {
        *self == Ccia0::Pwm4_7b
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm8_11b(&self) -> bool {
        *self == Ccia0::Pwm8_11b
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm12_15b(&self) -> bool {
        *self == Ccia0::Pwm12_15b
    }
}
#[doc = "Field `CCIA_0` writer - Select signal for TMRxx CCIA 0 input"]
pub type Ccia0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccia0, crate::Safe>;
impl<'a, REG> Ccia0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Tmrio0)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Tmrio1)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Tmrio2)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Tmrio3)
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Gpioab)
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Gpiocd)
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Gpioef)
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Gpiog)
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm0_3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Pwm0_3a)
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm4_7a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Pwm4_7a)
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm8_11a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Pwm8_11a)
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm12_15a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Pwm12_15a)
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm0_3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Pwm0_3b)
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm4_7b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Pwm4_7b)
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm8_11b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Pwm8_11b)
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm12_15b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia0::Pwm12_15b)
    }
}
#[doc = "Select signal for TMRxx CCIB 0 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccib0 {
    #[doc = "0: External pin"]
    Tmrio0 = 0,
    #[doc = "1: External pin"]
    Tmrio1 = 1,
    #[doc = "2: External pin"]
    Tmrio2 = 2,
    #[doc = "3: External pin"]
    Tmrio3 = 3,
    #[doc = "4: GPIOA or GPIOB adc soc impulse"]
    Gpioab = 4,
    #[doc = "5: GPIOC or GPIOD adc soc impulse"]
    Gpiocd = 5,
    #[doc = "6: GPIOE or GPIOF adc soc impulse"]
    Gpioef = 6,
    #[doc = "7: GPIOG adc soc impulse"]
    Gpiog = 7,
    #[doc = "8: PWM0-3 adc soc channel A impulse"]
    Pwm0_3a = 8,
    #[doc = "9: PWM4-7 adc soc channel A impulse"]
    Pwm4_7a = 9,
    #[doc = "10: PWM8-11 adc soc channel A impulse"]
    Pwm8_11a = 10,
    #[doc = "11: PWM12-15 adc soc channel A impulse"]
    Pwm12_15a = 11,
    #[doc = "12: PWM0-3 adc soc channel B impulse"]
    Pwm0_3b = 12,
    #[doc = "13: PWM4-7 adc soc channel B impulse"]
    Pwm4_7b = 13,
    #[doc = "14: PWM8-11 adc soc channel B impulse"]
    Pwm8_11b = 14,
    #[doc = "15: PWM12-15 adc soc channel B impulse"]
    Pwm12_15b = 15,
}
impl From<Ccib0> for u8 {
    #[inline(always)]
    fn from(variant: Ccib0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccib0 {
    type Ux = u8;
}
impl crate::IsEnum for Ccib0 {}
#[doc = "Field `CCIB_0` reader - Select signal for TMRxx CCIB 0 input"]
pub type Ccib0R = crate::FieldReader<Ccib0>;
impl Ccib0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccib0 {
        match self.bits {
            0 => Ccib0::Tmrio0,
            1 => Ccib0::Tmrio1,
            2 => Ccib0::Tmrio2,
            3 => Ccib0::Tmrio3,
            4 => Ccib0::Gpioab,
            5 => Ccib0::Gpiocd,
            6 => Ccib0::Gpioef,
            7 => Ccib0::Gpiog,
            8 => Ccib0::Pwm0_3a,
            9 => Ccib0::Pwm4_7a,
            10 => Ccib0::Pwm8_11a,
            11 => Ccib0::Pwm12_15a,
            12 => Ccib0::Pwm0_3b,
            13 => Ccib0::Pwm4_7b,
            14 => Ccib0::Pwm8_11b,
            15 => Ccib0::Pwm12_15b,
            _ => unreachable!(),
        }
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio0(&self) -> bool {
        *self == Ccib0::Tmrio0
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio1(&self) -> bool {
        *self == Ccib0::Tmrio1
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio2(&self) -> bool {
        *self == Ccib0::Tmrio2
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio3(&self) -> bool {
        *self == Ccib0::Tmrio3
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Ccib0::Gpioab
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Ccib0::Gpiocd
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Ccib0::Gpioef
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Ccib0::Gpiog
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm0_3a(&self) -> bool {
        *self == Ccib0::Pwm0_3a
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm4_7a(&self) -> bool {
        *self == Ccib0::Pwm4_7a
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm8_11a(&self) -> bool {
        *self == Ccib0::Pwm8_11a
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm12_15a(&self) -> bool {
        *self == Ccib0::Pwm12_15a
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm0_3b(&self) -> bool {
        *self == Ccib0::Pwm0_3b
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm4_7b(&self) -> bool {
        *self == Ccib0::Pwm4_7b
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm8_11b(&self) -> bool {
        *self == Ccib0::Pwm8_11b
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm12_15b(&self) -> bool {
        *self == Ccib0::Pwm12_15b
    }
}
#[doc = "Field `CCIB_0` writer - Select signal for TMRxx CCIB 0 input"]
pub type Ccib0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccib0, crate::Safe>;
impl<'a, REG> Ccib0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Tmrio0)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Tmrio1)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Tmrio2)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Tmrio3)
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Gpioab)
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Gpiocd)
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Gpioef)
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Gpiog)
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm0_3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Pwm0_3a)
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm4_7a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Pwm4_7a)
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm8_11a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Pwm8_11a)
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm12_15a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Pwm12_15a)
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm0_3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Pwm0_3b)
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm4_7b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Pwm4_7b)
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm8_11b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Pwm8_11b)
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm12_15b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib0::Pwm12_15b)
    }
}
#[doc = "Select signal for TMRxx CCIA 1 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccia1 {
    #[doc = "0: External pin"]
    Tmrio0 = 0,
    #[doc = "1: External pin"]
    Tmrio1 = 1,
    #[doc = "2: External pin"]
    Tmrio2 = 2,
    #[doc = "3: External pin"]
    Tmrio3 = 3,
    #[doc = "4: GPIOA or GPIOB adc soc impulse"]
    Gpioab = 4,
    #[doc = "5: GPIOC or GPIOD adc soc impulse"]
    Gpiocd = 5,
    #[doc = "6: GPIOE or GPIOF adc soc impulse"]
    Gpioef = 6,
    #[doc = "7: GPIOG adc soc impulse"]
    Gpiog = 7,
    #[doc = "8: PWM0-3 adc soc channel A impulse"]
    Pwm0_3a = 8,
    #[doc = "9: PWM4-7 adc soc channel A impulse"]
    Pwm4_7a = 9,
    #[doc = "10: PWM8-11 adc soc channel A impulse"]
    Pwm8_11a = 10,
    #[doc = "11: PWM12-15 adc soc channel A impulse"]
    Pwm12_15a = 11,
    #[doc = "12: PWM0-3 adc soc channel B impulse"]
    Pwm0_3b = 12,
    #[doc = "13: PWM4-7 adc soc channel B impulse"]
    Pwm4_7b = 13,
    #[doc = "14: PWM8-11 adc soc channel B impulse"]
    Pwm8_11b = 14,
    #[doc = "15: PWM12-15 adc soc channel B impulse"]
    Pwm12_15b = 15,
}
impl From<Ccia1> for u8 {
    #[inline(always)]
    fn from(variant: Ccia1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccia1 {
    type Ux = u8;
}
impl crate::IsEnum for Ccia1 {}
#[doc = "Field `CCIA_1` reader - Select signal for TMRxx CCIA 1 input"]
pub type Ccia1R = crate::FieldReader<Ccia1>;
impl Ccia1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccia1 {
        match self.bits {
            0 => Ccia1::Tmrio0,
            1 => Ccia1::Tmrio1,
            2 => Ccia1::Tmrio2,
            3 => Ccia1::Tmrio3,
            4 => Ccia1::Gpioab,
            5 => Ccia1::Gpiocd,
            6 => Ccia1::Gpioef,
            7 => Ccia1::Gpiog,
            8 => Ccia1::Pwm0_3a,
            9 => Ccia1::Pwm4_7a,
            10 => Ccia1::Pwm8_11a,
            11 => Ccia1::Pwm12_15a,
            12 => Ccia1::Pwm0_3b,
            13 => Ccia1::Pwm4_7b,
            14 => Ccia1::Pwm8_11b,
            15 => Ccia1::Pwm12_15b,
            _ => unreachable!(),
        }
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio0(&self) -> bool {
        *self == Ccia1::Tmrio0
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio1(&self) -> bool {
        *self == Ccia1::Tmrio1
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio2(&self) -> bool {
        *self == Ccia1::Tmrio2
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio3(&self) -> bool {
        *self == Ccia1::Tmrio3
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Ccia1::Gpioab
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Ccia1::Gpiocd
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Ccia1::Gpioef
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Ccia1::Gpiog
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm0_3a(&self) -> bool {
        *self == Ccia1::Pwm0_3a
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm4_7a(&self) -> bool {
        *self == Ccia1::Pwm4_7a
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm8_11a(&self) -> bool {
        *self == Ccia1::Pwm8_11a
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm12_15a(&self) -> bool {
        *self == Ccia1::Pwm12_15a
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm0_3b(&self) -> bool {
        *self == Ccia1::Pwm0_3b
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm4_7b(&self) -> bool {
        *self == Ccia1::Pwm4_7b
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm8_11b(&self) -> bool {
        *self == Ccia1::Pwm8_11b
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm12_15b(&self) -> bool {
        *self == Ccia1::Pwm12_15b
    }
}
#[doc = "Field `CCIA_1` writer - Select signal for TMRxx CCIA 1 input"]
pub type Ccia1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccia1, crate::Safe>;
impl<'a, REG> Ccia1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Tmrio0)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Tmrio1)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Tmrio2)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Tmrio3)
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Gpioab)
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Gpiocd)
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Gpioef)
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Gpiog)
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm0_3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Pwm0_3a)
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm4_7a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Pwm4_7a)
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm8_11a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Pwm8_11a)
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm12_15a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Pwm12_15a)
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm0_3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Pwm0_3b)
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm4_7b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Pwm4_7b)
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm8_11b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Pwm8_11b)
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm12_15b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia1::Pwm12_15b)
    }
}
#[doc = "Select signal for TMRxx CCIB 1 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccib1 {
    #[doc = "0: External pin"]
    Tmrio0 = 0,
    #[doc = "1: External pin"]
    Tmrio1 = 1,
    #[doc = "2: External pin"]
    Tmrio2 = 2,
    #[doc = "3: External pin"]
    Tmrio3 = 3,
    #[doc = "4: GPIOA or GPIOB adc soc impulse"]
    Gpioab = 4,
    #[doc = "5: GPIOC or GPIOD adc soc impulse"]
    Gpiocd = 5,
    #[doc = "6: GPIOE or GPIOF adc soc impulse"]
    Gpioef = 6,
    #[doc = "7: GPIOG adc soc impulse"]
    Gpiog = 7,
    #[doc = "8: PWM0-3 adc soc channel A impulse"]
    Pwm0_3a = 8,
    #[doc = "9: PWM4-7 adc soc channel A impulse"]
    Pwm4_7a = 9,
    #[doc = "10: PWM8-11 adc soc channel A impulse"]
    Pwm8_11a = 10,
    #[doc = "11: PWM12-15 adc soc channel A impulse"]
    Pwm12_15a = 11,
    #[doc = "12: PWM0-3 adc soc channel B impulse"]
    Pwm0_3b = 12,
    #[doc = "13: PWM4-7 adc soc channel B impulse"]
    Pwm4_7b = 13,
    #[doc = "14: PWM8-11 adc soc channel B impulse"]
    Pwm8_11b = 14,
    #[doc = "15: PWM12-15 adc soc channel B impulse"]
    Pwm12_15b = 15,
}
impl From<Ccib1> for u8 {
    #[inline(always)]
    fn from(variant: Ccib1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccib1 {
    type Ux = u8;
}
impl crate::IsEnum for Ccib1 {}
#[doc = "Field `CCIB_1` reader - Select signal for TMRxx CCIB 1 input"]
pub type Ccib1R = crate::FieldReader<Ccib1>;
impl Ccib1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccib1 {
        match self.bits {
            0 => Ccib1::Tmrio0,
            1 => Ccib1::Tmrio1,
            2 => Ccib1::Tmrio2,
            3 => Ccib1::Tmrio3,
            4 => Ccib1::Gpioab,
            5 => Ccib1::Gpiocd,
            6 => Ccib1::Gpioef,
            7 => Ccib1::Gpiog,
            8 => Ccib1::Pwm0_3a,
            9 => Ccib1::Pwm4_7a,
            10 => Ccib1::Pwm8_11a,
            11 => Ccib1::Pwm12_15a,
            12 => Ccib1::Pwm0_3b,
            13 => Ccib1::Pwm4_7b,
            14 => Ccib1::Pwm8_11b,
            15 => Ccib1::Pwm12_15b,
            _ => unreachable!(),
        }
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio0(&self) -> bool {
        *self == Ccib1::Tmrio0
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio1(&self) -> bool {
        *self == Ccib1::Tmrio1
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio2(&self) -> bool {
        *self == Ccib1::Tmrio2
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio3(&self) -> bool {
        *self == Ccib1::Tmrio3
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Ccib1::Gpioab
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Ccib1::Gpiocd
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Ccib1::Gpioef
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Ccib1::Gpiog
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm0_3a(&self) -> bool {
        *self == Ccib1::Pwm0_3a
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm4_7a(&self) -> bool {
        *self == Ccib1::Pwm4_7a
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm8_11a(&self) -> bool {
        *self == Ccib1::Pwm8_11a
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm12_15a(&self) -> bool {
        *self == Ccib1::Pwm12_15a
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm0_3b(&self) -> bool {
        *self == Ccib1::Pwm0_3b
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm4_7b(&self) -> bool {
        *self == Ccib1::Pwm4_7b
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm8_11b(&self) -> bool {
        *self == Ccib1::Pwm8_11b
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm12_15b(&self) -> bool {
        *self == Ccib1::Pwm12_15b
    }
}
#[doc = "Field `CCIB_1` writer - Select signal for TMRxx CCIB 1 input"]
pub type Ccib1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccib1, crate::Safe>;
impl<'a, REG> Ccib1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Tmrio0)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Tmrio1)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Tmrio2)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Tmrio3)
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Gpioab)
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Gpiocd)
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Gpioef)
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Gpiog)
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm0_3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Pwm0_3a)
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm4_7a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Pwm4_7a)
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm8_11a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Pwm8_11a)
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm12_15a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Pwm12_15a)
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm0_3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Pwm0_3b)
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm4_7b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Pwm4_7b)
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm8_11b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Pwm8_11b)
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm12_15b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib1::Pwm12_15b)
    }
}
#[doc = "Select signal for TMRxx CCIA 2 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccia2 {
    #[doc = "0: External pin"]
    Tmrio0 = 0,
    #[doc = "1: External pin"]
    Tmrio1 = 1,
    #[doc = "2: External pin"]
    Tmrio2 = 2,
    #[doc = "3: External pin"]
    Tmrio3 = 3,
    #[doc = "4: GPIOA or GPIOB adc soc impulse"]
    Gpioab = 4,
    #[doc = "5: GPIOC or GPIOD adc soc impulse"]
    Gpiocd = 5,
    #[doc = "6: GPIOE or GPIOF adc soc impulse"]
    Gpioef = 6,
    #[doc = "7: GPIOG adc soc impulse"]
    Gpiog = 7,
    #[doc = "8: PWM0-3 adc soc channel A impulse"]
    Pwm0_3a = 8,
    #[doc = "9: PWM4-7 adc soc channel A impulse"]
    Pwm4_7a = 9,
    #[doc = "10: PWM8-11 adc soc channel A impulse"]
    Pwm8_11a = 10,
    #[doc = "11: PWM12-15 adc soc channel A impulse"]
    Pwm12_15a = 11,
    #[doc = "12: PWM0-3 adc soc channel B impulse"]
    Pwm0_3b = 12,
    #[doc = "13: PWM4-7 adc soc channel B impulse"]
    Pwm4_7b = 13,
    #[doc = "14: PWM8-11 adc soc channel B impulse"]
    Pwm8_11b = 14,
    #[doc = "15: PWM12-15 adc soc channel B impulse"]
    Pwm12_15b = 15,
}
impl From<Ccia2> for u8 {
    #[inline(always)]
    fn from(variant: Ccia2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccia2 {
    type Ux = u8;
}
impl crate::IsEnum for Ccia2 {}
#[doc = "Field `CCIA_2` reader - Select signal for TMRxx CCIA 2 input"]
pub type Ccia2R = crate::FieldReader<Ccia2>;
impl Ccia2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccia2 {
        match self.bits {
            0 => Ccia2::Tmrio0,
            1 => Ccia2::Tmrio1,
            2 => Ccia2::Tmrio2,
            3 => Ccia2::Tmrio3,
            4 => Ccia2::Gpioab,
            5 => Ccia2::Gpiocd,
            6 => Ccia2::Gpioef,
            7 => Ccia2::Gpiog,
            8 => Ccia2::Pwm0_3a,
            9 => Ccia2::Pwm4_7a,
            10 => Ccia2::Pwm8_11a,
            11 => Ccia2::Pwm12_15a,
            12 => Ccia2::Pwm0_3b,
            13 => Ccia2::Pwm4_7b,
            14 => Ccia2::Pwm8_11b,
            15 => Ccia2::Pwm12_15b,
            _ => unreachable!(),
        }
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio0(&self) -> bool {
        *self == Ccia2::Tmrio0
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio1(&self) -> bool {
        *self == Ccia2::Tmrio1
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio2(&self) -> bool {
        *self == Ccia2::Tmrio2
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio3(&self) -> bool {
        *self == Ccia2::Tmrio3
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Ccia2::Gpioab
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Ccia2::Gpiocd
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Ccia2::Gpioef
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Ccia2::Gpiog
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm0_3a(&self) -> bool {
        *self == Ccia2::Pwm0_3a
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm4_7a(&self) -> bool {
        *self == Ccia2::Pwm4_7a
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm8_11a(&self) -> bool {
        *self == Ccia2::Pwm8_11a
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm12_15a(&self) -> bool {
        *self == Ccia2::Pwm12_15a
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm0_3b(&self) -> bool {
        *self == Ccia2::Pwm0_3b
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm4_7b(&self) -> bool {
        *self == Ccia2::Pwm4_7b
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm8_11b(&self) -> bool {
        *self == Ccia2::Pwm8_11b
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm12_15b(&self) -> bool {
        *self == Ccia2::Pwm12_15b
    }
}
#[doc = "Field `CCIA_2` writer - Select signal for TMRxx CCIA 2 input"]
pub type Ccia2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccia2, crate::Safe>;
impl<'a, REG> Ccia2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Tmrio0)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Tmrio1)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Tmrio2)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Tmrio3)
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Gpioab)
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Gpiocd)
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Gpioef)
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Gpiog)
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm0_3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Pwm0_3a)
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm4_7a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Pwm4_7a)
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm8_11a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Pwm8_11a)
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm12_15a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Pwm12_15a)
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm0_3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Pwm0_3b)
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm4_7b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Pwm4_7b)
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm8_11b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Pwm8_11b)
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm12_15b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia2::Pwm12_15b)
    }
}
#[doc = "Select signal for TMRxx CCIB 2 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccib2 {
    #[doc = "0: External pin"]
    Tmrio0 = 0,
    #[doc = "1: External pin"]
    Tmrio1 = 1,
    #[doc = "2: External pin"]
    Tmrio2 = 2,
    #[doc = "3: External pin"]
    Tmrio3 = 3,
    #[doc = "4: GPIOA or GPIOB adc soc impulse"]
    Gpioab = 4,
    #[doc = "5: GPIOC or GPIOD adc soc impulse"]
    Gpiocd = 5,
    #[doc = "6: GPIOE or GPIOF adc soc impulse"]
    Gpioef = 6,
    #[doc = "7: GPIOG adc soc impulse"]
    Gpiog = 7,
    #[doc = "8: PWM0-3 adc soc channel A impulse"]
    Pwm0_3a = 8,
    #[doc = "9: PWM4-7 adc soc channel A impulse"]
    Pwm4_7a = 9,
    #[doc = "10: PWM8-11 adc soc channel A impulse"]
    Pwm8_11a = 10,
    #[doc = "11: PWM12-15 adc soc channel A impulse"]
    Pwm12_15a = 11,
    #[doc = "12: PWM0-3 adc soc channel B impulse"]
    Pwm0_3b = 12,
    #[doc = "13: PWM4-7 adc soc channel B impulse"]
    Pwm4_7b = 13,
    #[doc = "14: PWM8-11 adc soc channel B impulse"]
    Pwm8_11b = 14,
    #[doc = "15: PWM12-15 adc soc channel B impulse"]
    Pwm12_15b = 15,
}
impl From<Ccib2> for u8 {
    #[inline(always)]
    fn from(variant: Ccib2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccib2 {
    type Ux = u8;
}
impl crate::IsEnum for Ccib2 {}
#[doc = "Field `CCIB_2` reader - Select signal for TMRxx CCIB 2 input"]
pub type Ccib2R = crate::FieldReader<Ccib2>;
impl Ccib2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccib2 {
        match self.bits {
            0 => Ccib2::Tmrio0,
            1 => Ccib2::Tmrio1,
            2 => Ccib2::Tmrio2,
            3 => Ccib2::Tmrio3,
            4 => Ccib2::Gpioab,
            5 => Ccib2::Gpiocd,
            6 => Ccib2::Gpioef,
            7 => Ccib2::Gpiog,
            8 => Ccib2::Pwm0_3a,
            9 => Ccib2::Pwm4_7a,
            10 => Ccib2::Pwm8_11a,
            11 => Ccib2::Pwm12_15a,
            12 => Ccib2::Pwm0_3b,
            13 => Ccib2::Pwm4_7b,
            14 => Ccib2::Pwm8_11b,
            15 => Ccib2::Pwm12_15b,
            _ => unreachable!(),
        }
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio0(&self) -> bool {
        *self == Ccib2::Tmrio0
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio1(&self) -> bool {
        *self == Ccib2::Tmrio1
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio2(&self) -> bool {
        *self == Ccib2::Tmrio2
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio3(&self) -> bool {
        *self == Ccib2::Tmrio3
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Ccib2::Gpioab
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Ccib2::Gpiocd
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Ccib2::Gpioef
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Ccib2::Gpiog
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm0_3a(&self) -> bool {
        *self == Ccib2::Pwm0_3a
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm4_7a(&self) -> bool {
        *self == Ccib2::Pwm4_7a
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm8_11a(&self) -> bool {
        *self == Ccib2::Pwm8_11a
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm12_15a(&self) -> bool {
        *self == Ccib2::Pwm12_15a
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm0_3b(&self) -> bool {
        *self == Ccib2::Pwm0_3b
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm4_7b(&self) -> bool {
        *self == Ccib2::Pwm4_7b
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm8_11b(&self) -> bool {
        *self == Ccib2::Pwm8_11b
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm12_15b(&self) -> bool {
        *self == Ccib2::Pwm12_15b
    }
}
#[doc = "Field `CCIB_2` writer - Select signal for TMRxx CCIB 2 input"]
pub type Ccib2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccib2, crate::Safe>;
impl<'a, REG> Ccib2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Tmrio0)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Tmrio1)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Tmrio2)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Tmrio3)
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Gpioab)
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Gpiocd)
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Gpioef)
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Gpiog)
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm0_3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Pwm0_3a)
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm4_7a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Pwm4_7a)
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm8_11a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Pwm8_11a)
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm12_15a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Pwm12_15a)
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm0_3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Pwm0_3b)
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm4_7b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Pwm4_7b)
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm8_11b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Pwm8_11b)
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm12_15b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib2::Pwm12_15b)
    }
}
#[doc = "Select signal for TMRxx CCIA 3 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccia3 {
    #[doc = "0: External pin"]
    Tmrio0 = 0,
    #[doc = "1: External pin"]
    Tmrio1 = 1,
    #[doc = "2: External pin"]
    Tmrio2 = 2,
    #[doc = "3: External pin"]
    Tmrio3 = 3,
    #[doc = "4: GPIOA or GPIOB adc soc impulse"]
    Gpioab = 4,
    #[doc = "5: GPIOC or GPIOD adc soc impulse"]
    Gpiocd = 5,
    #[doc = "6: GPIOE or GPIOF adc soc impulse"]
    Gpioef = 6,
    #[doc = "7: GPIOG adc soc impulse"]
    Gpiog = 7,
    #[doc = "8: PWM0-3 adc soc channel A impulse"]
    Pwm0_3a = 8,
    #[doc = "9: PWM4-7 adc soc channel A impulse"]
    Pwm4_7a = 9,
    #[doc = "10: PWM8-11 adc soc channel A impulse"]
    Pwm8_11a = 10,
    #[doc = "11: PWM12-15 adc soc channel A impulse"]
    Pwm12_15a = 11,
    #[doc = "12: PWM0-3 adc soc channel B impulse"]
    Pwm0_3b = 12,
    #[doc = "13: PWM4-7 adc soc channel B impulse"]
    Pwm4_7b = 13,
    #[doc = "14: PWM8-11 adc soc channel B impulse"]
    Pwm8_11b = 14,
    #[doc = "15: PWM12-15 adc soc channel B impulse"]
    Pwm12_15b = 15,
}
impl From<Ccia3> for u8 {
    #[inline(always)]
    fn from(variant: Ccia3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccia3 {
    type Ux = u8;
}
impl crate::IsEnum for Ccia3 {}
#[doc = "Field `CCIA_3` reader - Select signal for TMRxx CCIA 3 input"]
pub type Ccia3R = crate::FieldReader<Ccia3>;
impl Ccia3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccia3 {
        match self.bits {
            0 => Ccia3::Tmrio0,
            1 => Ccia3::Tmrio1,
            2 => Ccia3::Tmrio2,
            3 => Ccia3::Tmrio3,
            4 => Ccia3::Gpioab,
            5 => Ccia3::Gpiocd,
            6 => Ccia3::Gpioef,
            7 => Ccia3::Gpiog,
            8 => Ccia3::Pwm0_3a,
            9 => Ccia3::Pwm4_7a,
            10 => Ccia3::Pwm8_11a,
            11 => Ccia3::Pwm12_15a,
            12 => Ccia3::Pwm0_3b,
            13 => Ccia3::Pwm4_7b,
            14 => Ccia3::Pwm8_11b,
            15 => Ccia3::Pwm12_15b,
            _ => unreachable!(),
        }
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio0(&self) -> bool {
        *self == Ccia3::Tmrio0
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio1(&self) -> bool {
        *self == Ccia3::Tmrio1
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio2(&self) -> bool {
        *self == Ccia3::Tmrio2
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio3(&self) -> bool {
        *self == Ccia3::Tmrio3
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Ccia3::Gpioab
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Ccia3::Gpiocd
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Ccia3::Gpioef
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Ccia3::Gpiog
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm0_3a(&self) -> bool {
        *self == Ccia3::Pwm0_3a
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm4_7a(&self) -> bool {
        *self == Ccia3::Pwm4_7a
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm8_11a(&self) -> bool {
        *self == Ccia3::Pwm8_11a
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm12_15a(&self) -> bool {
        *self == Ccia3::Pwm12_15a
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm0_3b(&self) -> bool {
        *self == Ccia3::Pwm0_3b
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm4_7b(&self) -> bool {
        *self == Ccia3::Pwm4_7b
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm8_11b(&self) -> bool {
        *self == Ccia3::Pwm8_11b
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm12_15b(&self) -> bool {
        *self == Ccia3::Pwm12_15b
    }
}
#[doc = "Field `CCIA_3` writer - Select signal for TMRxx CCIA 3 input"]
pub type Ccia3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccia3, crate::Safe>;
impl<'a, REG> Ccia3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Tmrio0)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Tmrio1)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Tmrio2)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Tmrio3)
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Gpioab)
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Gpiocd)
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Gpioef)
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Gpiog)
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm0_3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Pwm0_3a)
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm4_7a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Pwm4_7a)
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm8_11a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Pwm8_11a)
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm12_15a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Pwm12_15a)
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm0_3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Pwm0_3b)
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm4_7b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Pwm4_7b)
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm8_11b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Pwm8_11b)
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm12_15b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccia3::Pwm12_15b)
    }
}
#[doc = "Select signal for TMRxx CCIB 3 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccib3 {
    #[doc = "0: External pin"]
    Tmrio0 = 0,
    #[doc = "1: External pin"]
    Tmrio1 = 1,
    #[doc = "2: External pin"]
    Tmrio2 = 2,
    #[doc = "3: External pin"]
    Tmrio3 = 3,
    #[doc = "4: GPIOA or GPIOB adc soc impulse"]
    Gpioab = 4,
    #[doc = "5: GPIOC or GPIOD adc soc impulse"]
    Gpiocd = 5,
    #[doc = "6: GPIOE or GPIOF adc soc impulse"]
    Gpioef = 6,
    #[doc = "7: GPIOG adc soc impulse"]
    Gpiog = 7,
    #[doc = "8: PWM0-3 adc soc channel A impulse"]
    Pwm0_3a = 8,
    #[doc = "9: PWM4-7 adc soc channel A impulse"]
    Pwm4_7a = 9,
    #[doc = "10: PWM8-11 adc soc channel A impulse"]
    Pwm8_11a = 10,
    #[doc = "11: PWM12-15 adc soc channel A impulse"]
    Pwm12_15a = 11,
    #[doc = "12: PWM0-3 adc soc channel B impulse"]
    Pwm0_3b = 12,
    #[doc = "13: PWM4-7 adc soc channel B impulse"]
    Pwm4_7b = 13,
    #[doc = "14: PWM8-11 adc soc channel B impulse"]
    Pwm8_11b = 14,
    #[doc = "15: PWM12-15 adc soc channel B impulse"]
    Pwm12_15b = 15,
}
impl From<Ccib3> for u8 {
    #[inline(always)]
    fn from(variant: Ccib3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccib3 {
    type Ux = u8;
}
impl crate::IsEnum for Ccib3 {}
#[doc = "Field `CCIB_3` reader - Select signal for TMRxx CCIB 3 input"]
pub type Ccib3R = crate::FieldReader<Ccib3>;
impl Ccib3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccib3 {
        match self.bits {
            0 => Ccib3::Tmrio0,
            1 => Ccib3::Tmrio1,
            2 => Ccib3::Tmrio2,
            3 => Ccib3::Tmrio3,
            4 => Ccib3::Gpioab,
            5 => Ccib3::Gpiocd,
            6 => Ccib3::Gpioef,
            7 => Ccib3::Gpiog,
            8 => Ccib3::Pwm0_3a,
            9 => Ccib3::Pwm4_7a,
            10 => Ccib3::Pwm8_11a,
            11 => Ccib3::Pwm12_15a,
            12 => Ccib3::Pwm0_3b,
            13 => Ccib3::Pwm4_7b,
            14 => Ccib3::Pwm8_11b,
            15 => Ccib3::Pwm12_15b,
            _ => unreachable!(),
        }
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio0(&self) -> bool {
        *self == Ccib3::Tmrio0
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio1(&self) -> bool {
        *self == Ccib3::Tmrio1
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio2(&self) -> bool {
        *self == Ccib3::Tmrio2
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn is_tmrio3(&self) -> bool {
        *self == Ccib3::Tmrio3
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Ccib3::Gpioab
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Ccib3::Gpiocd
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Ccib3::Gpioef
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Ccib3::Gpiog
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm0_3a(&self) -> bool {
        *self == Ccib3::Pwm0_3a
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm4_7a(&self) -> bool {
        *self == Ccib3::Pwm4_7a
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm8_11a(&self) -> bool {
        *self == Ccib3::Pwm8_11a
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn is_pwm12_15a(&self) -> bool {
        *self == Ccib3::Pwm12_15a
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm0_3b(&self) -> bool {
        *self == Ccib3::Pwm0_3b
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm4_7b(&self) -> bool {
        *self == Ccib3::Pwm4_7b
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm8_11b(&self) -> bool {
        *self == Ccib3::Pwm8_11b
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn is_pwm12_15b(&self) -> bool {
        *self == Ccib3::Pwm12_15b
    }
}
#[doc = "Field `CCIB_3` writer - Select signal for TMRxx CCIB 3 input"]
pub type Ccib3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ccib3, crate::Safe>;
impl<'a, REG> Ccib3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Tmrio0)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Tmrio1)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Tmrio2)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn tmrio3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Tmrio3)
    }
    #[doc = "GPIOA or GPIOB adc soc impulse"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Gpioab)
    }
    #[doc = "GPIOC or GPIOD adc soc impulse"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Gpiocd)
    }
    #[doc = "GPIOE or GPIOF adc soc impulse"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Gpioef)
    }
    #[doc = "GPIOG adc soc impulse"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Gpiog)
    }
    #[doc = "PWM0-3 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm0_3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Pwm0_3a)
    }
    #[doc = "PWM4-7 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm4_7a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Pwm4_7a)
    }
    #[doc = "PWM8-11 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm8_11a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Pwm8_11a)
    }
    #[doc = "PWM12-15 adc soc channel A impulse"]
    #[inline(always)]
    pub fn pwm12_15a(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Pwm12_15a)
    }
    #[doc = "PWM0-3 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm0_3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Pwm0_3b)
    }
    #[doc = "PWM4-7 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm4_7b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Pwm4_7b)
    }
    #[doc = "PWM8-11 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm8_11b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Pwm8_11b)
    }
    #[doc = "PWM12-15 adc soc channel B impulse"]
    #[inline(always)]
    pub fn pwm12_15b(self) -> &'a mut crate::W<REG> {
        self.variant(Ccib3::Pwm12_15b)
    }
}
impl R {
    #[doc = "Bits 0:3 - Select signal for TMRxx CCIA 0 input"]
    #[inline(always)]
    pub fn ccia_0(&self) -> Ccia0R {
        Ccia0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select signal for TMRxx CCIB 0 input"]
    #[inline(always)]
    pub fn ccib_0(&self) -> Ccib0R {
        Ccib0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select signal for TMRxx CCIA 1 input"]
    #[inline(always)]
    pub fn ccia_1(&self) -> Ccia1R {
        Ccia1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select signal for TMRxx CCIB 1 input"]
    #[inline(always)]
    pub fn ccib_1(&self) -> Ccib1R {
        Ccib1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select signal for TMRxx CCIA 2 input"]
    #[inline(always)]
    pub fn ccia_2(&self) -> Ccia2R {
        Ccia2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Select signal for TMRxx CCIB 2 input"]
    #[inline(always)]
    pub fn ccib_2(&self) -> Ccib2R {
        Ccib2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select signal for TMRxx CCIA 3 input"]
    #[inline(always)]
    pub fn ccia_3(&self) -> Ccia3R {
        Ccia3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Select signal for TMRxx CCIB 3 input"]
    #[inline(always)]
    pub fn ccib_3(&self) -> Ccib3R {
        Ccib3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select signal for TMRxx CCIA 0 input"]
    #[inline(always)]
    pub fn ccia_0(&mut self) -> Ccia0W<'_, ValueSpec> {
        Ccia0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select signal for TMRxx CCIB 0 input"]
    #[inline(always)]
    pub fn ccib_0(&mut self) -> Ccib0W<'_, ValueSpec> {
        Ccib0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Select signal for TMRxx CCIA 1 input"]
    #[inline(always)]
    pub fn ccia_1(&mut self) -> Ccia1W<'_, ValueSpec> {
        Ccia1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Select signal for TMRxx CCIB 1 input"]
    #[inline(always)]
    pub fn ccib_1(&mut self) -> Ccib1W<'_, ValueSpec> {
        Ccib1W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Select signal for TMRxx CCIA 2 input"]
    #[inline(always)]
    pub fn ccia_2(&mut self) -> Ccia2W<'_, ValueSpec> {
        Ccia2W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Select signal for TMRxx CCIB 2 input"]
    #[inline(always)]
    pub fn ccib_2(&mut self) -> Ccib2W<'_, ValueSpec> {
        Ccib2W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Select signal for TMRxx CCIA 3 input"]
    #[inline(always)]
    pub fn ccia_3(&mut self) -> Ccia3W<'_, ValueSpec> {
        Ccia3W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Select signal for TMRxx CCIB 3 input"]
    #[inline(always)]
    pub fn ccib_3(&mut self) -> Ccib3W<'_, ValueSpec> {
        Ccib3W::new(self, 28)
    }
}
#[doc = "TMRxx ccix input select (0-3 array consist of full 32 bit register, 4-16 only CCI*_0)\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
