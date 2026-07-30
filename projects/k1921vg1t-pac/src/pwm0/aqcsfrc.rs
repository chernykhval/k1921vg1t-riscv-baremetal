#[doc = "Register `AQCSFRC` reader"]
pub type R = crate::R<AqcsfrcSpec>;
#[doc = "Register `AQCSFRC` writer"]
pub type W = crate::W<AqcsfrcSpec>;
#[doc = "Continuous software force on output A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csfa {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
}
impl From<Csfa> for u8 {
    #[inline(always)]
    fn from(variant: Csfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csfa {
    type Ux = u8;
}
impl crate::IsEnum for Csfa {}
#[doc = "Field `CSFA` reader - Continuous software force on output A"]
pub type CsfaR = crate::FieldReader<Csfa>;
impl CsfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csfa> {
        match self.bits {
            0 => Some(Csfa::NoAction),
            1 => Some(Csfa::Clear),
            2 => Some(Csfa::Set),
            _ => None,
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Csfa::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Csfa::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Csfa::Set
    }
}
#[doc = "Field `CSFA` writer - Continuous software force on output A"]
pub type CsfaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csfa>;
impl<'a, REG> CsfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Csfa::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Csfa::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Csfa::Set)
    }
}
#[doc = "Continuous software force on output B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csfb {
    #[doc = "0: no action"]
    NoAction = 0,
    #[doc = "1: clear PWMA/PWMB"]
    Clear = 1,
    #[doc = "2: set PWMA/PWMB"]
    Set = 2,
}
impl From<Csfb> for u8 {
    #[inline(always)]
    fn from(variant: Csfb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csfb {
    type Ux = u8;
}
impl crate::IsEnum for Csfb {}
#[doc = "Field `CSFB` reader - Continuous software force on output B"]
pub type CsfbR = crate::FieldReader<Csfb>;
impl CsfbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csfb> {
        match self.bits {
            0 => Some(Csfb::NoAction),
            1 => Some(Csfb::Clear),
            2 => Some(Csfb::Set),
            _ => None,
        }
    }
    #[doc = "no action"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Csfb::NoAction
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Csfb::Clear
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Csfb::Set
    }
}
#[doc = "Field `CSFB` writer - Continuous software force on output B"]
pub type CsfbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csfb>;
impl<'a, REG> CsfbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Csfb::NoAction)
    }
    #[doc = "clear PWMA/PWMB"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Csfb::Clear)
    }
    #[doc = "set PWMA/PWMB"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Csfb::Set)
    }
}
impl R {
    #[doc = "Bits 0:1 - Continuous software force on output A"]
    #[inline(always)]
    pub fn csfa(&self) -> CsfaR {
        CsfaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Continuous software force on output B"]
    #[inline(always)]
    pub fn csfb(&self) -> CsfbR {
        CsfbR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Continuous software force on output A"]
    #[inline(always)]
    pub fn csfa(&mut self) -> CsfaW<'_, AqcsfrcSpec> {
        CsfaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Continuous software force on output B"]
    #[inline(always)]
    pub fn csfb(&mut self) -> CsfbW<'_, AqcsfrcSpec> {
        CsfbW::new(self, 2)
    }
}
#[doc = "Action-Qualifier Continuous Software Force Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aqcsfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aqcsfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AqcsfrcSpec;
impl crate::RegisterSpec for AqcsfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aqcsfrc::R`](R) reader structure"]
impl crate::Readable for AqcsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`aqcsfrc::W`](W) writer structure"]
impl crate::Writable for AqcsfrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AQCSFRC to value 0"]
impl crate::Resettable for AqcsfrcSpec {}
