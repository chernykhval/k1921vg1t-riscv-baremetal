#[doc = "Register `PATDORL` reader"]
pub type R = crate::R<PatdorlSpec>;
#[doc = "Register `PATDORL` writer"]
pub type W = crate::W<PatdorlSpec>;
#[doc = "Pattern number 0 for pattern generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat0 {
    #[doc = "0: Triangle or SIN pattern data 0"]
    TriangleSin = 0,
    #[doc = "31: Meander pattern data 0"]
    Meander = 31,
}
impl From<Pat0> for u8 {
    #[inline(always)]
    fn from(variant: Pat0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat0 {
    type Ux = u8;
}
impl crate::IsEnum for Pat0 {}
#[doc = "Field `PAT0` reader - Pattern number 0 for pattern generator"]
pub type Pat0R = crate::FieldReader<Pat0>;
impl Pat0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat0> {
        match self.bits {
            0 => Some(Pat0::TriangleSin),
            31 => Some(Pat0::Meander),
            _ => None,
        }
    }
    #[doc = "Triangle or SIN pattern data 0"]
    #[inline(always)]
    pub fn is_triangle_sin(&self) -> bool {
        *self == Pat0::TriangleSin
    }
    #[doc = "Meander pattern data 0"]
    #[inline(always)]
    pub fn is_meander(&self) -> bool {
        *self == Pat0::Meander
    }
}
#[doc = "Field `PAT0` writer - Pattern number 0 for pattern generator"]
pub type Pat0W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat0>;
impl<'a, REG> Pat0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle or SIN pattern data 0"]
    #[inline(always)]
    pub fn triangle_sin(self) -> &'a mut crate::W<REG> {
        self.variant(Pat0::TriangleSin)
    }
    #[doc = "Meander pattern data 0"]
    #[inline(always)]
    pub fn meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat0::Meander)
    }
}
#[doc = "Pattern number 1 for pattern generator\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat1 {
    #[doc = "4: Triangle pattern data 1"]
    Triangle = 4,
    #[doc = "6: SIN pattern data 1"]
    Sin = 6,
    #[doc = "31: Meander pattern data 1"]
    Meander = 31,
}
impl From<Pat1> for u8 {
    #[inline(always)]
    fn from(variant: Pat1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat1 {
    type Ux = u8;
}
impl crate::IsEnum for Pat1 {}
#[doc = "Field `PAT1` reader - Pattern number 1 for pattern generator"]
pub type Pat1R = crate::FieldReader<Pat1>;
impl Pat1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat1> {
        match self.bits {
            4 => Some(Pat1::Triangle),
            6 => Some(Pat1::Sin),
            31 => Some(Pat1::Meander),
            _ => None,
        }
    }
    #[doc = "Triangle pattern data 1"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == Pat1::Triangle
    }
    #[doc = "SIN pattern data 1"]
    #[inline(always)]
    pub fn is_sin(&self) -> bool {
        *self == Pat1::Sin
    }
    #[doc = "Meander pattern data 1"]
    #[inline(always)]
    pub fn is_meander(&self) -> bool {
        *self == Pat1::Meander
    }
}
#[doc = "Field `PAT1` writer - Pattern number 1 for pattern generator"]
pub type Pat1W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat1>;
impl<'a, REG> Pat1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle pattern data 1"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(Pat1::Triangle)
    }
    #[doc = "SIN pattern data 1"]
    #[inline(always)]
    pub fn sin(self) -> &'a mut crate::W<REG> {
        self.variant(Pat1::Sin)
    }
    #[doc = "Meander pattern data 1"]
    #[inline(always)]
    pub fn meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat1::Meander)
    }
}
#[doc = "Pattern number 2 for pattern generator\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat2 {
    #[doc = "8: Triangle pattern data 2"]
    Triangle = 8,
    #[doc = "12: SIN pattern data 2"]
    Sin = 12,
    #[doc = "31: Meander pattern data 2"]
    Meander = 31,
}
impl From<Pat2> for u8 {
    #[inline(always)]
    fn from(variant: Pat2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat2 {
    type Ux = u8;
}
impl crate::IsEnum for Pat2 {}
#[doc = "Field `PAT2` reader - Pattern number 2 for pattern generator"]
pub type Pat2R = crate::FieldReader<Pat2>;
impl Pat2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat2> {
        match self.bits {
            8 => Some(Pat2::Triangle),
            12 => Some(Pat2::Sin),
            31 => Some(Pat2::Meander),
            _ => None,
        }
    }
    #[doc = "Triangle pattern data 2"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == Pat2::Triangle
    }
    #[doc = "SIN pattern data 2"]
    #[inline(always)]
    pub fn is_sin(&self) -> bool {
        *self == Pat2::Sin
    }
    #[doc = "Meander pattern data 2"]
    #[inline(always)]
    pub fn is_meander(&self) -> bool {
        *self == Pat2::Meander
    }
}
#[doc = "Field `PAT2` writer - Pattern number 2 for pattern generator"]
pub type Pat2W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat2>;
impl<'a, REG> Pat2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle pattern data 2"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(Pat2::Triangle)
    }
    #[doc = "SIN pattern data 2"]
    #[inline(always)]
    pub fn sin(self) -> &'a mut crate::W<REG> {
        self.variant(Pat2::Sin)
    }
    #[doc = "Meander pattern data 2"]
    #[inline(always)]
    pub fn meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat2::Meander)
    }
}
#[doc = "Pattern number 3 for pattern generator\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat3 {
    #[doc = "12: Triangle pattern data 3"]
    Triangle = 12,
    #[doc = "17: SIN pattern data 3"]
    Sin = 17,
    #[doc = "31: Meander pattern data 3"]
    Meander = 31,
}
impl From<Pat3> for u8 {
    #[inline(always)]
    fn from(variant: Pat3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat3 {
    type Ux = u8;
}
impl crate::IsEnum for Pat3 {}
#[doc = "Field `PAT3` reader - Pattern number 3 for pattern generator"]
pub type Pat3R = crate::FieldReader<Pat3>;
impl Pat3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat3> {
        match self.bits {
            12 => Some(Pat3::Triangle),
            17 => Some(Pat3::Sin),
            31 => Some(Pat3::Meander),
            _ => None,
        }
    }
    #[doc = "Triangle pattern data 3"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == Pat3::Triangle
    }
    #[doc = "SIN pattern data 3"]
    #[inline(always)]
    pub fn is_sin(&self) -> bool {
        *self == Pat3::Sin
    }
    #[doc = "Meander pattern data 3"]
    #[inline(always)]
    pub fn is_meander(&self) -> bool {
        *self == Pat3::Meander
    }
}
#[doc = "Field `PAT3` writer - Pattern number 3 for pattern generator"]
pub type Pat3W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat3>;
impl<'a, REG> Pat3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle pattern data 3"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(Pat3::Triangle)
    }
    #[doc = "SIN pattern data 3"]
    #[inline(always)]
    pub fn sin(self) -> &'a mut crate::W<REG> {
        self.variant(Pat3::Sin)
    }
    #[doc = "Meander pattern data 3"]
    #[inline(always)]
    pub fn meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat3::Meander)
    }
}
#[doc = "Pattern number 4 for pattern generator\n\nValue on reset: 22"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat4 {
    #[doc = "16: Triangle pattern data 4"]
    Triangle = 16,
    #[doc = "22: SIN pattern data 4"]
    Sin = 22,
    #[doc = "31: Meander pattern data 4"]
    Meander = 31,
}
impl From<Pat4> for u8 {
    #[inline(always)]
    fn from(variant: Pat4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat4 {
    type Ux = u8;
}
impl crate::IsEnum for Pat4 {}
#[doc = "Field `PAT4` reader - Pattern number 4 for pattern generator"]
pub type Pat4R = crate::FieldReader<Pat4>;
impl Pat4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat4> {
        match self.bits {
            16 => Some(Pat4::Triangle),
            22 => Some(Pat4::Sin),
            31 => Some(Pat4::Meander),
            _ => None,
        }
    }
    #[doc = "Triangle pattern data 4"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == Pat4::Triangle
    }
    #[doc = "SIN pattern data 4"]
    #[inline(always)]
    pub fn is_sin(&self) -> bool {
        *self == Pat4::Sin
    }
    #[doc = "Meander pattern data 4"]
    #[inline(always)]
    pub fn is_meander(&self) -> bool {
        *self == Pat4::Meander
    }
}
#[doc = "Field `PAT4` writer - Pattern number 4 for pattern generator"]
pub type Pat4W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat4>;
impl<'a, REG> Pat4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle pattern data 4"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(Pat4::Triangle)
    }
    #[doc = "SIN pattern data 4"]
    #[inline(always)]
    pub fn sin(self) -> &'a mut crate::W<REG> {
        self.variant(Pat4::Sin)
    }
    #[doc = "Meander pattern data 4"]
    #[inline(always)]
    pub fn meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat4::Meander)
    }
}
#[doc = "Pattern number 5 for pattern generator\n\nValue on reset: 26"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pat5 {
    #[doc = "19: Triangle pattern data 5"]
    Triangle = 19,
    #[doc = "26: SIN pattern data 5"]
    Sin = 26,
    #[doc = "31: Meander pattern data 5"]
    Meander = 31,
}
impl From<Pat5> for u8 {
    #[inline(always)]
    fn from(variant: Pat5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pat5 {
    type Ux = u8;
}
impl crate::IsEnum for Pat5 {}
#[doc = "Field `PAT5` reader - Pattern number 5 for pattern generator"]
pub type Pat5R = crate::FieldReader<Pat5>;
impl Pat5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pat5> {
        match self.bits {
            19 => Some(Pat5::Triangle),
            26 => Some(Pat5::Sin),
            31 => Some(Pat5::Meander),
            _ => None,
        }
    }
    #[doc = "Triangle pattern data 5"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == Pat5::Triangle
    }
    #[doc = "SIN pattern data 5"]
    #[inline(always)]
    pub fn is_sin(&self) -> bool {
        *self == Pat5::Sin
    }
    #[doc = "Meander pattern data 5"]
    #[inline(always)]
    pub fn is_meander(&self) -> bool {
        *self == Pat5::Meander
    }
}
#[doc = "Field `PAT5` writer - Pattern number 5 for pattern generator"]
pub type Pat5W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pat5>;
impl<'a, REG> Pat5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Triangle pattern data 5"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut crate::W<REG> {
        self.variant(Pat5::Triangle)
    }
    #[doc = "SIN pattern data 5"]
    #[inline(always)]
    pub fn sin(self) -> &'a mut crate::W<REG> {
        self.variant(Pat5::Sin)
    }
    #[doc = "Meander pattern data 5"]
    #[inline(always)]
    pub fn meander(self) -> &'a mut crate::W<REG> {
        self.variant(Pat5::Meander)
    }
}
impl R {
    #[doc = "Bits 0:4 - Pattern number 0 for pattern generator"]
    #[inline(always)]
    pub fn pat0(&self) -> Pat0R {
        Pat0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern number 1 for pattern generator"]
    #[inline(always)]
    pub fn pat1(&self) -> Pat1R {
        Pat1R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern number 2 for pattern generator"]
    #[inline(always)]
    pub fn pat2(&self) -> Pat2R {
        Pat2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Pattern number 3 for pattern generator"]
    #[inline(always)]
    pub fn pat3(&self) -> Pat3R {
        Pat3R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Pattern number 4 for pattern generator"]
    #[inline(always)]
    pub fn pat4(&self) -> Pat4R {
        Pat4R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Pattern number 5 for pattern generator"]
    #[inline(always)]
    pub fn pat5(&self) -> Pat5R {
        Pat5R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern number 0 for pattern generator"]
    #[inline(always)]
    pub fn pat0(&mut self) -> Pat0W<'_, PatdorlSpec> {
        Pat0W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Pattern number 1 for pattern generator"]
    #[inline(always)]
    pub fn pat1(&mut self) -> Pat1W<'_, PatdorlSpec> {
        Pat1W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Pattern number 2 for pattern generator"]
    #[inline(always)]
    pub fn pat2(&mut self) -> Pat2W<'_, PatdorlSpec> {
        Pat2W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Pattern number 3 for pattern generator"]
    #[inline(always)]
    pub fn pat3(&mut self) -> Pat3W<'_, PatdorlSpec> {
        Pat3W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Pattern number 4 for pattern generator"]
    #[inline(always)]
    pub fn pat4(&mut self) -> Pat4W<'_, PatdorlSpec> {
        Pat4W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Pattern number 5 for pattern generator"]
    #[inline(always)]
    pub fn pat5(&mut self) -> Pat5W<'_, PatdorlSpec> {
        Pat5W::new(self, 25)
    }
}
#[doc = "Pattern data output register (low part)\n\nYou can [`read`](crate::Reg::read) this register and get [`patdorl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patdorl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PatdorlSpec;
impl crate::RegisterSpec for PatdorlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patdorl::R`](R) reader structure"]
impl crate::Readable for PatdorlSpec {}
#[doc = "`write(|w| ..)` method takes [`patdorl::W`](W) writer structure"]
impl crate::Writable for PatdorlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PATDORL to value 0x3568_b0c0"]
impl crate::Resettable for PatdorlSpec {
    const RESET_VALUE: u32 = 0x3568_b0c0;
}
