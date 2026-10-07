#[doc = "Register `EMUX` reader"]
pub type R = crate::R<EmuxSpec>;
#[doc = "Register `EMUX` writer"]
pub type W = crate::W<EmuxSpec>;
#[doc = "Select start event for sequencer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em0 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: ACMP 0 interrupt"]
    Acmp0 = 1,
    #[doc = "2: ACMP 1 interrupt"]
    Acmp1 = 2,
    #[doc = "3: ACMP 2 interrupt"]
    Acmp2 = 3,
    #[doc = "4: ACMP 3 interrupt"]
    Acmp3 = 4,
    #[doc = "5: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 5,
    #[doc = "6: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 6,
    #[doc = "7: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 7,
    #[doc = "8: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 8,
    #[doc = "9: GPIO A or GPIO B interrupt"]
    Gpioab = 9,
    #[doc = "10: GPIO C or GPIO D interrupt"]
    Gpiocd = 10,
    #[doc = "11: GPIO E or GPIO F interrupt"]
    Gpioef = 11,
    #[doc = "12: GPIO G interrupt"]
    Gpiog = 12,
    #[doc = "13: Combined all 16 A channels of PWMs"]
    Pwmagrp = 13,
    #[doc = "14: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 14,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em0> for u8 {
    #[inline(always)]
    fn from(variant: Em0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em0 {
    type Ux = u8;
}
impl crate::IsEnum for Em0 {}
#[doc = "Field `EM0` reader - Select start event for sequencer 0"]
pub type Em0R = crate::FieldReader<Em0>;
impl Em0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em0 {
        match self.bits {
            0 => Em0::SwReq,
            1 => Em0::Acmp0,
            2 => Em0::Acmp1,
            3 => Em0::Acmp2,
            4 => Em0::Acmp3,
            5 => Em0::Tmr0123,
            6 => Em0::Tmr4567,
            7 => Em0::Tmr89ab,
            8 => Em0::Tmrcdef,
            9 => Em0::Gpioab,
            10 => Em0::Gpiocd,
            11 => Em0::Gpioef,
            12 => Em0::Gpiog,
            13 => Em0::Pwmagrp,
            14 => Em0::Pwmbgrp,
            15 => Em0::Cycle,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em0::SwReq
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Em0::Acmp0
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Em0::Acmp1
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Em0::Acmp2
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Em0::Acmp3
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Em0::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Em0::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Em0::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Em0::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Em0::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Em0::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Em0::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Em0::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Em0::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Em0::Pwmbgrp
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em0::Cycle
    }
}
#[doc = "Field `EM0` writer - Select start event for sequencer 0"]
pub type Em0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em0, crate::Safe>;
impl<'a, REG> Em0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::SwReq)
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Acmp0)
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Acmp1)
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Acmp2)
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Acmp3)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Pwmbgrp)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em0::Cycle)
    }
}
#[doc = "Select start event for sequencer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em1 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: ACMP 0 interrupt"]
    Acmp0 = 1,
    #[doc = "2: ACMP 1 interrupt"]
    Acmp1 = 2,
    #[doc = "3: ACMP 2 interrupt"]
    Acmp2 = 3,
    #[doc = "4: ACMP 3 interrupt"]
    Acmp3 = 4,
    #[doc = "5: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 5,
    #[doc = "6: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 6,
    #[doc = "7: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 7,
    #[doc = "8: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 8,
    #[doc = "9: GPIO A or GPIO B interrupt"]
    Gpioab = 9,
    #[doc = "10: GPIO C or GPIO D interrupt"]
    Gpiocd = 10,
    #[doc = "11: GPIO E or GPIO F interrupt"]
    Gpioef = 11,
    #[doc = "12: GPIO G interrupt"]
    Gpiog = 12,
    #[doc = "13: Combined all 16 A channels of PWMs"]
    Pwmagrp = 13,
    #[doc = "14: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 14,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em1> for u8 {
    #[inline(always)]
    fn from(variant: Em1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em1 {
    type Ux = u8;
}
impl crate::IsEnum for Em1 {}
#[doc = "Field `EM1` reader - Select start event for sequencer 1"]
pub type Em1R = crate::FieldReader<Em1>;
impl Em1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em1 {
        match self.bits {
            0 => Em1::SwReq,
            1 => Em1::Acmp0,
            2 => Em1::Acmp1,
            3 => Em1::Acmp2,
            4 => Em1::Acmp3,
            5 => Em1::Tmr0123,
            6 => Em1::Tmr4567,
            7 => Em1::Tmr89ab,
            8 => Em1::Tmrcdef,
            9 => Em1::Gpioab,
            10 => Em1::Gpiocd,
            11 => Em1::Gpioef,
            12 => Em1::Gpiog,
            13 => Em1::Pwmagrp,
            14 => Em1::Pwmbgrp,
            15 => Em1::Cycle,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em1::SwReq
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Em1::Acmp0
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Em1::Acmp1
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Em1::Acmp2
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Em1::Acmp3
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Em1::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Em1::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Em1::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Em1::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Em1::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Em1::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Em1::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Em1::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Em1::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Em1::Pwmbgrp
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em1::Cycle
    }
}
#[doc = "Field `EM1` writer - Select start event for sequencer 1"]
pub type Em1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em1, crate::Safe>;
impl<'a, REG> Em1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::SwReq)
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Acmp0)
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Acmp1)
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Acmp2)
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Acmp3)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Pwmbgrp)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em1::Cycle)
    }
}
#[doc = "Select start event for sequencer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em2 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: ACMP 0 interrupt"]
    Acmp0 = 1,
    #[doc = "2: ACMP 1 interrupt"]
    Acmp1 = 2,
    #[doc = "3: ACMP 2 interrupt"]
    Acmp2 = 3,
    #[doc = "4: ACMP 3 interrupt"]
    Acmp3 = 4,
    #[doc = "5: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 5,
    #[doc = "6: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 6,
    #[doc = "7: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 7,
    #[doc = "8: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 8,
    #[doc = "9: GPIO A or GPIO B interrupt"]
    Gpioab = 9,
    #[doc = "10: GPIO C or GPIO D interrupt"]
    Gpiocd = 10,
    #[doc = "11: GPIO E or GPIO F interrupt"]
    Gpioef = 11,
    #[doc = "12: GPIO G interrupt"]
    Gpiog = 12,
    #[doc = "13: Combined all 16 A channels of PWMs"]
    Pwmagrp = 13,
    #[doc = "14: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 14,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em2> for u8 {
    #[inline(always)]
    fn from(variant: Em2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em2 {
    type Ux = u8;
}
impl crate::IsEnum for Em2 {}
#[doc = "Field `EM2` reader - Select start event for sequencer 2"]
pub type Em2R = crate::FieldReader<Em2>;
impl Em2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em2 {
        match self.bits {
            0 => Em2::SwReq,
            1 => Em2::Acmp0,
            2 => Em2::Acmp1,
            3 => Em2::Acmp2,
            4 => Em2::Acmp3,
            5 => Em2::Tmr0123,
            6 => Em2::Tmr4567,
            7 => Em2::Tmr89ab,
            8 => Em2::Tmrcdef,
            9 => Em2::Gpioab,
            10 => Em2::Gpiocd,
            11 => Em2::Gpioef,
            12 => Em2::Gpiog,
            13 => Em2::Pwmagrp,
            14 => Em2::Pwmbgrp,
            15 => Em2::Cycle,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em2::SwReq
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Em2::Acmp0
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Em2::Acmp1
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Em2::Acmp2
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Em2::Acmp3
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Em2::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Em2::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Em2::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Em2::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Em2::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Em2::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Em2::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Em2::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Em2::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Em2::Pwmbgrp
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em2::Cycle
    }
}
#[doc = "Field `EM2` writer - Select start event for sequencer 2"]
pub type Em2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em2, crate::Safe>;
impl<'a, REG> Em2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::SwReq)
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Acmp0)
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Acmp1)
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Acmp2)
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Acmp3)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Pwmbgrp)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em2::Cycle)
    }
}
#[doc = "Select start event for sequencer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em3 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: ACMP 0 interrupt"]
    Acmp0 = 1,
    #[doc = "2: ACMP 1 interrupt"]
    Acmp1 = 2,
    #[doc = "3: ACMP 2 interrupt"]
    Acmp2 = 3,
    #[doc = "4: ACMP 3 interrupt"]
    Acmp3 = 4,
    #[doc = "5: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 5,
    #[doc = "6: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 6,
    #[doc = "7: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 7,
    #[doc = "8: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 8,
    #[doc = "9: GPIO A or GPIO B interrupt"]
    Gpioab = 9,
    #[doc = "10: GPIO C or GPIO D interrupt"]
    Gpiocd = 10,
    #[doc = "11: GPIO E or GPIO F interrupt"]
    Gpioef = 11,
    #[doc = "12: GPIO G interrupt"]
    Gpiog = 12,
    #[doc = "13: Combined all 16 A channels of PWMs"]
    Pwmagrp = 13,
    #[doc = "14: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 14,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em3> for u8 {
    #[inline(always)]
    fn from(variant: Em3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em3 {
    type Ux = u8;
}
impl crate::IsEnum for Em3 {}
#[doc = "Field `EM3` reader - Select start event for sequencer 3"]
pub type Em3R = crate::FieldReader<Em3>;
impl Em3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em3 {
        match self.bits {
            0 => Em3::SwReq,
            1 => Em3::Acmp0,
            2 => Em3::Acmp1,
            3 => Em3::Acmp2,
            4 => Em3::Acmp3,
            5 => Em3::Tmr0123,
            6 => Em3::Tmr4567,
            7 => Em3::Tmr89ab,
            8 => Em3::Tmrcdef,
            9 => Em3::Gpioab,
            10 => Em3::Gpiocd,
            11 => Em3::Gpioef,
            12 => Em3::Gpiog,
            13 => Em3::Pwmagrp,
            14 => Em3::Pwmbgrp,
            15 => Em3::Cycle,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em3::SwReq
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Em3::Acmp0
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Em3::Acmp1
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Em3::Acmp2
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Em3::Acmp3
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Em3::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Em3::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Em3::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Em3::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Em3::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Em3::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Em3::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Em3::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Em3::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Em3::Pwmbgrp
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em3::Cycle
    }
}
#[doc = "Field `EM3` writer - Select start event for sequencer 3"]
pub type Em3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em3, crate::Safe>;
impl<'a, REG> Em3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::SwReq)
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Acmp0)
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Acmp1)
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Acmp2)
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Acmp3)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Pwmbgrp)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em3::Cycle)
    }
}
#[doc = "Select start event for sequencer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em4 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: ACMP 0 interrupt"]
    Acmp0 = 1,
    #[doc = "2: ACMP 1 interrupt"]
    Acmp1 = 2,
    #[doc = "3: ACMP 2 interrupt"]
    Acmp2 = 3,
    #[doc = "4: ACMP 3 interrupt"]
    Acmp3 = 4,
    #[doc = "5: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 5,
    #[doc = "6: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 6,
    #[doc = "7: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 7,
    #[doc = "8: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 8,
    #[doc = "9: GPIO A or GPIO B interrupt"]
    Gpioab = 9,
    #[doc = "10: GPIO C or GPIO D interrupt"]
    Gpiocd = 10,
    #[doc = "11: GPIO E or GPIO F interrupt"]
    Gpioef = 11,
    #[doc = "12: GPIO G interrupt"]
    Gpiog = 12,
    #[doc = "13: Combined all 16 A channels of PWMs"]
    Pwmagrp = 13,
    #[doc = "14: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 14,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em4> for u8 {
    #[inline(always)]
    fn from(variant: Em4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em4 {
    type Ux = u8;
}
impl crate::IsEnum for Em4 {}
#[doc = "Field `EM4` reader - Select start event for sequencer 4"]
pub type Em4R = crate::FieldReader<Em4>;
impl Em4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em4 {
        match self.bits {
            0 => Em4::SwReq,
            1 => Em4::Acmp0,
            2 => Em4::Acmp1,
            3 => Em4::Acmp2,
            4 => Em4::Acmp3,
            5 => Em4::Tmr0123,
            6 => Em4::Tmr4567,
            7 => Em4::Tmr89ab,
            8 => Em4::Tmrcdef,
            9 => Em4::Gpioab,
            10 => Em4::Gpiocd,
            11 => Em4::Gpioef,
            12 => Em4::Gpiog,
            13 => Em4::Pwmagrp,
            14 => Em4::Pwmbgrp,
            15 => Em4::Cycle,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em4::SwReq
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Em4::Acmp0
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Em4::Acmp1
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Em4::Acmp2
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Em4::Acmp3
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Em4::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Em4::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Em4::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Em4::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Em4::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Em4::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Em4::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Em4::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Em4::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Em4::Pwmbgrp
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em4::Cycle
    }
}
#[doc = "Field `EM4` writer - Select start event for sequencer 4"]
pub type Em4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em4, crate::Safe>;
impl<'a, REG> Em4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::SwReq)
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Acmp0)
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Acmp1)
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Acmp2)
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Acmp3)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Pwmbgrp)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em4::Cycle)
    }
}
#[doc = "Select start event for sequencer 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em5 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: ACMP 0 interrupt"]
    Acmp0 = 1,
    #[doc = "2: ACMP 1 interrupt"]
    Acmp1 = 2,
    #[doc = "3: ACMP 2 interrupt"]
    Acmp2 = 3,
    #[doc = "4: ACMP 3 interrupt"]
    Acmp3 = 4,
    #[doc = "5: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 5,
    #[doc = "6: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 6,
    #[doc = "7: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 7,
    #[doc = "8: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 8,
    #[doc = "9: GPIO A or GPIO B interrupt"]
    Gpioab = 9,
    #[doc = "10: GPIO C or GPIO D interrupt"]
    Gpiocd = 10,
    #[doc = "11: GPIO E or GPIO F interrupt"]
    Gpioef = 11,
    #[doc = "12: GPIO G interrupt"]
    Gpiog = 12,
    #[doc = "13: Combined all 16 A channels of PWMs"]
    Pwmagrp = 13,
    #[doc = "14: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 14,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em5> for u8 {
    #[inline(always)]
    fn from(variant: Em5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em5 {
    type Ux = u8;
}
impl crate::IsEnum for Em5 {}
#[doc = "Field `EM5` reader - Select start event for sequencer 5"]
pub type Em5R = crate::FieldReader<Em5>;
impl Em5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em5 {
        match self.bits {
            0 => Em5::SwReq,
            1 => Em5::Acmp0,
            2 => Em5::Acmp1,
            3 => Em5::Acmp2,
            4 => Em5::Acmp3,
            5 => Em5::Tmr0123,
            6 => Em5::Tmr4567,
            7 => Em5::Tmr89ab,
            8 => Em5::Tmrcdef,
            9 => Em5::Gpioab,
            10 => Em5::Gpiocd,
            11 => Em5::Gpioef,
            12 => Em5::Gpiog,
            13 => Em5::Pwmagrp,
            14 => Em5::Pwmbgrp,
            15 => Em5::Cycle,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em5::SwReq
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Em5::Acmp0
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Em5::Acmp1
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Em5::Acmp2
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Em5::Acmp3
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Em5::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Em5::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Em5::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Em5::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Em5::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Em5::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Em5::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Em5::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Em5::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Em5::Pwmbgrp
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em5::Cycle
    }
}
#[doc = "Field `EM5` writer - Select start event for sequencer 5"]
pub type Em5W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em5, crate::Safe>;
impl<'a, REG> Em5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::SwReq)
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Acmp0)
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Acmp1)
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Acmp2)
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Acmp3)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Pwmbgrp)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em5::Cycle)
    }
}
#[doc = "Select start event for sequencer 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em6 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: ACMP 0 interrupt"]
    Acmp0 = 1,
    #[doc = "2: ACMP 1 interrupt"]
    Acmp1 = 2,
    #[doc = "3: ACMP 2 interrupt"]
    Acmp2 = 3,
    #[doc = "4: ACMP 3 interrupt"]
    Acmp3 = 4,
    #[doc = "5: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 5,
    #[doc = "6: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 6,
    #[doc = "7: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 7,
    #[doc = "8: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 8,
    #[doc = "9: GPIO A or GPIO B interrupt"]
    Gpioab = 9,
    #[doc = "10: GPIO C or GPIO D interrupt"]
    Gpiocd = 10,
    #[doc = "11: GPIO E or GPIO F interrupt"]
    Gpioef = 11,
    #[doc = "12: GPIO G interrupt"]
    Gpiog = 12,
    #[doc = "13: Combined all 16 A channels of PWMs"]
    Pwmagrp = 13,
    #[doc = "14: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 14,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em6> for u8 {
    #[inline(always)]
    fn from(variant: Em6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em6 {
    type Ux = u8;
}
impl crate::IsEnum for Em6 {}
#[doc = "Field `EM6` reader - Select start event for sequencer 6"]
pub type Em6R = crate::FieldReader<Em6>;
impl Em6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em6 {
        match self.bits {
            0 => Em6::SwReq,
            1 => Em6::Acmp0,
            2 => Em6::Acmp1,
            3 => Em6::Acmp2,
            4 => Em6::Acmp3,
            5 => Em6::Tmr0123,
            6 => Em6::Tmr4567,
            7 => Em6::Tmr89ab,
            8 => Em6::Tmrcdef,
            9 => Em6::Gpioab,
            10 => Em6::Gpiocd,
            11 => Em6::Gpioef,
            12 => Em6::Gpiog,
            13 => Em6::Pwmagrp,
            14 => Em6::Pwmbgrp,
            15 => Em6::Cycle,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em6::SwReq
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Em6::Acmp0
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Em6::Acmp1
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Em6::Acmp2
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Em6::Acmp3
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Em6::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Em6::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Em6::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Em6::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Em6::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Em6::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Em6::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Em6::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Em6::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Em6::Pwmbgrp
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em6::Cycle
    }
}
#[doc = "Field `EM6` writer - Select start event for sequencer 6"]
pub type Em6W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em6, crate::Safe>;
impl<'a, REG> Em6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::SwReq)
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Acmp0)
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Acmp1)
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Acmp2)
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Acmp3)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Pwmbgrp)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em6::Cycle)
    }
}
#[doc = "Select start event for sequencer 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em7 {
    #[doc = "0: software request by GSYNC bit"]
    SwReq = 0,
    #[doc = "1: ACMP 0 interrupt"]
    Acmp0 = 1,
    #[doc = "2: ACMP 1 interrupt"]
    Acmp1 = 2,
    #[doc = "3: ACMP 2 interrupt"]
    Acmp2 = 3,
    #[doc = "4: ACMP 3 interrupt"]
    Acmp3 = 4,
    #[doc = "5: Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    Tmr0123 = 5,
    #[doc = "6: Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    Tmr4567 = 6,
    #[doc = "7: Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    Tmr89ab = 7,
    #[doc = "8: Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    Tmrcdef = 8,
    #[doc = "9: GPIO A or GPIO B interrupt"]
    Gpioab = 9,
    #[doc = "10: GPIO C or GPIO D interrupt"]
    Gpiocd = 10,
    #[doc = "11: GPIO E or GPIO F interrupt"]
    Gpioef = 11,
    #[doc = "12: GPIO G interrupt"]
    Gpiog = 12,
    #[doc = "13: Combined all 16 A channels of PWMs"]
    Pwmagrp = 13,
    #[doc = "14: Combined all 16 B channels of PWMs"]
    Pwmbgrp = 14,
    #[doc = "15: Cycle mode"]
    Cycle = 15,
}
impl From<Em7> for u8 {
    #[inline(always)]
    fn from(variant: Em7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em7 {
    type Ux = u8;
}
impl crate::IsEnum for Em7 {}
#[doc = "Field `EM7` reader - Select start event for sequencer 7"]
pub type Em7R = crate::FieldReader<Em7>;
impl Em7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em7 {
        match self.bits {
            0 => Em7::SwReq,
            1 => Em7::Acmp0,
            2 => Em7::Acmp1,
            3 => Em7::Acmp2,
            4 => Em7::Acmp3,
            5 => Em7::Tmr0123,
            6 => Em7::Tmr4567,
            7 => Em7::Tmr89ab,
            8 => Em7::Tmrcdef,
            9 => Em7::Gpioab,
            10 => Em7::Gpiocd,
            11 => Em7::Gpioef,
            12 => Em7::Gpiog,
            13 => Em7::Pwmagrp,
            14 => Em7::Pwmbgrp,
            15 => Em7::Cycle,
            _ => unreachable!(),
        }
    }
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn is_sw_req(&self) -> bool {
        *self == Em7::SwReq
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Em7::Acmp0
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Em7::Acmp1
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == Em7::Acmp2
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == Em7::Acmp3
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn is_tmr0123(&self) -> bool {
        *self == Em7::Tmr0123
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn is_tmr4567(&self) -> bool {
        *self == Em7::Tmr4567
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn is_tmr89ab(&self) -> bool {
        *self == Em7::Tmr89ab
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn is_tmrcdef(&self) -> bool {
        *self == Em7::Tmrcdef
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn is_gpioab(&self) -> bool {
        *self == Em7::Gpioab
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn is_gpiocd(&self) -> bool {
        *self == Em7::Gpiocd
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn is_gpioef(&self) -> bool {
        *self == Em7::Gpioef
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == Em7::Gpiog
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmagrp(&self) -> bool {
        *self == Em7::Pwmagrp
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn is_pwmbgrp(&self) -> bool {
        *self == Em7::Pwmbgrp
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn is_cycle(&self) -> bool {
        *self == Em7::Cycle
    }
}
#[doc = "Field `EM7` writer - Select start event for sequencer 7"]
pub type Em7W<'a, REG> = crate::FieldWriter<'a, REG, 4, Em7, crate::Safe>;
impl<'a, REG> Em7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software request by GSYNC bit"]
    #[inline(always)]
    pub fn sw_req(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::SwReq)
    }
    #[doc = "ACMP 0 interrupt"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Acmp0)
    }
    #[doc = "ACMP 1 interrupt"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Acmp1)
    }
    #[doc = "ACMP 2 interrupt"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Acmp2)
    }
    #[doc = "ACMP 3 interrupt"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Acmp3)
    }
    #[doc = "Timer 0 or Timer 1 or Timer 2 or Timer 3 request"]
    #[inline(always)]
    pub fn tmr0123(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Tmr0123)
    }
    #[doc = "Timer 4 or Timer 5 or Timer 6 or Timer 7 request"]
    #[inline(always)]
    pub fn tmr4567(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Tmr4567)
    }
    #[doc = "Timer 8 or Timer 9 or Timer 10 or Timer 11 request"]
    #[inline(always)]
    pub fn tmr89ab(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Tmr89ab)
    }
    #[doc = "Timer 12 or Timer 13 or Timer 14 or Timer 15 request"]
    #[inline(always)]
    pub fn tmrcdef(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Tmrcdef)
    }
    #[doc = "GPIO A or GPIO B interrupt"]
    #[inline(always)]
    pub fn gpioab(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Gpioab)
    }
    #[doc = "GPIO C or GPIO D interrupt"]
    #[inline(always)]
    pub fn gpiocd(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Gpiocd)
    }
    #[doc = "GPIO E or GPIO F interrupt"]
    #[inline(always)]
    pub fn gpioef(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Gpioef)
    }
    #[doc = "GPIO G interrupt"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Gpiog)
    }
    #[doc = "Combined all 16 A channels of PWMs"]
    #[inline(always)]
    pub fn pwmagrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Pwmagrp)
    }
    #[doc = "Combined all 16 B channels of PWMs"]
    #[inline(always)]
    pub fn pwmbgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Pwmbgrp)
    }
    #[doc = "Cycle mode"]
    #[inline(always)]
    pub fn cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Em7::Cycle)
    }
}
impl R {
    #[doc = "Bits 0:3 - Select start event for sequencer 0"]
    #[inline(always)]
    pub fn em0(&self) -> Em0R {
        Em0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select start event for sequencer 1"]
    #[inline(always)]
    pub fn em1(&self) -> Em1R {
        Em1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select start event for sequencer 2"]
    #[inline(always)]
    pub fn em2(&self) -> Em2R {
        Em2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select start event for sequencer 3"]
    #[inline(always)]
    pub fn em3(&self) -> Em3R {
        Em3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select start event for sequencer 4"]
    #[inline(always)]
    pub fn em4(&self) -> Em4R {
        Em4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Select start event for sequencer 5"]
    #[inline(always)]
    pub fn em5(&self) -> Em5R {
        Em5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select start event for sequencer 6"]
    #[inline(always)]
    pub fn em6(&self) -> Em6R {
        Em6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Select start event for sequencer 7"]
    #[inline(always)]
    pub fn em7(&self) -> Em7R {
        Em7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select start event for sequencer 0"]
    #[inline(always)]
    pub fn em0(&mut self) -> Em0W<'_, EmuxSpec> {
        Em0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select start event for sequencer 1"]
    #[inline(always)]
    pub fn em1(&mut self) -> Em1W<'_, EmuxSpec> {
        Em1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Select start event for sequencer 2"]
    #[inline(always)]
    pub fn em2(&mut self) -> Em2W<'_, EmuxSpec> {
        Em2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Select start event for sequencer 3"]
    #[inline(always)]
    pub fn em3(&mut self) -> Em3W<'_, EmuxSpec> {
        Em3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Select start event for sequencer 4"]
    #[inline(always)]
    pub fn em4(&mut self) -> Em4W<'_, EmuxSpec> {
        Em4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Select start event for sequencer 5"]
    #[inline(always)]
    pub fn em5(&mut self) -> Em5W<'_, EmuxSpec> {
        Em5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Select start event for sequencer 6"]
    #[inline(always)]
    pub fn em6(&mut self) -> Em6W<'_, EmuxSpec> {
        Em6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Select start event for sequencer 7"]
    #[inline(always)]
    pub fn em7(&mut self) -> Em7W<'_, EmuxSpec> {
        Em7W::new(self, 28)
    }
}
#[doc = "Sequencer start event selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`emux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmuxSpec;
impl crate::RegisterSpec for EmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emux::R`](R) reader structure"]
impl crate::Readable for EmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`emux::W`](W) writer structure"]
impl crate::Writable for EmuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMUX to value 0"]
impl crate::Resettable for EmuxSpec {}
