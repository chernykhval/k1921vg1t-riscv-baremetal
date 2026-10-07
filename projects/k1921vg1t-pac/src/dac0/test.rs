#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Select output signal for dac pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DacOut {
    #[doc = "0: normal mode"]
    Normal = 0,
    #[doc = "1: Connect out to AVDD"]
    Pullup = 1,
    #[doc = "2: Connect out to DVDD"]
    Pulldown = 2,
    #[doc = "3: Tri-state output"]
    Zstate = 3,
}
impl From<DacOut> for u8 {
    #[inline(always)]
    fn from(variant: DacOut) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DacOut {
    type Ux = u8;
}
impl crate::IsEnum for DacOut {}
#[doc = "Field `DAC_OUT` reader - Select output signal for dac pad"]
pub type DacOutR = crate::FieldReader<DacOut>;
impl DacOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DacOut {
        match self.bits {
            0 => DacOut::Normal,
            1 => DacOut::Pullup,
            2 => DacOut::Pulldown,
            3 => DacOut::Zstate,
            _ => unreachable!(),
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DacOut::Normal
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == DacOut::Pullup
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == DacOut::Pulldown
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn is_zstate(&self) -> bool {
        *self == DacOut::Zstate
    }
}
#[doc = "Field `DAC_OUT` writer - Select output signal for dac pad"]
pub type DacOutW<'a, REG> = crate::FieldWriter<'a, REG, 2, DacOut, crate::Safe>;
impl<'a, REG> DacOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(DacOut::Normal)
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(DacOut::Pullup)
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(DacOut::Pulldown)
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn zstate(self) -> &'a mut crate::W<REG> {
        self.variant(DacOut::Zstate)
    }
}
#[doc = "Select output signal for adc0\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc0Test {
    #[doc = "0: normal mode"]
    Normal = 0,
    #[doc = "1: Connect out to AVDD"]
    Pullup = 1,
    #[doc = "2: Connect out to DVDD"]
    Pulldown = 2,
    #[doc = "3: Tri-state output"]
    Zstate = 3,
}
impl From<Adc0Test> for u8 {
    #[inline(always)]
    fn from(variant: Adc0Test) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc0Test {
    type Ux = u8;
}
impl crate::IsEnum for Adc0Test {}
#[doc = "Field `ADC0_TEST` reader - Select output signal for adc0"]
pub type Adc0TestR = crate::FieldReader<Adc0Test>;
impl Adc0TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0Test {
        match self.bits {
            0 => Adc0Test::Normal,
            1 => Adc0Test::Pullup,
            2 => Adc0Test::Pulldown,
            3 => Adc0Test::Zstate,
            _ => unreachable!(),
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Adc0Test::Normal
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Adc0Test::Pullup
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Adc0Test::Pulldown
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn is_zstate(&self) -> bool {
        *self == Adc0Test::Zstate
    }
}
#[doc = "Field `ADC0_TEST` writer - Select output signal for adc0"]
pub type Adc0TestW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc0Test, crate::Safe>;
impl<'a, REG> Adc0TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0Test::Normal)
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0Test::Pullup)
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0Test::Pulldown)
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn zstate(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0Test::Zstate)
    }
}
#[doc = "Select output signal for adc1\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc1Test {
    #[doc = "0: normal mode"]
    Normal = 0,
    #[doc = "1: Connect out to AVDD"]
    Pullup = 1,
    #[doc = "2: Connect out to DVDD"]
    Pulldown = 2,
    #[doc = "3: Tri-state output"]
    Zstate = 3,
}
impl From<Adc1Test> for u8 {
    #[inline(always)]
    fn from(variant: Adc1Test) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc1Test {
    type Ux = u8;
}
impl crate::IsEnum for Adc1Test {}
#[doc = "Field `ADC1_TEST` reader - Select output signal for adc1"]
pub type Adc1TestR = crate::FieldReader<Adc1Test>;
impl Adc1TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc1Test {
        match self.bits {
            0 => Adc1Test::Normal,
            1 => Adc1Test::Pullup,
            2 => Adc1Test::Pulldown,
            3 => Adc1Test::Zstate,
            _ => unreachable!(),
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Adc1Test::Normal
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Adc1Test::Pullup
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Adc1Test::Pulldown
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn is_zstate(&self) -> bool {
        *self == Adc1Test::Zstate
    }
}
#[doc = "Field `ADC1_TEST` writer - Select output signal for adc1"]
pub type Adc1TestW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc1Test, crate::Safe>;
impl<'a, REG> Adc1TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1Test::Normal)
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1Test::Pullup)
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1Test::Pulldown)
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn zstate(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1Test::Zstate)
    }
}
#[doc = "Select output signal for adc2\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc2Test {
    #[doc = "0: normal mode"]
    Normal = 0,
    #[doc = "1: Connect out to AVDD"]
    Pullup = 1,
    #[doc = "2: Connect out to DVDD"]
    Pulldown = 2,
    #[doc = "3: Tri-state output"]
    Zstate = 3,
}
impl From<Adc2Test> for u8 {
    #[inline(always)]
    fn from(variant: Adc2Test) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc2Test {
    type Ux = u8;
}
impl crate::IsEnum for Adc2Test {}
#[doc = "Field `ADC2_TEST` reader - Select output signal for adc2"]
pub type Adc2TestR = crate::FieldReader<Adc2Test>;
impl Adc2TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc2Test {
        match self.bits {
            0 => Adc2Test::Normal,
            1 => Adc2Test::Pullup,
            2 => Adc2Test::Pulldown,
            3 => Adc2Test::Zstate,
            _ => unreachable!(),
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Adc2Test::Normal
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Adc2Test::Pullup
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Adc2Test::Pulldown
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn is_zstate(&self) -> bool {
        *self == Adc2Test::Zstate
    }
}
#[doc = "Field `ADC2_TEST` writer - Select output signal for adc2"]
pub type Adc2TestW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc2Test, crate::Safe>;
impl<'a, REG> Adc2TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Adc2Test::Normal)
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Adc2Test::Pullup)
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Adc2Test::Pulldown)
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn zstate(self) -> &'a mut crate::W<REG> {
        self.variant(Adc2Test::Zstate)
    }
}
#[doc = "Select output signal for adc3\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc3Test {
    #[doc = "0: normal mode"]
    Normal = 0,
    #[doc = "1: Connect out to AVDD"]
    Pullup = 1,
    #[doc = "2: Connect out to DVDD"]
    Pulldown = 2,
    #[doc = "3: Tri-state output"]
    Zstate = 3,
}
impl From<Adc3Test> for u8 {
    #[inline(always)]
    fn from(variant: Adc3Test) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc3Test {
    type Ux = u8;
}
impl crate::IsEnum for Adc3Test {}
#[doc = "Field `ADC3_TEST` reader - Select output signal for adc3"]
pub type Adc3TestR = crate::FieldReader<Adc3Test>;
impl Adc3TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc3Test {
        match self.bits {
            0 => Adc3Test::Normal,
            1 => Adc3Test::Pullup,
            2 => Adc3Test::Pulldown,
            3 => Adc3Test::Zstate,
            _ => unreachable!(),
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Adc3Test::Normal
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Adc3Test::Pullup
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Adc3Test::Pulldown
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn is_zstate(&self) -> bool {
        *self == Adc3Test::Zstate
    }
}
#[doc = "Field `ADC3_TEST` writer - Select output signal for adc3"]
pub type Adc3TestW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc3Test, crate::Safe>;
impl<'a, REG> Adc3TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Adc3Test::Normal)
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Adc3Test::Pullup)
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Adc3Test::Pulldown)
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn zstate(self) -> &'a mut crate::W<REG> {
        self.variant(Adc3Test::Zstate)
    }
}
#[doc = "Select output signal for cmp0\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmp0Test {
    #[doc = "0: normal mode"]
    Normal = 0,
    #[doc = "1: Connect out to AVDD"]
    Pullup = 1,
    #[doc = "2: Connect out to DVDD"]
    Pulldown = 2,
    #[doc = "3: Tri-state output"]
    Zstate = 3,
}
impl From<Cmp0Test> for u8 {
    #[inline(always)]
    fn from(variant: Cmp0Test) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmp0Test {
    type Ux = u8;
}
impl crate::IsEnum for Cmp0Test {}
#[doc = "Field `CMP0_TEST` reader - Select output signal for cmp0"]
pub type Cmp0TestR = crate::FieldReader<Cmp0Test>;
impl Cmp0TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0Test {
        match self.bits {
            0 => Cmp0Test::Normal,
            1 => Cmp0Test::Pullup,
            2 => Cmp0Test::Pulldown,
            3 => Cmp0Test::Zstate,
            _ => unreachable!(),
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Cmp0Test::Normal
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Cmp0Test::Pullup
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Cmp0Test::Pulldown
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn is_zstate(&self) -> bool {
        *self == Cmp0Test::Zstate
    }
}
#[doc = "Field `CMP0_TEST` writer - Select output signal for cmp0"]
pub type Cmp0TestW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmp0Test, crate::Safe>;
impl<'a, REG> Cmp0TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0Test::Normal)
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0Test::Pullup)
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0Test::Pulldown)
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn zstate(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0Test::Zstate)
    }
}
#[doc = "Select output signal for cmp1\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmp1Test {
    #[doc = "0: normal mode"]
    Normal = 0,
    #[doc = "1: Connect out to AVDD"]
    Pullup = 1,
    #[doc = "2: Connect out to DVDD"]
    Pulldown = 2,
    #[doc = "3: Tri-state output"]
    Zstate = 3,
}
impl From<Cmp1Test> for u8 {
    #[inline(always)]
    fn from(variant: Cmp1Test) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmp1Test {
    type Ux = u8;
}
impl crate::IsEnum for Cmp1Test {}
#[doc = "Field `CMP1_TEST` reader - Select output signal for cmp1"]
pub type Cmp1TestR = crate::FieldReader<Cmp1Test>;
impl Cmp1TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp1Test {
        match self.bits {
            0 => Cmp1Test::Normal,
            1 => Cmp1Test::Pullup,
            2 => Cmp1Test::Pulldown,
            3 => Cmp1Test::Zstate,
            _ => unreachable!(),
        }
    }
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Cmp1Test::Normal
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Cmp1Test::Pullup
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Cmp1Test::Pulldown
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn is_zstate(&self) -> bool {
        *self == Cmp1Test::Zstate
    }
}
#[doc = "Field `CMP1_TEST` writer - Select output signal for cmp1"]
pub type Cmp1TestW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmp1Test, crate::Safe>;
impl<'a, REG> Cmp1TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1Test::Normal)
    }
    #[doc = "Connect out to AVDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1Test::Pullup)
    }
    #[doc = "Connect out to DVDD"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1Test::Pulldown)
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn zstate(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1Test::Zstate)
    }
}
#[doc = "Select output signal for reference output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RefTest {
    #[doc = "0: Tri-state output"]
    Zstate = 0,
    #[doc = "1: dac_ref for output(reserved)"]
    DacRef = 1,
    #[doc = "2: Connect external reference"]
    External = 2,
    #[doc = "3: Connect to internal reference"]
    Internal = 3,
}
impl From<RefTest> for u8 {
    #[inline(always)]
    fn from(variant: RefTest) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RefTest {
    type Ux = u8;
}
impl crate::IsEnum for RefTest {}
#[doc = "Field `REF_TEST` reader - Select output signal for reference output"]
pub type RefTestR = crate::FieldReader<RefTest>;
impl RefTestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefTest {
        match self.bits {
            0 => RefTest::Zstate,
            1 => RefTest::DacRef,
            2 => RefTest::External,
            3 => RefTest::Internal,
            _ => unreachable!(),
        }
    }
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn is_zstate(&self) -> bool {
        *self == RefTest::Zstate
    }
    #[doc = "dac_ref for output(reserved)"]
    #[inline(always)]
    pub fn is_dac_ref(&self) -> bool {
        *self == RefTest::DacRef
    }
    #[doc = "Connect external reference"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == RefTest::External
    }
    #[doc = "Connect to internal reference"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == RefTest::Internal
    }
}
#[doc = "Field `REF_TEST` writer - Select output signal for reference output"]
pub type RefTestW<'a, REG> = crate::FieldWriter<'a, REG, 2, RefTest, crate::Safe>;
impl<'a, REG> RefTestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Tri-state output"]
    #[inline(always)]
    pub fn zstate(self) -> &'a mut crate::W<REG> {
        self.variant(RefTest::Zstate)
    }
    #[doc = "dac_ref for output(reserved)"]
    #[inline(always)]
    pub fn dac_ref(self) -> &'a mut crate::W<REG> {
        self.variant(RefTest::DacRef)
    }
    #[doc = "Connect external reference"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(RefTest::External)
    }
    #[doc = "Connect to internal reference"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(RefTest::Internal)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select output signal for dac pad"]
    #[inline(always)]
    pub fn dac_out(&self) -> DacOutR {
        DacOutR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Select output signal for adc0"]
    #[inline(always)]
    pub fn adc0_test(&self) -> Adc0TestR {
        Adc0TestR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Select output signal for adc1"]
    #[inline(always)]
    pub fn adc1_test(&self) -> Adc1TestR {
        Adc1TestR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Select output signal for adc2"]
    #[inline(always)]
    pub fn adc2_test(&self) -> Adc2TestR {
        Adc2TestR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Select output signal for adc3"]
    #[inline(always)]
    pub fn adc3_test(&self) -> Adc3TestR {
        Adc3TestR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Select output signal for cmp0"]
    #[inline(always)]
    pub fn cmp0_test(&self) -> Cmp0TestR {
        Cmp0TestR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Select output signal for cmp1"]
    #[inline(always)]
    pub fn cmp1_test(&self) -> Cmp1TestR {
        Cmp1TestR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Select output signal for reference output"]
    #[inline(always)]
    pub fn ref_test(&self) -> RefTestR {
        RefTestR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select output signal for dac pad"]
    #[inline(always)]
    pub fn dac_out(&mut self) -> DacOutW<'_, TestSpec> {
        DacOutW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Select output signal for adc0"]
    #[inline(always)]
    pub fn adc0_test(&mut self) -> Adc0TestW<'_, TestSpec> {
        Adc0TestW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Select output signal for adc1"]
    #[inline(always)]
    pub fn adc1_test(&mut self) -> Adc1TestW<'_, TestSpec> {
        Adc1TestW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Select output signal for adc2"]
    #[inline(always)]
    pub fn adc2_test(&mut self) -> Adc2TestW<'_, TestSpec> {
        Adc2TestW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Select output signal for adc3"]
    #[inline(always)]
    pub fn adc3_test(&mut self) -> Adc3TestW<'_, TestSpec> {
        Adc3TestW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Select output signal for cmp0"]
    #[inline(always)]
    pub fn cmp0_test(&mut self) -> Cmp0TestW<'_, TestSpec> {
        Cmp0TestW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Select output signal for cmp1"]
    #[inline(always)]
    pub fn cmp1_test(&mut self) -> Cmp1TestW<'_, TestSpec> {
        Cmp1TestW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Select output signal for reference output"]
    #[inline(always)]
    pub fn ref_test(&mut self) -> RefTestW<'_, TestSpec> {
        RefTestW::new(self, 14)
    }
}
#[doc = "DAC test register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0x3ffc"]
impl crate::Resettable for TestSpec {
    const RESET_VALUE: u32 = 0x3ffc;
}
