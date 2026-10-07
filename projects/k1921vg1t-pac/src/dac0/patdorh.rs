#[doc = "Register `PATDORH` reader"]
pub type R = crate::R<PatdorhSpec>;
#[doc = "Register `PATDORH` writer"]
pub type W = crate::W<PatdorhSpec>;
#[doc = "Pattern number 6 for pattern generator\n\nValue on reset: 29"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat6 {
    #[doc = "23: Triangle pattern data 6"]
    Triangle = 23,
    #[doc = "29: SIN pattern data 6"]
    Sin = 29,
    #[doc = "31: Meander pattern data 6"]
    Meander = 31,
}
impl From<Pat6> for u8 {
    #[inline(always)]
    fn from(variant: Pat6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat6 {
    type Ux = u8;
}
impl crate::IsEnum for Pat6 {}
#[doc = "Field `PAT6` reader - Pattern number 6 for pattern generator"]
pub type Pat6R = crate::FieldReader<Pat6>;
impl Pat6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat6> {
        match self.bits {
            23 => Some(Pat6::Triangle),
            29 => Some(Pat6::Sin),
            31 => Some(Pat6::Meander),
            _ => None,
        }
    }
    #[doc = "Triangle pattern data 6"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == Pat6::Triangle
    }
    #[doc = "SIN pattern data 6"]
    #[inline(always)]
    pub fn is_sin(&self) -> bool {
        *self == Pat6::Sin
    }
    #[doc = "Meander pattern data 6"]
    #[inline(always)]
    pub fn is_meander(&self) -> bool {
        *self == Pat6::Meander
    }
}
#[doc = "Field `PAT6` writer - Pattern number 6 for pattern generator"]
pub type Pat6W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat6>;
impl<'a, REG> Pat6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle pattern data 6"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(Pat6::Triangle)
    }
    #[doc = "SIN pattern data 6"]
    #[inline(always)]
    pub fn sin(self) -> &'a mut crate::W<REG> {
        self.variant(Pat6::Sin)
    }
    #[doc = "Meander pattern data 6"]
    #[inline(always)]
    pub fn meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat6::Meander)
    }
}
#[doc = "Pattern number 7 for pattern generator\n\nValue on reset: 30"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat7 {
    #[doc = "27: Triangle pattern data 7"]
    Triangle = 27,
    #[doc = "30: SIN pattern data 7"]
    Sin = 30,
    #[doc = "31: Meander pattern data 7"]
    Meander = 31,
}
impl From<Pat7> for u8 {
    #[inline(always)]
    fn from(variant: Pat7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat7 {
    type Ux = u8;
}
impl crate::IsEnum for Pat7 {}
#[doc = "Field `PAT7` reader - Pattern number 7 for pattern generator"]
pub type Pat7R = crate::FieldReader<Pat7>;
impl Pat7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat7> {
        match self.bits {
            27 => Some(Pat7::Triangle),
            30 => Some(Pat7::Sin),
            31 => Some(Pat7::Meander),
            _ => None,
        }
    }
    #[doc = "Triangle pattern data 7"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == Pat7::Triangle
    }
    #[doc = "SIN pattern data 7"]
    #[inline(always)]
    pub fn is_sin(&self) -> bool {
        *self == Pat7::Sin
    }
    #[doc = "Meander pattern data 7"]
    #[inline(always)]
    pub fn is_meander(&self) -> bool {
        *self == Pat7::Meander
    }
}
#[doc = "Field `PAT7` writer - Pattern number 7 for pattern generator"]
pub type Pat7W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat7>;
impl<'a, REG> Pat7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle pattern data 7"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(Pat7::Triangle)
    }
    #[doc = "SIN pattern data 7"]
    #[inline(always)]
    pub fn sin(self) -> &'a mut crate::W<REG> {
        self.variant(Pat7::Sin)
    }
    #[doc = "Meander pattern data 7"]
    #[inline(always)]
    pub fn meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat7::Meander)
    }
}
#[doc = "Pattern number 8 for pattern generator\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat8 {
    #[doc = "31: Triangle, SIN or meander pattern data 8"]
    TriangleSinMeander = 31,
}
impl From<Pat8> for u8 {
    #[inline(always)]
    fn from(variant: Pat8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat8 {
    type Ux = u8;
}
impl crate::IsEnum for Pat8 {}
#[doc = "Field `PAT8` reader - Pattern number 8 for pattern generator"]
pub type Pat8R = crate::FieldReader<Pat8>;
impl Pat8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat8> {
        match self.bits {
            31 => Some(Pat8::TriangleSinMeander),
            _ => None,
        }
    }
    #[doc = "Triangle, SIN or meander pattern data 8"]
    #[inline(always)]
    pub fn is_triangle_sin_meander(&self) -> bool {
        *self == Pat8::TriangleSinMeander
    }
}
#[doc = "Field `PAT8` writer - Pattern number 8 for pattern generator"]
pub type Pat8W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat8>;
impl<'a, REG> Pat8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle, SIN or meander pattern data 8"]
    #[inline(always)]
    pub fn triangle_sin_meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat8::TriangleSinMeander)
    }
}
impl R {
    #[doc = "Bits 0:4 - Pattern number 6 for pattern generator"]
    #[inline(always)]
    pub fn pat6(&self) -> Pat6R {
        Pat6R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern number 7 for pattern generator"]
    #[inline(always)]
    pub fn pat7(&self) -> Pat7R {
        Pat7R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern number 8 for pattern generator"]
    #[inline(always)]
    pub fn pat8(&self) -> Pat8R {
        Pat8R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern number 6 for pattern generator"]
    #[inline(always)]
    pub fn pat6(&mut self) -> Pat6W<'_, PatdorhSpec> {
        Pat6W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Pattern number 7 for pattern generator"]
    #[inline(always)]
    pub fn pat7(&mut self) -> Pat7W<'_, PatdorhSpec> {
        Pat7W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Pattern number 8 for pattern generator"]
    #[inline(always)]
    pub fn pat8(&mut self) -> Pat8W<'_, PatdorhSpec> {
        Pat8W::new(self, 10)
    }
}
#[doc = "Pattern data output register (high part)\n\nYou can [`read`](crate::Reg::read) this register and get [`patdorh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patdorh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PatdorhSpec;
impl crate::RegisterSpec for PatdorhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patdorh::R`](R) reader structure"]
impl crate::Readable for PatdorhSpec {}
#[doc = "`write(|w| ..)` method takes [`patdorh::W`](W) writer structure"]
impl crate::Writable for PatdorhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PATDORH to value 0x7fdd"]
impl crate::Resettable for PatdorhSpec {
    const RESET_VALUE: u32 = 0x7fdd;
}
