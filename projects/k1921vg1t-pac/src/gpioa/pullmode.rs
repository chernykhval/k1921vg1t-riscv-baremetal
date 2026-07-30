#[doc = "Register `PULLMODE` reader"]
pub type R = crate::R<PullmodeSpec>;
#[doc = "Register `PULLMODE` writer"]
pub type W = crate::W<PullmodeSpec>;
#[doc = "Select pull mode for pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin0 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin0> for bool {
    #[inline(always)]
    fn from(variant: Pin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN0` reader - Select pull mode for pin 0"]
pub type Pin0R = crate::BitReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin0 {
        match self.bits {
            false => Pin0::Disable,
            true => Pin0::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin0::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin0::Pu
    }
}
#[doc = "Field `PIN0` writer - Select pull mode for pin 0"]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG, Pin0>;
impl<'a, REG> Pin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Pu)
    }
}
#[doc = "Select pull mode for pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin1 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin1> for bool {
    #[inline(always)]
    fn from(variant: Pin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN1` reader - Select pull mode for pin 1"]
pub type Pin1R = crate::BitReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin1 {
        match self.bits {
            false => Pin1::Disable,
            true => Pin1::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin1::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin1::Pu
    }
}
#[doc = "Field `PIN1` writer - Select pull mode for pin 1"]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG, Pin1>;
impl<'a, REG> Pin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Pu)
    }
}
#[doc = "Select pull mode for pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin2 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin2> for bool {
    #[inline(always)]
    fn from(variant: Pin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN2` reader - Select pull mode for pin 2"]
pub type Pin2R = crate::BitReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin2 {
        match self.bits {
            false => Pin2::Disable,
            true => Pin2::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin2::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin2::Pu
    }
}
#[doc = "Field `PIN2` writer - Select pull mode for pin 2"]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG, Pin2>;
impl<'a, REG> Pin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Pu)
    }
}
#[doc = "Select pull mode for pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin3 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin3> for bool {
    #[inline(always)]
    fn from(variant: Pin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN3` reader - Select pull mode for pin 3"]
pub type Pin3R = crate::BitReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin3 {
        match self.bits {
            false => Pin3::Disable,
            true => Pin3::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin3::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin3::Pu
    }
}
#[doc = "Field `PIN3` writer - Select pull mode for pin 3"]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG, Pin3>;
impl<'a, REG> Pin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Pu)
    }
}
#[doc = "Select pull mode for pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin4 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin4> for bool {
    #[inline(always)]
    fn from(variant: Pin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN4` reader - Select pull mode for pin 4"]
pub type Pin4R = crate::BitReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin4 {
        match self.bits {
            false => Pin4::Disable,
            true => Pin4::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin4::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin4::Pu
    }
}
#[doc = "Field `PIN4` writer - Select pull mode for pin 4"]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG, Pin4>;
impl<'a, REG> Pin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Pu)
    }
}
#[doc = "Select pull mode for pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin5 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin5> for bool {
    #[inline(always)]
    fn from(variant: Pin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN5` reader - Select pull mode for pin 5"]
pub type Pin5R = crate::BitReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin5 {
        match self.bits {
            false => Pin5::Disable,
            true => Pin5::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin5::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin5::Pu
    }
}
#[doc = "Field `PIN5` writer - Select pull mode for pin 5"]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG, Pin5>;
impl<'a, REG> Pin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Pu)
    }
}
#[doc = "Select pull mode for pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin6 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin6> for bool {
    #[inline(always)]
    fn from(variant: Pin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN6` reader - Select pull mode for pin 6"]
pub type Pin6R = crate::BitReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin6 {
        match self.bits {
            false => Pin6::Disable,
            true => Pin6::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin6::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin6::Pu
    }
}
#[doc = "Field `PIN6` writer - Select pull mode for pin 6"]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG, Pin6>;
impl<'a, REG> Pin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Pu)
    }
}
#[doc = "Select pull mode for pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin7 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin7> for bool {
    #[inline(always)]
    fn from(variant: Pin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN7` reader - Select pull mode for pin 7"]
pub type Pin7R = crate::BitReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin7 {
        match self.bits {
            false => Pin7::Disable,
            true => Pin7::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin7::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin7::Pu
    }
}
#[doc = "Field `PIN7` writer - Select pull mode for pin 7"]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG, Pin7>;
impl<'a, REG> Pin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Pu)
    }
}
#[doc = "Select pull mode for pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin8 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin8> for bool {
    #[inline(always)]
    fn from(variant: Pin8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN8` reader - Select pull mode for pin 8"]
pub type Pin8R = crate::BitReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin8 {
        match self.bits {
            false => Pin8::Disable,
            true => Pin8::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin8::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin8::Pu
    }
}
#[doc = "Field `PIN8` writer - Select pull mode for pin 8"]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG, Pin8>;
impl<'a, REG> Pin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Pu)
    }
}
#[doc = "Select pull mode for pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin9 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin9> for bool {
    #[inline(always)]
    fn from(variant: Pin9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN9` reader - Select pull mode for pin 9"]
pub type Pin9R = crate::BitReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin9 {
        match self.bits {
            false => Pin9::Disable,
            true => Pin9::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin9::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin9::Pu
    }
}
#[doc = "Field `PIN9` writer - Select pull mode for pin 9"]
pub type Pin9W<'a, REG> = crate::BitWriter<'a, REG, Pin9>;
impl<'a, REG> Pin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Pu)
    }
}
#[doc = "Select pull mode for pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin10 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin10> for bool {
    #[inline(always)]
    fn from(variant: Pin10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN10` reader - Select pull mode for pin 10"]
pub type Pin10R = crate::BitReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin10 {
        match self.bits {
            false => Pin10::Disable,
            true => Pin10::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin10::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin10::Pu
    }
}
#[doc = "Field `PIN10` writer - Select pull mode for pin 10"]
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG, Pin10>;
impl<'a, REG> Pin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Pu)
    }
}
#[doc = "Select pull mode for pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin11 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin11> for bool {
    #[inline(always)]
    fn from(variant: Pin11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN11` reader - Select pull mode for pin 11"]
pub type Pin11R = crate::BitReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin11 {
        match self.bits {
            false => Pin11::Disable,
            true => Pin11::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin11::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin11::Pu
    }
}
#[doc = "Field `PIN11` writer - Select pull mode for pin 11"]
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG, Pin11>;
impl<'a, REG> Pin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Pu)
    }
}
#[doc = "Select pull mode for pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin12 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin12> for bool {
    #[inline(always)]
    fn from(variant: Pin12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN12` reader - Select pull mode for pin 12"]
pub type Pin12R = crate::BitReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin12 {
        match self.bits {
            false => Pin12::Disable,
            true => Pin12::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin12::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin12::Pu
    }
}
#[doc = "Field `PIN12` writer - Select pull mode for pin 12"]
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG, Pin12>;
impl<'a, REG> Pin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Pu)
    }
}
#[doc = "Select pull mode for pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin13 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin13> for bool {
    #[inline(always)]
    fn from(variant: Pin13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN13` reader - Select pull mode for pin 13"]
pub type Pin13R = crate::BitReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin13 {
        match self.bits {
            false => Pin13::Disable,
            true => Pin13::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin13::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin13::Pu
    }
}
#[doc = "Field `PIN13` writer - Select pull mode for pin 13"]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG, Pin13>;
impl<'a, REG> Pin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Pu)
    }
}
#[doc = "Select pull mode for pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin14 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin14> for bool {
    #[inline(always)]
    fn from(variant: Pin14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN14` reader - Select pull mode for pin 14"]
pub type Pin14R = crate::BitReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin14 {
        match self.bits {
            false => Pin14::Disable,
            true => Pin14::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin14::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin14::Pu
    }
}
#[doc = "Field `PIN14` writer - Select pull mode for pin 14"]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG, Pin14>;
impl<'a, REG> Pin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Pu)
    }
}
#[doc = "Select pull mode for pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin15 {
    #[doc = "0: Pull disabled"]
    Disable = 0,
    #[doc = "1: Pull-up"]
    Pu = 1,
}
impl From<Pin15> for bool {
    #[inline(always)]
    fn from(variant: Pin15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN15` reader - Select pull mode for pin 15"]
pub type Pin15R = crate::BitReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin15 {
        match self.bits {
            false => Pin15::Disable,
            true => Pin15::Pu,
        }
    }
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pin15::Disable
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == Pin15::Pu
    }
}
#[doc = "Field `PIN15` writer - Select pull mode for pin 15"]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG, Pin15>;
impl<'a, REG> Pin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Disable)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Pu)
    }
}
impl R {
    #[doc = "Bit 0 - Select pull mode for pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select pull mode for pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select pull mode for pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select pull mode for pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select pull mode for pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select pull mode for pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select pull mode for pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select pull mode for pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select pull mode for pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select pull mode for pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select pull mode for pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select pull mode for pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select pull mode for pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select pull mode for pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select pull mode for pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select pull mode for pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select pull mode for pin 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, PullmodeSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Select pull mode for pin 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, PullmodeSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Select pull mode for pin 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<'_, PullmodeSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Select pull mode for pin 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, PullmodeSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Select pull mode for pin 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> Pin4W<'_, PullmodeSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Select pull mode for pin 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> Pin5W<'_, PullmodeSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Select pull mode for pin 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, PullmodeSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Select pull mode for pin 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, PullmodeSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Select pull mode for pin 8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> Pin8W<'_, PullmodeSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9 - Select pull mode for pin 9"]
    #[inline(always)]
    pub fn pin9(&mut self) -> Pin9W<'_, PullmodeSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10 - Select pull mode for pin 10"]
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<'_, PullmodeSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11 - Select pull mode for pin 11"]
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<'_, PullmodeSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12 - Select pull mode for pin 12"]
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<'_, PullmodeSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13 - Select pull mode for pin 13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<'_, PullmodeSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - Select pull mode for pin 14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<'_, PullmodeSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - Select pull mode for pin 15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<'_, PullmodeSpec> {
        Pin15W::new(self, 15)
    }
}
#[doc = "Select pull mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`pullmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pullmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PullmodeSpec;
impl crate::RegisterSpec for PullmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pullmode::R`](R) reader structure"]
impl crate::Readable for PullmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`pullmode::W`](W) writer structure"]
impl crate::Writable for PullmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PULLMODE to value 0"]
impl crate::Resettable for PullmodeSpec {}
