#[doc = "Register `AQCTLB` reader"]
pub type R = crate::R<AqctlbSpec>;
#[doc = "Register `AQCTLB` writer"]
pub type W = crate::W<AqctlbSpec>;
#[doc = "Action when counter equals zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Zro {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Zro> for u8 {
    #[inline(always)]
    fn from(variant: Zro) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Zro {
    type Ux = u8;
}
impl crate::IsEnum for Zro {}
#[doc = "Field `ZRO` reader - Action when counter equals zero"]
pub type ZroR = crate::FieldReader<Zro>;
impl ZroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zro {
        match self.bits {
            0 => Zro::NoAction,
            1 => Zro::Clear,
            2 => Zro::Set,
            3 => Zro::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Zro::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Zro::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Zro::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Zro::Toggle
    }
}
#[doc = "Field `ZRO` writer - Action when counter equals zero"]
pub type ZroW<'a, REG> = crate::FieldWriter<'a, REG, 2, Zro, crate::Safe>;
impl<'a, REG> ZroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Zro::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Zro::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Zro::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Zro::Toggle)
    }
}
#[doc = "Action when the counter equals the period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prd {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Prd> for u8 {
    #[inline(always)]
    fn from(variant: Prd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prd {
    type Ux = u8;
}
impl crate::IsEnum for Prd {}
#[doc = "Field `PRD` reader - Action when the counter equals the period"]
pub type PrdR = crate::FieldReader<Prd>;
impl PrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prd {
        match self.bits {
            0 => Prd::NoAction,
            1 => Prd::Clear,
            2 => Prd::Set,
            3 => Prd::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Prd::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Prd::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Prd::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Prd::Toggle
    }
}
#[doc = "Field `PRD` writer - Action when the counter equals the period"]
pub type PrdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prd, crate::Safe>;
impl<'a, REG> PrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Prd::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Prd::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Prd::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Prd::Toggle)
    }
}
#[doc = "Action when the counter equals the active CMPA register and the counter is incrementing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cau {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Cau> for u8 {
    #[inline(always)]
    fn from(variant: Cau) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cau {
    type Ux = u8;
}
impl crate::IsEnum for Cau {}
#[doc = "Field `CAU` reader - Action when the counter equals the active CMPA register and the counter is incrementing"]
pub type CauR = crate::FieldReader<Cau>;
impl CauR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cau {
        match self.bits {
            0 => Cau::NoAction,
            1 => Cau::Clear,
            2 => Cau::Set,
            3 => Cau::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Cau::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cau::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cau::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cau::Toggle
    }
}
#[doc = "Field `CAU` writer - Action when the counter equals the active CMPA register and the counter is incrementing"]
pub type CauW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cau, crate::Safe>;
impl<'a, REG> CauW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Cau::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cau::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cau::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cau::Toggle)
    }
}
#[doc = "Action when the counter equals the active CMPA register and the counter is decrementing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cad {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Cad> for u8 {
    #[inline(always)]
    fn from(variant: Cad) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cad {
    type Ux = u8;
}
impl crate::IsEnum for Cad {}
#[doc = "Field `CAD` reader - Action when the counter equals the active CMPA register and the counter is decrementing"]
pub type CadR = crate::FieldReader<Cad>;
impl CadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cad {
        match self.bits {
            0 => Cad::NoAction,
            1 => Cad::Clear,
            2 => Cad::Set,
            3 => Cad::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Cad::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cad::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cad::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cad::Toggle
    }
}
#[doc = "Field `CAD` writer - Action when the counter equals the active CMPA register and the counter is decrementing"]
pub type CadW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cad, crate::Safe>;
impl<'a, REG> CadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Cad::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cad::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cad::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cad::Toggle)
    }
}
#[doc = "Action when the counter equals the active CMPB register and the counter is incrementing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cbu {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Cbu> for u8 {
    #[inline(always)]
    fn from(variant: Cbu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cbu {
    type Ux = u8;
}
impl crate::IsEnum for Cbu {}
#[doc = "Field `CBU` reader - Action when the counter equals the active CMPB register and the counter is incrementing"]
pub type CbuR = crate::FieldReader<Cbu>;
impl CbuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbu {
        match self.bits {
            0 => Cbu::NoAction,
            1 => Cbu::Clear,
            2 => Cbu::Set,
            3 => Cbu::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Cbu::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cbu::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cbu::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cbu::Toggle
    }
}
#[doc = "Field `CBU` writer - Action when the counter equals the active CMPB register and the counter is incrementing"]
pub type CbuW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cbu, crate::Safe>;
impl<'a, REG> CbuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Cbu::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cbu::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cbu::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cbu::Toggle)
    }
}
#[doc = "Action when the time-base counter equals the active CMPB register and the counter is decrementing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cbd {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
    #[doc = "3: inverse PWMA/PWMB"]
    Toggle = 3,
}
impl From<Cbd> for u8 {
    #[inline(always)]
    fn from(variant: Cbd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cbd {
    type Ux = u8;
}
impl crate::IsEnum for Cbd {}
#[doc = "Field `CBD` reader - Action when the time-base counter equals the active CMPB register and the counter is decrementing"]
pub type CbdR = crate::FieldReader<Cbd>;
impl CbdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cbd {
        match self.bits {
            0 => Cbd::NoAction,
            1 => Cbd::Clear,
            2 => Cbd::Set,
            3 => Cbd::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Cbd::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cbd::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cbd::Set
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cbd::Toggle
    }
}
#[doc = "Field `CBD` writer - Action when the time-base counter equals the active CMPB register and the counter is decrementing"]
pub type CbdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cbd, crate::Safe>;
impl<'a, REG> CbdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Cbd::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cbd::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cbd::Set)
    }
    #[doc = "inverse PWMA/PWMB"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cbd::Toggle)
    }
}
impl R {
    #[doc = "Bits 0:1 - Action when counter equals zero"]
    #[inline(always)]
    pub fn zro(&self) -> ZroR {
        ZroR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Action when the counter equals the period"]
    #[inline(always)]
    pub fn prd(&self) -> PrdR {
        PrdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Action when the counter equals the active CMPA register and the counter is incrementing"]
    #[inline(always)]
    pub fn cau(&self) -> CauR {
        CauR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Action when the counter equals the active CMPA register and the counter is decrementing"]
    #[inline(always)]
    pub fn cad(&self) -> CadR {
        CadR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Action when the counter equals the active CMPB register and the counter is incrementing"]
    #[inline(always)]
    pub fn cbu(&self) -> CbuR {
        CbuR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Action when the time-base counter equals the active CMPB register and the counter is decrementing"]
    #[inline(always)]
    pub fn cbd(&self) -> CbdR {
        CbdR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action when counter equals zero"]
    #[inline(always)]
    pub fn zro(&mut self) -> ZroW<'_, AqctlbSpec> {
        ZroW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Action when the counter equals the period"]
    #[inline(always)]
    pub fn prd(&mut self) -> PrdW<'_, AqctlbSpec> {
        PrdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Action when the counter equals the active CMPA register and the counter is incrementing"]
    #[inline(always)]
    pub fn cau(&mut self) -> CauW<'_, AqctlbSpec> {
        CauW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Action when the counter equals the active CMPA register and the counter is decrementing"]
    #[inline(always)]
    pub fn cad(&mut self) -> CadW<'_, AqctlbSpec> {
        CadW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Action when the counter equals the active CMPB register and the counter is incrementing"]
    #[inline(always)]
    pub fn cbu(&mut self) -> CbuW<'_, AqctlbSpec> {
        CbuW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Action when the time-base counter equals the active CMPB register and the counter is decrementing"]
    #[inline(always)]
    pub fn cbd(&mut self) -> CbdW<'_, AqctlbSpec> {
        CbdW::new(self, 10)
    }
}
#[doc = "Action-Qualifier Output B Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aqctlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aqctlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AqctlbSpec;
impl crate::RegisterSpec for AqctlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aqctlb::R`](R) reader structure"]
impl crate::Readable for AqctlbSpec {}
#[doc = "`write(|w| ..)` method takes [`aqctlb::W`](W) writer structure"]
impl crate::Writable for AqctlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AQCTLB to value 0"]
impl crate::Resettable for AqctlbSpec {}
