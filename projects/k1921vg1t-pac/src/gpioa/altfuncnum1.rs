#[doc = "Register `ALTFUNCNUM1` reader"]
pub type R = crate::R<Altfuncnum1Spec>;
#[doc = "Register `ALTFUNCNUM1` writer"]
pub type W = crate::W<Altfuncnum1Spec>;
#[doc = "Select altfunc number for pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin8 {
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
impl From<Pin8> for u8 {
    #[inline(always)]
    fn from(variant: Pin8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin8 {
    type Ux = u8;
}
impl crate::IsEnum for Pin8 {}
#[doc = "Field `PIN8` reader - Select altfunc number for pin 8"]
pub type Pin8R = crate::FieldReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin8> {
        match self.bits {
            0 => Some(Pin8::Disable),
            1 => Some(Pin8::Af1),
            2 => Some(Pin8::Af2),
            3 => Some(Pin8::Af3),
            4 => Some(Pin8::Af4),
            5 => Some(Pin8::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin8::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin8::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin8::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin8::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin8::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin8::Af5
    }
}
#[doc = "Field `PIN8` writer - Select altfunc number for pin 8"]
pub type Pin8W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin8>;
impl<'a, REG> Pin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Af5)
    }
}
#[doc = "Select altfunc number for pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin9 {
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
impl From<Pin9> for u8 {
    #[inline(always)]
    fn from(variant: Pin9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin9 {
    type Ux = u8;
}
impl crate::IsEnum for Pin9 {}
#[doc = "Field `PIN9` reader - Select altfunc number for pin 9"]
pub type Pin9R = crate::FieldReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin9> {
        match self.bits {
            0 => Some(Pin9::Disable),
            1 => Some(Pin9::Af1),
            2 => Some(Pin9::Af2),
            3 => Some(Pin9::Af3),
            4 => Some(Pin9::Af4),
            5 => Some(Pin9::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin9::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin9::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin9::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin9::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin9::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin9::Af5
    }
}
#[doc = "Field `PIN9` writer - Select altfunc number for pin 9"]
pub type Pin9W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin9>;
impl<'a, REG> Pin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Af5)
    }
}
#[doc = "Select altfunc number for pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin10 {
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
impl From<Pin10> for u8 {
    #[inline(always)]
    fn from(variant: Pin10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin10 {
    type Ux = u8;
}
impl crate::IsEnum for Pin10 {}
#[doc = "Field `PIN10` reader - Select altfunc number for pin 10"]
pub type Pin10R = crate::FieldReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin10> {
        match self.bits {
            0 => Some(Pin10::Disable),
            1 => Some(Pin10::Af1),
            2 => Some(Pin10::Af2),
            3 => Some(Pin10::Af3),
            4 => Some(Pin10::Af4),
            5 => Some(Pin10::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin10::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin10::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin10::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin10::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin10::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin10::Af5
    }
}
#[doc = "Field `PIN10` writer - Select altfunc number for pin 10"]
pub type Pin10W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin10>;
impl<'a, REG> Pin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Af5)
    }
}
#[doc = "Select altfunc number for pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin11 {
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
impl From<Pin11> for u8 {
    #[inline(always)]
    fn from(variant: Pin11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin11 {
    type Ux = u8;
}
impl crate::IsEnum for Pin11 {}
#[doc = "Field `PIN11` reader - Select altfunc number for pin 11"]
pub type Pin11R = crate::FieldReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin11> {
        match self.bits {
            0 => Some(Pin11::Disable),
            1 => Some(Pin11::Af1),
            2 => Some(Pin11::Af2),
            3 => Some(Pin11::Af3),
            4 => Some(Pin11::Af4),
            5 => Some(Pin11::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin11::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin11::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin11::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin11::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin11::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin11::Af5
    }
}
#[doc = "Field `PIN11` writer - Select altfunc number for pin 11"]
pub type Pin11W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin11>;
impl<'a, REG> Pin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Af5)
    }
}
#[doc = "Select altfunc number for pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin12 {
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
impl From<Pin12> for u8 {
    #[inline(always)]
    fn from(variant: Pin12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin12 {
    type Ux = u8;
}
impl crate::IsEnum for Pin12 {}
#[doc = "Field `PIN12` reader - Select altfunc number for pin 12"]
pub type Pin12R = crate::FieldReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin12> {
        match self.bits {
            0 => Some(Pin12::Disable),
            1 => Some(Pin12::Af1),
            2 => Some(Pin12::Af2),
            3 => Some(Pin12::Af3),
            4 => Some(Pin12::Af4),
            5 => Some(Pin12::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin12::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin12::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin12::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin12::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin12::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin12::Af5
    }
}
#[doc = "Field `PIN12` writer - Select altfunc number for pin 12"]
pub type Pin12W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin12>;
impl<'a, REG> Pin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Af5)
    }
}
#[doc = "Select altfunc number for pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin13 {
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
impl From<Pin13> for u8 {
    #[inline(always)]
    fn from(variant: Pin13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin13 {
    type Ux = u8;
}
impl crate::IsEnum for Pin13 {}
#[doc = "Field `PIN13` reader - Select altfunc number for pin 13"]
pub type Pin13R = crate::FieldReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin13> {
        match self.bits {
            0 => Some(Pin13::Disable),
            1 => Some(Pin13::Af1),
            2 => Some(Pin13::Af2),
            3 => Some(Pin13::Af3),
            4 => Some(Pin13::Af4),
            5 => Some(Pin13::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin13::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin13::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin13::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin13::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin13::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin13::Af5
    }
}
#[doc = "Field `PIN13` writer - Select altfunc number for pin 13"]
pub type Pin13W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin13>;
impl<'a, REG> Pin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Af5)
    }
}
#[doc = "Select altfunc number for pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin14 {
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
impl From<Pin14> for u8 {
    #[inline(always)]
    fn from(variant: Pin14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin14 {
    type Ux = u8;
}
impl crate::IsEnum for Pin14 {}
#[doc = "Field `PIN14` reader - Select altfunc number for pin 14"]
pub type Pin14R = crate::FieldReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin14> {
        match self.bits {
            0 => Some(Pin14::Disable),
            1 => Some(Pin14::Af1),
            2 => Some(Pin14::Af2),
            3 => Some(Pin14::Af3),
            4 => Some(Pin14::Af4),
            5 => Some(Pin14::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin14::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin14::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin14::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin14::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin14::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin14::Af5
    }
}
#[doc = "Field `PIN14` writer - Select altfunc number for pin 14"]
pub type Pin14W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin14>;
impl<'a, REG> Pin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Af5)
    }
}
#[doc = "Select altfunc number for pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin15 {
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
impl From<Pin15> for u8 {
    #[inline(always)]
    fn from(variant: Pin15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin15 {
    type Ux = u8;
}
impl crate::IsEnum for Pin15 {}
#[doc = "Field `PIN15` reader - Select altfunc number for pin 15"]
pub type Pin15R = crate::FieldReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin15> {
        match self.bits {
            0 => Some(Pin15::Disable),
            1 => Some(Pin15::Af1),
            2 => Some(Pin15::Af2),
            3 => Some(Pin15::Af3),
            4 => Some(Pin15::Af4),
            5 => Some(Pin15::Af5),
            _ => None,
        }
    }
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin15::Disable
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Pin15::Af1
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Pin15::Af2
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Pin15::Af3
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Pin15::Af4
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Pin15::Af5
    }
}
#[doc = "Field `PIN15` writer - Select altfunc number for pin 15"]
pub type Pin15W<'a, REG> = crate::FieldWriter<'a, REG, 3, Pin15>;
impl<'a, REG> Pin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Altfunc disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Disable)
    }
    #[doc = "Altfunc 1 enable"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Af1)
    }
    #[doc = "Altfunc 2 enable"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Af2)
    }
    #[doc = "Altfunc 3 enable"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Af3)
    }
    #[doc = "Altfunc 4 enable"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Af4)
    }
    #[doc = "Altfunc 5 enable"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Af5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select altfunc number for pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Select altfunc number for pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Select altfunc number for pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select altfunc number for pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Select altfunc number for pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Select altfunc number for pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select altfunc number for pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Select altfunc number for pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select altfunc number for pin 8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> Pin8W<'_, Altfuncnum1Spec> {
        Pin8W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select altfunc number for pin 9"]
    #[inline(always)]
    pub fn pin9(&mut self) -> Pin9W<'_, Altfuncnum1Spec> {
        Pin9W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Select altfunc number for pin 10"]
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<'_, Altfuncnum1Spec> {
        Pin10W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Select altfunc number for pin 11"]
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<'_, Altfuncnum1Spec> {
        Pin11W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Select altfunc number for pin 12"]
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<'_, Altfuncnum1Spec> {
        Pin12W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Select altfunc number for pin 13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<'_, Altfuncnum1Spec> {
        Pin13W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Select altfunc number for pin 14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<'_, Altfuncnum1Spec> {
        Pin14W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Select altfunc number for pin 15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<'_, Altfuncnum1Spec> {
        Pin15W::new(self, 28)
    }
}
#[doc = "Alternative function number register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`altfuncnum1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altfuncnum1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Altfuncnum1Spec;
impl crate::RegisterSpec for Altfuncnum1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altfuncnum1::R`](R) reader structure"]
impl crate::Readable for Altfuncnum1Spec {}
#[doc = "`write(|w| ..)` method takes [`altfuncnum1::W`](W) writer structure"]
impl crate::Writable for Altfuncnum1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALTFUNCNUM1 to value 0"]
impl crate::Resettable for Altfuncnum1Spec {}
