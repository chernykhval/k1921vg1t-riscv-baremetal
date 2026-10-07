#[doc = "Register `PCCTL` reader"]
pub type R = crate::R<PcctlSpec>;
#[doc = "Register `PCCTL` writer"]
pub type W = crate::W<PcctlSpec>;
#[doc = "Field `CHPEN` reader - PWM-chopping enable"]
pub type ChpenR = crate::BitReader;
#[doc = "Field `CHPEN` writer - PWM-chopping enable"]
pub type ChpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSTWTH` reader - One-Shot pulse width"]
pub type OstwthR = crate::FieldReader;
#[doc = "Field `OSTWTH` writer - One-Shot pulse width"]
pub type OstwthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Chopping clock frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chpfreq {
    #[doc = "0: sync frequency divide by 1"]
    Div1 = 0,
    #[doc = "1: sync frequency divide by 2"]
    Div2 = 1,
    #[doc = "2: sync frequency divide by 3"]
    Div3 = 2,
    #[doc = "3: sync frequency divide by 4"]
    Div4 = 3,
    #[doc = "4: sync frequency divide by 5"]
    Div5 = 4,
    #[doc = "5: sync frequency divide by 6"]
    Div6 = 5,
    #[doc = "6: sync frequency divide by 7"]
    Div7 = 6,
    #[doc = "7: sync frequency divide by 8"]
    Div8 = 7,
}
impl From<Chpfreq> for u8 {
    #[inline(always)]
    fn from(variant: Chpfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chpfreq {
    type Ux = u8;
}
impl crate::IsEnum for Chpfreq {}
#[doc = "Field `CHPFREQ` reader - Chopping clock frequency"]
pub type ChpfreqR = crate::FieldReader<Chpfreq>;
impl ChpfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chpfreq {
        match self.bits {
            0 => Chpfreq::Div1,
            1 => Chpfreq::Div2,
            2 => Chpfreq::Div3,
            3 => Chpfreq::Div4,
            4 => Chpfreq::Div5,
            5 => Chpfreq::Div6,
            6 => Chpfreq::Div7,
            7 => Chpfreq::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "sync frequency divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Chpfreq::Div1
    }
    #[doc = "sync frequency divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Chpfreq::Div2
    }
    #[doc = "sync frequency divide by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Chpfreq::Div3
    }
    #[doc = "sync frequency divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Chpfreq::Div4
    }
    #[doc = "sync frequency divide by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Chpfreq::Div5
    }
    #[doc = "sync frequency divide by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Chpfreq::Div6
    }
    #[doc = "sync frequency divide by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Chpfreq::Div7
    }
    #[doc = "sync frequency divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Chpfreq::Div8
    }
}
#[doc = "Field `CHPFREQ` writer - Chopping clock frequency"]
pub type ChpfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3, Chpfreq, crate::Safe>;
impl<'a, REG> ChpfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "sync frequency divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Chpfreq::Div1)
    }
    #[doc = "sync frequency divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Chpfreq::Div2)
    }
    #[doc = "sync frequency divide by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Chpfreq::Div3)
    }
    #[doc = "sync frequency divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Chpfreq::Div4)
    }
    #[doc = "sync frequency divide by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Chpfreq::Div5)
    }
    #[doc = "sync frequency divide by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Chpfreq::Div6)
    }
    #[doc = "sync frequency divide by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Chpfreq::Div7)
    }
    #[doc = "sync frequency divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Chpfreq::Div8)
    }
}
#[doc = "Chopping clock duty cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chpduty {
    #[doc = "0: duty 1/8"]
    Duty1_8 = 0,
    #[doc = "1: duty 2/8"]
    Duty2_8 = 1,
    #[doc = "2: duty 3/8"]
    Duty3_8 = 2,
    #[doc = "3: duty 4/8"]
    Duty4_8 = 3,
    #[doc = "4: duty 5/8"]
    Duty5_8 = 4,
    #[doc = "5: duty 6/8"]
    Duty6_8 = 5,
    #[doc = "6: duty 7/8"]
    Duty7_8 = 6,
}
impl From<Chpduty> for u8 {
    #[inline(always)]
    fn from(variant: Chpduty) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chpduty {
    type Ux = u8;
}
impl crate::IsEnum for Chpduty {}
#[doc = "Field `CHPDUTY` reader - Chopping clock duty cycle"]
pub type ChpdutyR = crate::FieldReader<Chpduty>;
impl ChpdutyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chpduty> {
        match self.bits {
            0 => Some(Chpduty::Duty1_8),
            1 => Some(Chpduty::Duty2_8),
            2 => Some(Chpduty::Duty3_8),
            3 => Some(Chpduty::Duty4_8),
            4 => Some(Chpduty::Duty5_8),
            5 => Some(Chpduty::Duty6_8),
            6 => Some(Chpduty::Duty7_8),
            _ => None,
        }
    }
    #[doc = "duty 1/8"]
    #[inline(always)]
    pub fn is_duty_1_8(&self) -> bool {
        *self == Chpduty::Duty1_8
    }
    #[doc = "duty 2/8"]
    #[inline(always)]
    pub fn is_duty_2_8(&self) -> bool {
        *self == Chpduty::Duty2_8
    }
    #[doc = "duty 3/8"]
    #[inline(always)]
    pub fn is_duty_3_8(&self) -> bool {
        *self == Chpduty::Duty3_8
    }
    #[doc = "duty 4/8"]
    #[inline(always)]
    pub fn is_duty_4_8(&self) -> bool {
        *self == Chpduty::Duty4_8
    }
    #[doc = "duty 5/8"]
    #[inline(always)]
    pub fn is_duty_5_8(&self) -> bool {
        *self == Chpduty::Duty5_8
    }
    #[doc = "duty 6/8"]
    #[inline(always)]
    pub fn is_duty_6_8(&self) -> bool {
        *self == Chpduty::Duty6_8
    }
    #[doc = "duty 7/8"]
    #[inline(always)]
    pub fn is_duty_7_8(&self) -> bool {
        *self == Chpduty::Duty7_8
    }
}
#[doc = "Field `CHPDUTY` writer - Chopping clock duty cycle"]
pub type ChpdutyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Chpduty>;
impl<'a, REG> ChpdutyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "duty 1/8"]
    #[inline(always)]
    pub fn duty_1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chpduty::Duty1_8)
    }
    #[doc = "duty 2/8"]
    #[inline(always)]
    pub fn duty_2_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chpduty::Duty2_8)
    }
    #[doc = "duty 3/8"]
    #[inline(always)]
    pub fn duty_3_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chpduty::Duty3_8)
    }
    #[doc = "duty 4/8"]
    #[inline(always)]
    pub fn duty_4_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chpduty::Duty4_8)
    }
    #[doc = "duty 5/8"]
    #[inline(always)]
    pub fn duty_5_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chpduty::Duty5_8)
    }
    #[doc = "duty 6/8"]
    #[inline(always)]
    pub fn duty_6_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chpduty::Duty6_8)
    }
    #[doc = "duty 7/8"]
    #[inline(always)]
    pub fn duty_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chpduty::Duty7_8)
    }
}
impl R {
    #[doc = "Bit 0 - PWM-chopping enable"]
    #[inline(always)]
    pub fn chpen(&self) -> ChpenR {
        ChpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - One-Shot pulse width"]
    #[inline(always)]
    pub fn ostwth(&self) -> OstwthR {
        OstwthR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Chopping clock frequency"]
    #[inline(always)]
    pub fn chpfreq(&self) -> ChpfreqR {
        ChpfreqR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Chopping clock duty cycle"]
    #[inline(always)]
    pub fn chpduty(&self) -> ChpdutyR {
        ChpdutyR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWM-chopping enable"]
    #[inline(always)]
    pub fn chpen(&mut self) -> ChpenW<'_, PcctlSpec> {
        ChpenW::new(self, 0)
    }
    #[doc = "Bits 1:4 - One-Shot pulse width"]
    #[inline(always)]
    pub fn ostwth(&mut self) -> OstwthW<'_, PcctlSpec> {
        OstwthW::new(self, 1)
    }
    #[doc = "Bits 5:7 - Chopping clock frequency"]
    #[inline(always)]
    pub fn chpfreq(&mut self) -> ChpfreqW<'_, PcctlSpec> {
        ChpfreqW::new(self, 5)
    }
    #[doc = "Bits 8:10 - Chopping clock duty cycle"]
    #[inline(always)]
    pub fn chpduty(&mut self) -> ChpdutyW<'_, PcctlSpec> {
        ChpdutyW::new(self, 8)
    }
}
#[doc = "PWM-Chopper Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcctlSpec;
impl crate::RegisterSpec for PcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcctl::R`](R) reader structure"]
impl crate::Readable for PcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcctl::W`](W) writer structure"]
impl crate::Writable for PcctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCCTL to value 0"]
impl crate::Resettable for PcctlSpec {}
