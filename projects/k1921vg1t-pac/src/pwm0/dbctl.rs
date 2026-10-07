#[doc = "Register `DBCTL` reader"]
pub type R = crate::R<DbctlSpec>;
#[doc = "Register `DBCTL` writer"]
pub type W = crate::W<DbctlSpec>;
#[doc = "Dead-band output mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outmode {
    #[doc = "0: edge for deadtime is no specified"]
    NoSpec = 0,
    #[doc = "1: deadtime on PWMB negedge"]
    Bneg = 1,
    #[doc = "2: deadtime on PWMA posedge"]
    Apos = 2,
    #[doc = "3: deadtime on PWMA posedge and PWMB negedge"]
    AposBneg = 3,
}
impl From<Outmode> for u8 {
    #[inline(always)]
    fn from(variant: Outmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outmode {
    type Ux = u8;
}
impl crate::IsEnum for Outmode {}
#[doc = "Field `OUTMODE` reader - Dead-band output mode control"]
pub type OutmodeR = crate::FieldReader<Outmode>;
impl OutmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outmode {
        match self.bits {
            0 => Outmode::NoSpec,
            1 => Outmode::Bneg,
            2 => Outmode::Apos,
            3 => Outmode::AposBneg,
            _ => unreachable!(),
        }
    }
    #[doc = "edge for deadtime is no specified"]
    #[inline(always)]
    pub fn is_no_spec(&self) -> bool {
        *self == Outmode::NoSpec
    }
    #[doc = "deadtime on PWMB negedge"]
    #[inline(always)]
    pub fn is_bneg(&self) -> bool {
        *self == Outmode::Bneg
    }
    #[doc = "deadtime on PWMA posedge"]
    #[inline(always)]
    pub fn is_apos(&self) -> bool {
        *self == Outmode::Apos
    }
    #[doc = "deadtime on PWMA posedge and PWMB negedge"]
    #[inline(always)]
    pub fn is_apos_bneg(&self) -> bool {
        *self == Outmode::AposBneg
    }
}
#[doc = "Field `OUTMODE` writer - Dead-band output mode control"]
pub type OutmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Outmode, crate::Safe>;
impl<'a, REG> OutmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "edge for deadtime is no specified"]
    #[inline(always)]
    pub fn no_spec(self) -> &'a mut crate::W<REG> {
        self.variant(Outmode::NoSpec)
    }
    #[doc = "deadtime on PWMB negedge"]
    #[inline(always)]
    pub fn bneg(self) -> &'a mut crate::W<REG> {
        self.variant(Outmode::Bneg)
    }
    #[doc = "deadtime on PWMA posedge"]
    #[inline(always)]
    pub fn apos(self) -> &'a mut crate::W<REG> {
        self.variant(Outmode::Apos)
    }
    #[doc = "deadtime on PWMA posedge and PWMB negedge"]
    #[inline(always)]
    pub fn apos_bneg(self) -> &'a mut crate::W<REG> {
        self.variant(Outmode::AposBneg)
    }
}
#[doc = "Polarity select control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polsel {
    #[doc = "0: inverse disabled"]
    InvDisable = 0,
    #[doc = "1: inverse on PWMA"]
    InvA = 1,
    #[doc = "2: inverse on PWMB"]
    InvB = 2,
    #[doc = "3: inverse on PWMA and PWMB"]
    InvAb = 3,
}
impl From<Polsel> for u8 {
    #[inline(always)]
    fn from(variant: Polsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polsel {
    type Ux = u8;
}
impl crate::IsEnum for Polsel {}
#[doc = "Field `POLSEL` reader - Polarity select control"]
pub type PolselR = crate::FieldReader<Polsel>;
impl PolselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polsel {
        match self.bits {
            0 => Polsel::InvDisable,
            1 => Polsel::InvA,
            2 => Polsel::InvB,
            3 => Polsel::InvAb,
            _ => unreachable!(),
        }
    }
    #[doc = "inverse disabled"]
    #[inline(always)]
    pub fn is_inv_disable(&self) -> bool {
        *self == Polsel::InvDisable
    }
    #[doc = "inverse on PWMA"]
    #[inline(always)]
    pub fn is_inv_a(&self) -> bool {
        *self == Polsel::InvA
    }
    #[doc = "inverse on PWMB"]
    #[inline(always)]
    pub fn is_inv_b(&self) -> bool {
        *self == Polsel::InvB
    }
    #[doc = "inverse on PWMA and PWMB"]
    #[inline(always)]
    pub fn is_inv_ab(&self) -> bool {
        *self == Polsel::InvAb
    }
}
#[doc = "Field `POLSEL` writer - Polarity select control"]
pub type PolselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Polsel, crate::Safe>;
impl<'a, REG> PolselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "inverse disabled"]
    #[inline(always)]
    pub fn inv_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polsel::InvDisable)
    }
    #[doc = "inverse on PWMA"]
    #[inline(always)]
    pub fn inv_a(self) -> &'a mut crate::W<REG> {
        self.variant(Polsel::InvA)
    }
    #[doc = "inverse on PWMB"]
    #[inline(always)]
    pub fn inv_b(self) -> &'a mut crate::W<REG> {
        self.variant(Polsel::InvB)
    }
    #[doc = "inverse on PWMA and PWMB"]
    #[inline(always)]
    pub fn inv_ab(self) -> &'a mut crate::W<REG> {
        self.variant(Polsel::InvAb)
    }
}
#[doc = "Dead band input mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inmode {
    #[doc = "0: PWMA is used for posedge and negedge control"]
    AposNeg = 0,
    #[doc = "1: PWMA is used for negedge and PWMB is used for posedge control"]
    AnegBpos = 1,
    #[doc = "2: PWMA is used for posedge and PWMB is used for negedge control"]
    AposBneg = 2,
    #[doc = "3: PWMB is used for posedge and negedge control"]
    BposNeg = 3,
}
impl From<Inmode> for u8 {
    #[inline(always)]
    fn from(variant: Inmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inmode {
    type Ux = u8;
}
impl crate::IsEnum for Inmode {}
#[doc = "Field `INMODE` reader - Dead band input mode control"]
pub type InmodeR = crate::FieldReader<Inmode>;
impl InmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inmode {
        match self.bits {
            0 => Inmode::AposNeg,
            1 => Inmode::AnegBpos,
            2 => Inmode::AposBneg,
            3 => Inmode::BposNeg,
            _ => unreachable!(),
        }
    }
    #[doc = "PWMA is used for posedge and negedge control"]
    #[inline(always)]
    pub fn is_apos_neg(&self) -> bool {
        *self == Inmode::AposNeg
    }
    #[doc = "PWMA is used for negedge and PWMB is used for posedge control"]
    #[inline(always)]
    pub fn is_aneg_bpos(&self) -> bool {
        *self == Inmode::AnegBpos
    }
    #[doc = "PWMA is used for posedge and PWMB is used for negedge control"]
    #[inline(always)]
    pub fn is_apos_bneg(&self) -> bool {
        *self == Inmode::AposBneg
    }
    #[doc = "PWMB is used for posedge and negedge control"]
    #[inline(always)]
    pub fn is_bpos_neg(&self) -> bool {
        *self == Inmode::BposNeg
    }
}
#[doc = "Field `INMODE` writer - Dead band input mode control"]
pub type InmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Inmode, crate::Safe>;
impl<'a, REG> InmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWMA is used for posedge and negedge control"]
    #[inline(always)]
    pub fn apos_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Inmode::AposNeg)
    }
    #[doc = "PWMA is used for negedge and PWMB is used for posedge control"]
    #[inline(always)]
    pub fn aneg_bpos(self) -> &'a mut crate::W<REG> {
        self.variant(Inmode::AnegBpos)
    }
    #[doc = "PWMA is used for posedge and PWMB is used for negedge control"]
    #[inline(always)]
    pub fn apos_bneg(self) -> &'a mut crate::W<REG> {
        self.variant(Inmode::AposBneg)
    }
    #[doc = "PWMB is used for posedge and negedge control"]
    #[inline(always)]
    pub fn bpos_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Inmode::BposNeg)
    }
}
impl R {
    #[doc = "Bits 0:1 - Dead-band output mode control"]
    #[inline(always)]
    pub fn outmode(&self) -> OutmodeR {
        OutmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Polarity select control"]
    #[inline(always)]
    pub fn polsel(&self) -> PolselR {
        PolselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Dead band input mode control"]
    #[inline(always)]
    pub fn inmode(&self) -> InmodeR {
        InmodeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Dead-band output mode control"]
    #[inline(always)]
    pub fn outmode(&mut self) -> OutmodeW<'_, DbctlSpec> {
        OutmodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Polarity select control"]
    #[inline(always)]
    pub fn polsel(&mut self) -> PolselW<'_, DbctlSpec> {
        PolselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Dead band input mode control"]
    #[inline(always)]
    pub fn inmode(&mut self) -> InmodeW<'_, DbctlSpec> {
        InmodeW::new(self, 4)
    }
}
#[doc = "Dead-Band Generator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbctlSpec;
impl crate::RegisterSpec for DbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbctl::R`](R) reader structure"]
impl crate::Readable for DbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbctl::W`](W) writer structure"]
impl crate::Writable for DbctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBCTL to value 0"]
impl crate::Resettable for DbctlSpec {}
