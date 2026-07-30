#[doc = "Register `HDCTL` reader"]
pub type R = crate::R<HdctlSpec>;
#[doc = "Register `HDCTL` writer"]
pub type W = crate::W<HdctlSpec>;
#[doc = "Action when hold detection A is invoked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hda {
    #[doc = "1: PWMA/PWMB go to 1 on failture"]
    Set = 1,
    #[doc = "2: PWMA/PWMB go to 0 on failture"]
    Clear = 2,
    #[doc = "3: no action on failture"]
    NoAction = 3,
}
impl From<Hda> for u8 {
    #[inline(always)]
    fn from(variant: Hda) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hda {
    type Ux = u8;
}
impl crate::IsEnum for Hda {}
#[doc = "Field `HDA` reader - Action when hold detection A is invoked"]
pub type HdaR = crate::FieldReader<Hda>;
impl HdaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hda> {
        match self.bits {
            1 => Some(Hda::Set),
            2 => Some(Hda::Clear),
            3 => Some(Hda::NoAction),
            _ => None,
        }
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Hda::Set
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Hda::Clear
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Hda::NoAction
    }
}
#[doc = "Field `HDA` writer - Action when hold detection A is invoked"]
pub type HdaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hda>;
impl<'a, REG> HdaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Hda::Set)
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Hda::Clear)
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Hda::NoAction)
    }
}
#[doc = "Action when hold detection B is invoked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hdb {
    #[doc = "1: PWMA/PWMB go to 1 on failture"]
    Set = 1,
    #[doc = "2: PWMA/PWMB go to 0 on failture"]
    Clear = 2,
    #[doc = "3: no action on failture"]
    NoAction = 3,
}
impl From<Hdb> for u8 {
    #[inline(always)]
    fn from(variant: Hdb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hdb {
    type Ux = u8;
}
impl crate::IsEnum for Hdb {}
#[doc = "Field `HDB` reader - Action when hold detection B is invoked"]
pub type HdbR = crate::FieldReader<Hdb>;
impl HdbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hdb> {
        match self.bits {
            1 => Some(Hdb::Set),
            2 => Some(Hdb::Clear),
            3 => Some(Hdb::NoAction),
            _ => None,
        }
    }
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Hdb::Set
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Hdb::Clear
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == Hdb::NoAction
    }
}
#[doc = "Field `HDB` writer - Action when hold detection B is invoked"]
pub type HdbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hdb>;
impl<'a, REG> HdbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWMA/PWMB go to 1 on failture"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Hdb::Set)
    }
    #[doc = "PWMA/PWMB go to 0 on failture"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Hdb::Clear)
    }
    #[doc = "no action on failture"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(Hdb::NoAction)
    }
}
#[doc = "Field `CBC` reader - Cycle-by-Cycle hold detector enable"]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - Cycle-by-Cycle hold detector enable"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` reader - One-Shot hold detector enable"]
pub type OstR = crate::BitReader;
#[doc = "Field `OST` writer - One-Shot hold detector enable"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Action when hold detection A is invoked"]
    #[inline(always)]
    pub fn hda(&self) -> HdaR {
        HdaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Action when hold detection B is invoked"]
    #[inline(always)]
    pub fn hdb(&self) -> HdbR {
        HdbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 8 - Cycle-by-Cycle hold detector enable"]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - One-Shot hold detector enable"]
    #[inline(always)]
    pub fn ost(&self) -> OstR {
        OstR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action when hold detection A is invoked"]
    #[inline(always)]
    pub fn hda(&mut self) -> HdaW<'_, HdctlSpec> {
        HdaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Action when hold detection B is invoked"]
    #[inline(always)]
    pub fn hdb(&mut self) -> HdbW<'_, HdctlSpec> {
        HdbW::new(self, 2)
    }
    #[doc = "Bit 8 - Cycle-by-Cycle hold detector enable"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, HdctlSpec> {
        CbcW::new(self, 8)
    }
    #[doc = "Bit 12 - One-Shot hold detector enable"]
    #[inline(always)]
    pub fn ost(&mut self) -> OstW<'_, HdctlSpec> {
        OstW::new(self, 12)
    }
}
#[doc = "Hold Detector Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdctlSpec;
impl crate::RegisterSpec for HdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdctl::R`](R) reader structure"]
impl crate::Readable for HdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hdctl::W`](W) writer structure"]
impl crate::Writable for HdctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HDCTL to value 0"]
impl crate::Resettable for HdctlSpec {}
