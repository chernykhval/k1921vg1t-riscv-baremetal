#[doc = "Register `TMRCFG` reader"]
pub type R = crate::R<TmrcfgSpec>;
#[doc = "Register `TMRCFG` writer"]
pub type W = crate::W<TmrcfgSpec>;
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr0clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr0clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr0clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR0CLKMUX` reader - Select ecternal clock source"]
pub type Tmr0clkmuxR = crate::BitReader<Tmr0clkmux>;
impl Tmr0clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr0clkmux {
        match self.bits {
            false => Tmr0clkmux::Extpin,
            true => Tmr0clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr0clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr0clkmux::Pretimer
    }
}
#[doc = "Field `TMR0CLKMUX` writer - Select ecternal clock source"]
pub type Tmr0clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr0clkmux>;
impl<'a, REG> Tmr0clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr0clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr1clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr1clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr1clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR1CLKMUX` reader - Select ecternal clock source"]
pub type Tmr1clkmuxR = crate::BitReader<Tmr1clkmux>;
impl Tmr1clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr1clkmux {
        match self.bits {
            false => Tmr1clkmux::Extpin,
            true => Tmr1clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr1clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr1clkmux::Pretimer
    }
}
#[doc = "Field `TMR1CLKMUX` writer - Select ecternal clock source"]
pub type Tmr1clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr1clkmux>;
impl<'a, REG> Tmr1clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr1clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr1clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr2clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr2clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr2clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR2CLKMUX` reader - Select ecternal clock source"]
pub type Tmr2clkmuxR = crate::BitReader<Tmr2clkmux>;
impl Tmr2clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr2clkmux {
        match self.bits {
            false => Tmr2clkmux::Extpin,
            true => Tmr2clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr2clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr2clkmux::Pretimer
    }
}
#[doc = "Field `TMR2CLKMUX` writer - Select ecternal clock source"]
pub type Tmr2clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr2clkmux>;
impl<'a, REG> Tmr2clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr2clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr2clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr3clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr3clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr3clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR3CLKMUX` reader - Select ecternal clock source"]
pub type Tmr3clkmuxR = crate::BitReader<Tmr3clkmux>;
impl Tmr3clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr3clkmux {
        match self.bits {
            false => Tmr3clkmux::Extpin,
            true => Tmr3clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr3clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr3clkmux::Pretimer
    }
}
#[doc = "Field `TMR3CLKMUX` writer - Select ecternal clock source"]
pub type Tmr3clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr3clkmux>;
impl<'a, REG> Tmr3clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr3clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr3clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr4clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr4clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr4clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR4CLKMUX` reader - Select ecternal clock source"]
pub type Tmr4clkmuxR = crate::BitReader<Tmr4clkmux>;
impl Tmr4clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr4clkmux {
        match self.bits {
            false => Tmr4clkmux::Extpin,
            true => Tmr4clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr4clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr4clkmux::Pretimer
    }
}
#[doc = "Field `TMR4CLKMUX` writer - Select ecternal clock source"]
pub type Tmr4clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr4clkmux>;
impl<'a, REG> Tmr4clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr4clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr5clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr5clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr5clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR5CLKMUX` reader - Select ecternal clock source"]
pub type Tmr5clkmuxR = crate::BitReader<Tmr5clkmux>;
impl Tmr5clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr5clkmux {
        match self.bits {
            false => Tmr5clkmux::Extpin,
            true => Tmr5clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr5clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr5clkmux::Pretimer
    }
}
#[doc = "Field `TMR5CLKMUX` writer - Select ecternal clock source"]
pub type Tmr5clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr5clkmux>;
impl<'a, REG> Tmr5clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr5clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr5clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr6clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr6clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr6clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR6CLKMUX` reader - Select ecternal clock source"]
pub type Tmr6clkmuxR = crate::BitReader<Tmr6clkmux>;
impl Tmr6clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr6clkmux {
        match self.bits {
            false => Tmr6clkmux::Extpin,
            true => Tmr6clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr6clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr6clkmux::Pretimer
    }
}
#[doc = "Field `TMR6CLKMUX` writer - Select ecternal clock source"]
pub type Tmr6clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr6clkmux>;
impl<'a, REG> Tmr6clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr6clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr6clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr7clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr7clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr7clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR7CLKMUX` reader - Select ecternal clock source"]
pub type Tmr7clkmuxR = crate::BitReader<Tmr7clkmux>;
impl Tmr7clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr7clkmux {
        match self.bits {
            false => Tmr7clkmux::Extpin,
            true => Tmr7clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr7clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr7clkmux::Pretimer
    }
}
#[doc = "Field `TMR7CLKMUX` writer - Select ecternal clock source"]
pub type Tmr7clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr7clkmux>;
impl<'a, REG> Tmr7clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr7clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr7clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr8clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr8clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr8clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR8CLKMUX` reader - Select ecternal clock source"]
pub type Tmr8clkmuxR = crate::BitReader<Tmr8clkmux>;
impl Tmr8clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr8clkmux {
        match self.bits {
            false => Tmr8clkmux::Extpin,
            true => Tmr8clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr8clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr8clkmux::Pretimer
    }
}
#[doc = "Field `TMR8CLKMUX` writer - Select ecternal clock source"]
pub type Tmr8clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr8clkmux>;
impl<'a, REG> Tmr8clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr8clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr8clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr9clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr9clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr9clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR9CLKMUX` reader - Select ecternal clock source"]
pub type Tmr9clkmuxR = crate::BitReader<Tmr9clkmux>;
impl Tmr9clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr9clkmux {
        match self.bits {
            false => Tmr9clkmux::Extpin,
            true => Tmr9clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr9clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr9clkmux::Pretimer
    }
}
#[doc = "Field `TMR9CLKMUX` writer - Select ecternal clock source"]
pub type Tmr9clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr9clkmux>;
impl<'a, REG> Tmr9clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr9clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr9clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr10clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr10clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr10clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR10CLKMUX` reader - Select ecternal clock source"]
pub type Tmr10clkmuxR = crate::BitReader<Tmr10clkmux>;
impl Tmr10clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr10clkmux {
        match self.bits {
            false => Tmr10clkmux::Extpin,
            true => Tmr10clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr10clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr10clkmux::Pretimer
    }
}
#[doc = "Field `TMR10CLKMUX` writer - Select ecternal clock source"]
pub type Tmr10clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr10clkmux>;
impl<'a, REG> Tmr10clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr10clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr10clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr11clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr11clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr11clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR11CLKMUX` reader - Select ecternal clock source"]
pub type Tmr11clkmuxR = crate::BitReader<Tmr11clkmux>;
impl Tmr11clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr11clkmux {
        match self.bits {
            false => Tmr11clkmux::Extpin,
            true => Tmr11clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr11clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr11clkmux::Pretimer
    }
}
#[doc = "Field `TMR11CLKMUX` writer - Select ecternal clock source"]
pub type Tmr11clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr11clkmux>;
impl<'a, REG> Tmr11clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr11clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr11clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr12clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr12clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr12clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR12CLKMUX` reader - Select ecternal clock source"]
pub type Tmr12clkmuxR = crate::BitReader<Tmr12clkmux>;
impl Tmr12clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr12clkmux {
        match self.bits {
            false => Tmr12clkmux::Extpin,
            true => Tmr12clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr12clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr12clkmux::Pretimer
    }
}
#[doc = "Field `TMR12CLKMUX` writer - Select ecternal clock source"]
pub type Tmr12clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr12clkmux>;
impl<'a, REG> Tmr12clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr12clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr12clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr13clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr13clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr13clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR13CLKMUX` reader - Select ecternal clock source"]
pub type Tmr13clkmuxR = crate::BitReader<Tmr13clkmux>;
impl Tmr13clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr13clkmux {
        match self.bits {
            false => Tmr13clkmux::Extpin,
            true => Tmr13clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr13clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr13clkmux::Pretimer
    }
}
#[doc = "Field `TMR13CLKMUX` writer - Select ecternal clock source"]
pub type Tmr13clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr13clkmux>;
impl<'a, REG> Tmr13clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr14clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr14clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr14clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR14CLKMUX` reader - Select ecternal clock source"]
pub type Tmr14clkmuxR = crate::BitReader<Tmr14clkmux>;
impl Tmr14clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr14clkmux {
        match self.bits {
            false => Tmr14clkmux::Extpin,
            true => Tmr14clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr14clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr14clkmux::Pretimer
    }
}
#[doc = "Field `TMR14CLKMUX` writer - Select ecternal clock source"]
pub type Tmr14clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr14clkmux>;
impl<'a, REG> Tmr14clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr14clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr14clkmux::Pretimer)
    }
}
#[doc = "Select ecternal clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmr15clkmux {
    #[doc = "0: external controler pin"]
    Extpin = 0,
    #[doc = "1: previous timer PWM-output"]
    Pretimer = 1,
}
impl From<Tmr15clkmux> for bool {
    #[inline(always)]
    fn from(variant: Tmr15clkmux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR15CLKMUX` reader - Select ecternal clock source"]
pub type Tmr15clkmuxR = crate::BitReader<Tmr15clkmux>;
impl Tmr15clkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmr15clkmux {
        match self.bits {
            false => Tmr15clkmux::Extpin,
            true => Tmr15clkmux::Pretimer,
        }
    }
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn is_extpin(&self) -> bool {
        *self == Tmr15clkmux::Extpin
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn is_pretimer(&self) -> bool {
        *self == Tmr15clkmux::Pretimer
    }
}
#[doc = "Field `TMR15CLKMUX` writer - Select ecternal clock source"]
pub type Tmr15clkmuxW<'a, REG> = crate::BitWriter<'a, REG, Tmr15clkmux>;
impl<'a, REG> Tmr15clkmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external controler pin"]
    #[inline(always)]
    pub fn extpin(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr15clkmux::Extpin)
    }
    #[doc = "previous timer PWM-output"]
    #[inline(always)]
    pub fn pretimer(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr15clkmux::Pretimer)
    }
}
impl R {
    #[doc = "Bit 0 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr0clkmux(&self) -> Tmr0clkmuxR {
        Tmr0clkmuxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr1clkmux(&self) -> Tmr1clkmuxR {
        Tmr1clkmuxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr2clkmux(&self) -> Tmr2clkmuxR {
        Tmr2clkmuxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr3clkmux(&self) -> Tmr3clkmuxR {
        Tmr3clkmuxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr4clkmux(&self) -> Tmr4clkmuxR {
        Tmr4clkmuxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr5clkmux(&self) -> Tmr5clkmuxR {
        Tmr5clkmuxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr6clkmux(&self) -> Tmr6clkmuxR {
        Tmr6clkmuxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr7clkmux(&self) -> Tmr7clkmuxR {
        Tmr7clkmuxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr8clkmux(&self) -> Tmr8clkmuxR {
        Tmr8clkmuxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr9clkmux(&self) -> Tmr9clkmuxR {
        Tmr9clkmuxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr10clkmux(&self) -> Tmr10clkmuxR {
        Tmr10clkmuxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr11clkmux(&self) -> Tmr11clkmuxR {
        Tmr11clkmuxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr12clkmux(&self) -> Tmr12clkmuxR {
        Tmr12clkmuxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr13clkmux(&self) -> Tmr13clkmuxR {
        Tmr13clkmuxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr14clkmux(&self) -> Tmr14clkmuxR {
        Tmr14clkmuxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr15clkmux(&self) -> Tmr15clkmuxR {
        Tmr15clkmuxR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr0clkmux(&mut self) -> Tmr0clkmuxW<'_, TmrcfgSpec> {
        Tmr0clkmuxW::new(self, 0)
    }
    #[doc = "Bit 1 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr1clkmux(&mut self) -> Tmr1clkmuxW<'_, TmrcfgSpec> {
        Tmr1clkmuxW::new(self, 1)
    }
    #[doc = "Bit 2 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr2clkmux(&mut self) -> Tmr2clkmuxW<'_, TmrcfgSpec> {
        Tmr2clkmuxW::new(self, 2)
    }
    #[doc = "Bit 3 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr3clkmux(&mut self) -> Tmr3clkmuxW<'_, TmrcfgSpec> {
        Tmr3clkmuxW::new(self, 3)
    }
    #[doc = "Bit 4 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr4clkmux(&mut self) -> Tmr4clkmuxW<'_, TmrcfgSpec> {
        Tmr4clkmuxW::new(self, 4)
    }
    #[doc = "Bit 5 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr5clkmux(&mut self) -> Tmr5clkmuxW<'_, TmrcfgSpec> {
        Tmr5clkmuxW::new(self, 5)
    }
    #[doc = "Bit 6 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr6clkmux(&mut self) -> Tmr6clkmuxW<'_, TmrcfgSpec> {
        Tmr6clkmuxW::new(self, 6)
    }
    #[doc = "Bit 7 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr7clkmux(&mut self) -> Tmr7clkmuxW<'_, TmrcfgSpec> {
        Tmr7clkmuxW::new(self, 7)
    }
    #[doc = "Bit 8 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr8clkmux(&mut self) -> Tmr8clkmuxW<'_, TmrcfgSpec> {
        Tmr8clkmuxW::new(self, 8)
    }
    #[doc = "Bit 9 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr9clkmux(&mut self) -> Tmr9clkmuxW<'_, TmrcfgSpec> {
        Tmr9clkmuxW::new(self, 9)
    }
    #[doc = "Bit 10 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr10clkmux(&mut self) -> Tmr10clkmuxW<'_, TmrcfgSpec> {
        Tmr10clkmuxW::new(self, 10)
    }
    #[doc = "Bit 11 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr11clkmux(&mut self) -> Tmr11clkmuxW<'_, TmrcfgSpec> {
        Tmr11clkmuxW::new(self, 11)
    }
    #[doc = "Bit 12 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr12clkmux(&mut self) -> Tmr12clkmuxW<'_, TmrcfgSpec> {
        Tmr12clkmuxW::new(self, 12)
    }
    #[doc = "Bit 13 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr13clkmux(&mut self) -> Tmr13clkmuxW<'_, TmrcfgSpec> {
        Tmr13clkmuxW::new(self, 13)
    }
    #[doc = "Bit 14 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr14clkmux(&mut self) -> Tmr14clkmuxW<'_, TmrcfgSpec> {
        Tmr14clkmuxW::new(self, 14)
    }
    #[doc = "Bit 15 - Select ecternal clock source"]
    #[inline(always)]
    pub fn tmr15clkmux(&mut self) -> Tmr15clkmuxW<'_, TmrcfgSpec> {
        Tmr15clkmuxW::new(self, 15)
    }
}
#[doc = "Timers external clock config\n\nYou can [`read`](crate::Reg::read) this register and get [`tmrcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmrcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrcfgSpec;
impl crate::RegisterSpec for TmrcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmrcfg::R`](R) reader structure"]
impl crate::Readable for TmrcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tmrcfg::W`](W) writer structure"]
impl crate::Writable for TmrcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMRCFG to value 0"]
impl crate::Resettable for TmrcfgSpec {}
