#[doc = "Register `ALTFUNCNUM0` reader"]
pub type R = crate::R<Altfuncnum0Spec>;
#[doc = "Register `ALTFUNCNUM0` writer"]
pub type W = crate::W<Altfuncnum0Spec>;
#[doc = "Select altfunc number for pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin0 {
    #[doc = "0: Altfunc disabled"]
    Disable = 0,
    #[doc = "1: Altfunc 1 enable"]
    Af1 = 1,
    #[doc = "2: Altfunc 2 enable"]
    Af2 = 2,
    #[doc = "3: Altfunc 3 enable"]
    Af3 = 3,
    #[doc = "4: Altfunc 4 enable"]
    Af4 = 4,
    #[doc = "5: Altfunc 5 enable"]
    Af5 = 5,
}
impl From<Pin0> for u8 {
    #[inline(always)]
    fn from(variant: Pin0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin0 {
    type Ux = u8;
}
impl crate::IsEnum for Pin0 {}
#[doc = "Field `PIN0` reader - Select altfunc number for pin 0"]
pub type Pin0R = crate::FieldReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin0> {
        match self.bits {
            0 => Some(Pin0::Disable),
            1 => Some(Pin0::Af1),
            2 => Some(Pin0::Af2),
            3 => Some(Pin0::Af3),
            4 => Some(Pin0::Af4),
            5 => Some(Pin0::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin0::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin0::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin0::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin0::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin0::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin0::Af5
    }
}
#[doc = "Field `PIN0` writer - Select altfunc number for pin 0"]
pub type Pin0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin0>;
impl<'a, REG> Pin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Af5)
    }
}
#[doc = "Select altfunc number for pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin1 {
    #[doc = "0: Altfunc disabled"]
    Disable = 0,
    #[doc = "1: Altfunc 1 enable"]
    Af1 = 1,
    #[doc = "2: Altfunc 2 enable"]
    Af2 = 2,
    #[doc = "3: Altfunc 3 enable"]
    Af3 = 3,
    #[doc = "4: Altfunc 4 enable"]
    Af4 = 4,
    #[doc = "5: Altfunc 5 enable"]
    Af5 = 5,
}
impl From<Pin1> for u8 {
    #[inline(always)]
    fn from(variant: Pin1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin1 {
    type Ux = u8;
}
impl crate::IsEnum for Pin1 {}
#[doc = "Field `PIN1` reader - Select altfunc number for pin 1"]
pub type Pin1R = crate::FieldReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin1> {
        match self.bits {
            0 => Some(Pin1::Disable),
            1 => Some(Pin1::Af1),
            2 => Some(Pin1::Af2),
            3 => Some(Pin1::Af3),
            4 => Some(Pin1::Af4),
            5 => Some(Pin1::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin1::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin1::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin1::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin1::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin1::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin1::Af5
    }
}
#[doc = "Field `PIN1` writer - Select altfunc number for pin 1"]
pub type Pin1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin1>;
impl<'a, REG> Pin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Af5)
    }
}
#[doc = "Select altfunc number for pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin2 {
    #[doc = "0: Altfunc disabled"]
    Disable = 0,
    #[doc = "1: Altfunc 1 enable"]
    Af1 = 1,
    #[doc = "2: Altfunc 2 enable"]
    Af2 = 2,
    #[doc = "3: Altfunc 3 enable"]
    Af3 = 3,
    #[doc = "4: Altfunc 4 enable"]
    Af4 = 4,
    #[doc = "5: Altfunc 5 enable"]
    Af5 = 5,
}
impl From<Pin2> for u8 {
    #[inline(always)]
    fn from(variant: Pin2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin2 {
    type Ux = u8;
}
impl crate::IsEnum for Pin2 {}
#[doc = "Field `PIN2` reader - Select altfunc number for pin 2"]
pub type Pin2R = crate::FieldReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin2> {
        match self.bits {
            0 => Some(Pin2::Disable),
            1 => Some(Pin2::Af1),
            2 => Some(Pin2::Af2),
            3 => Some(Pin2::Af3),
            4 => Some(Pin2::Af4),
            5 => Some(Pin2::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin2::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin2::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin2::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin2::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin2::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin2::Af5
    }
}
#[doc = "Field `PIN2` writer - Select altfunc number for pin 2"]
pub type Pin2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin2>;
impl<'a, REG> Pin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Af5)
    }
}
#[doc = "Select altfunc number for pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin3 {
    #[doc = "0: Altfunc disabled"]
    Disable = 0,
    #[doc = "1: Altfunc 1 enable"]
    Af1 = 1,
    #[doc = "2: Altfunc 2 enable"]
    Af2 = 2,
    #[doc = "3: Altfunc 3 enable"]
    Af3 = 3,
    #[doc = "4: Altfunc 4 enable"]
    Af4 = 4,
    #[doc = "5: Altfunc 5 enable"]
    Af5 = 5,
}
impl From<Pin3> for u8 {
    #[inline(always)]
    fn from(variant: Pin3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin3 {
    type Ux = u8;
}
impl crate::IsEnum for Pin3 {}
#[doc = "Field `PIN3` reader - Select altfunc number for pin 3"]
pub type Pin3R = crate::FieldReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin3> {
        match self.bits {
            0 => Some(Pin3::Disable),
            1 => Some(Pin3::Af1),
            2 => Some(Pin3::Af2),
            3 => Some(Pin3::Af3),
            4 => Some(Pin3::Af4),
            5 => Some(Pin3::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin3::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin3::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin3::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin3::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin3::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin3::Af5
    }
}
#[doc = "Field `PIN3` writer - Select altfunc number for pin 3"]
pub type Pin3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin3>;
impl<'a, REG> Pin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Af5)
    }
}
#[doc = "Select altfunc number for pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin4 {
    #[doc = "0: Altfunc disabled"]
    Disable = 0,
    #[doc = "1: Altfunc 1 enable"]
    Af1 = 1,
    #[doc = "2: Altfunc 2 enable"]
    Af2 = 2,
    #[doc = "3: Altfunc 3 enable"]
    Af3 = 3,
    #[doc = "4: Altfunc 4 enable"]
    Af4 = 4,
    #[doc = "5: Altfunc 5 enable"]
    Af5 = 5,
}
impl From<Pin4> for u8 {
    #[inline(always)]
    fn from(variant: Pin4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin4 {
    type Ux = u8;
}
impl crate::IsEnum for Pin4 {}
#[doc = "Field `PIN4` reader - Select altfunc number for pin 4"]
pub type Pin4R = crate::FieldReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin4> {
        match self.bits {
            0 => Some(Pin4::Disable),
            1 => Some(Pin4::Af1),
            2 => Some(Pin4::Af2),
            3 => Some(Pin4::Af3),
            4 => Some(Pin4::Af4),
            5 => Some(Pin4::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin4::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin4::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin4::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin4::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin4::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin4::Af5
    }
}
#[doc = "Field `PIN4` writer - Select altfunc number for pin 4"]
pub type Pin4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin4>;
impl<'a, REG> Pin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Af5)
    }
}
#[doc = "Select altfunc number for pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin5 {
    #[doc = "0: Altfunc disabled"]
    Disable = 0,
    #[doc = "1: Altfunc 1 enable"]
    Af1 = 1,
    #[doc = "2: Altfunc 2 enable"]
    Af2 = 2,
    #[doc = "3: Altfunc 3 enable"]
    Af3 = 3,
    #[doc = "4: Altfunc 4 enable"]
    Af4 = 4,
    #[doc = "5: Altfunc 5 enable"]
    Af5 = 5,
}
impl From<Pin5> for u8 {
    #[inline(always)]
    fn from(variant: Pin5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin5 {
    type Ux = u8;
}
impl crate::IsEnum for Pin5 {}
#[doc = "Field `PIN5` reader - Select altfunc number for pin 5"]
pub type Pin5R = crate::FieldReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin5> {
        match self.bits {
            0 => Some(Pin5::Disable),
            1 => Some(Pin5::Af1),
            2 => Some(Pin5::Af2),
            3 => Some(Pin5::Af3),
            4 => Some(Pin5::Af4),
            5 => Some(Pin5::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin5::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin5::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin5::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin5::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin5::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin5::Af5
    }
}
#[doc = "Field `PIN5` writer - Select altfunc number for pin 5"]
pub type Pin5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin5>;
impl<'a, REG> Pin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Af5)
    }
}
#[doc = "Select altfunc number for pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin6 {
    #[doc = "0: Altfunc disabled"]
    Disable = 0,
    #[doc = "1: Altfunc 1 enable"]
    Af1 = 1,
    #[doc = "2: Altfunc 2 enable"]
    Af2 = 2,
    #[doc = "3: Altfunc 3 enable"]
    Af3 = 3,
    #[doc = "4: Altfunc 4 enable"]
    Af4 = 4,
    #[doc = "5: Altfunc 5 enable"]
    Af5 = 5,
}
impl From<Pin6> for u8 {
    #[inline(always)]
    fn from(variant: Pin6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin6 {
    type Ux = u8;
}
impl crate::IsEnum for Pin6 {}
#[doc = "Field `PIN6` reader - Select altfunc number for pin 6"]
pub type Pin6R = crate::FieldReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin6> {
        match self.bits {
            0 => Some(Pin6::Disable),
            1 => Some(Pin6::Af1),
            2 => Some(Pin6::Af2),
            3 => Some(Pin6::Af3),
            4 => Some(Pin6::Af4),
            5 => Some(Pin6::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin6::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin6::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin6::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin6::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin6::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin6::Af5
    }
}
#[doc = "Field `PIN6` writer - Select altfunc number for pin 6"]
pub type Pin6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin6>;
impl<'a, REG> Pin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Af5)
    }
}
#[doc = "Select altfunc number for pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin7 {
    #[doc = "0: Altfunc disabled"]
    Disable = 0,
    #[doc = "1: Altfunc 1 enable"]
    Af1 = 1,
    #[doc = "2: Altfunc 2 enable"]
    Af2 = 2,
    #[doc = "3: Altfunc 3 enable"]
    Af3 = 3,
    #[doc = "4: Altfunc 4 enable"]
    Af4 = 4,
    #[doc = "5: Altfunc 5 enable"]
    Af5 = 5,
}
impl From<Pin7> for u8 {
    #[inline(always)]
    fn from(variant: Pin7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin7 {
    type Ux = u8;
}
impl crate::IsEnum for Pin7 {}
#[doc = "Field `PIN7` reader - Select altfunc number for pin 7"]
pub type Pin7R = crate::FieldReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin7> {
        match self.bits {
            0 => Some(Pin7::Disable),
            1 => Some(Pin7::Af1),
            2 => Some(Pin7::Af2),
            3 => Some(Pin7::Af3),
            4 => Some(Pin7::Af4),
            5 => Some(Pin7::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin7::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin7::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin7::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin7::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin7::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin7::Af5
    }
}
#[doc = "Field `PIN7` writer - Select altfunc number for pin 7"]
pub type Pin7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin7>;
impl<'a, REG> Pin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Af5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select altfunc number for pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Select altfunc number for pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Select altfunc number for pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select altfunc number for pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Select altfunc number for pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Select altfunc number for pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select altfunc number for pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Select altfunc number for pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select altfunc number for pin 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, Altfuncnum0Spec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select altfunc number for pin 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, Altfuncnum0Spec> {
        Pin1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Select altfunc number for pin 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<'_, Altfuncnum0Spec> {
        Pin2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Select altfunc number for pin 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, Altfuncnum0Spec> {
        Pin3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Select altfunc number for pin 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> Pin4W<'_, Altfuncnum0Spec> {
        Pin4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Select altfunc number for pin 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> Pin5W<'_, Altfuncnum0Spec> {
        Pin5W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Select altfunc number for pin 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, Altfuncnum0Spec> {
        Pin6W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Select altfunc number for pin 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, Altfuncnum0Spec> {
        Pin7W::new(self, 28)
    }
}
#[doc = "Alternative function number register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`altfuncnum0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altfuncnum0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Altfuncnum0Spec;
impl crate::RegisterSpec for Altfuncnum0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altfuncnum0::R`](R) reader structure"]
impl crate::Readable for Altfuncnum0Spec {}
#[doc = "`write(|w| ..)` method takes [`altfuncnum0::W`](W) writer structure"]
impl crate::Writable for Altfuncnum0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALTFUNCNUM0 to value 0"]
impl crate::Resettable for Altfuncnum0Spec {}
