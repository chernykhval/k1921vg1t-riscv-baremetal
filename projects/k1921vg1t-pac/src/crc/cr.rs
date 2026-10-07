#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RESET` writer - RESET bit"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode calc CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Standart mode"]
    StandartCrc = 0,
    #[doc = "1: STM32 compatible mode"]
    Stm32Crc = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Mode calc CRC"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::StandartCrc,
            true => Mode::Stm32Crc,
        }
    }
    #[doc = "Standart mode"]
    #[inline(always)]
    pub fn is_standart_crc(&self) -> bool {
        *self == Mode::StandartCrc
    }
    #[doc = "STM32 compatible mode"]
    #[inline(always)]
    pub fn is_stm32_crc(&self) -> bool {
        *self == Mode::Stm32Crc
    }
}
#[doc = "Field `MODE` writer - Mode calc CRC"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standart mode"]
    #[inline(always)]
    pub fn standart_crc(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::StandartCrc)
    }
    #[doc = "STM32 compatible mode"]
    #[inline(always)]
    pub fn stm32_crc(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Stm32Crc)
    }
}
#[doc = "XOR for data out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xorout {
    #[doc = "0: Output XOR operation is disabled"]
    Disable = 0,
    #[doc = "1: Output XOR operation is enabled"]
    Enable = 1,
}
impl From<Xorout> for bool {
    #[inline(always)]
    fn from(variant: Xorout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XOROUT` reader - XOR for data out"]
pub type XoroutR = crate::BitReader<Xorout>;
impl XoroutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xorout {
        match self.bits {
            false => Xorout::Disable,
            true => Xorout::Enable,
        }
    }
    #[doc = "Output XOR operation is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Xorout::Disable
    }
    #[doc = "Output XOR operation is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Xorout::Enable
    }
}
#[doc = "Field `XOROUT` writer - XOR for data out"]
pub type XoroutW<'a, REG> = crate::BitWriter<'a, REG, Xorout>;
impl<'a, REG> XoroutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output XOR operation is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Xorout::Disable)
    }
    #[doc = "Output XOR operation is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Xorout::Enable)
    }
}
#[doc = "Polynomial size. These bits control the size of the polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polysize {
    #[doc = "0: 32 bit polynomial"]
    Pol32 = 0,
    #[doc = "1: 16 bit polynomial"]
    Pol16 = 1,
    #[doc = "2: 8 bit polynomial"]
    Pol8 = 2,
    #[doc = "3: 7 bit polynomial"]
    Pol7 = 3,
}
impl From<Polysize> for u8 {
    #[inline(always)]
    fn from(variant: Polysize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polysize {
    type Ux = u8;
}
impl crate::IsEnum for Polysize {}
#[doc = "Field `POLYSIZE` reader - Polynomial size. These bits control the size of the polynomial"]
pub type PolysizeR = crate::FieldReader<Polysize>;
impl PolysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polysize {
        match self.bits {
            0 => Polysize::Pol32,
            1 => Polysize::Pol16,
            2 => Polysize::Pol8,
            3 => Polysize::Pol7,
            _ => unreachable!(),
        }
    }
    #[doc = "32 bit polynomial"]
    #[inline(always)]
    pub fn is_pol32(&self) -> bool {
        *self == Polysize::Pol32
    }
    #[doc = "16 bit polynomial"]
    #[inline(always)]
    pub fn is_pol16(&self) -> bool {
        *self == Polysize::Pol16
    }
    #[doc = "8 bit polynomial"]
    #[inline(always)]
    pub fn is_pol8(&self) -> bool {
        *self == Polysize::Pol8
    }
    #[doc = "7 bit polynomial"]
    #[inline(always)]
    pub fn is_pol7(&self) -> bool {
        *self == Polysize::Pol7
    }
}
#[doc = "Reverse input data. These bits control the reversal of the bit order of the input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RevIn {
    #[doc = "0: Bit order not affected(default)"]
    Disable = 0,
    #[doc = "1: Bit reversal done by byte"]
    RevByte = 1,
    #[doc = "2: Bit reversal done by half-word"]
    RevHalfword = 2,
    #[doc = "3: Bit reversal done by word"]
    RevWord = 3,
}
impl From<RevIn> for u8 {
    #[inline(always)]
    fn from(variant: RevIn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RevIn {
    type Ux = u8;
}
impl crate::IsEnum for RevIn {}
#[doc = "Field `REV_IN` reader - Reverse input data. These bits control the reversal of the bit order of the input data"]
pub type RevInR = crate::FieldReader<RevIn>;
impl RevInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RevIn {
        match self.bits {
            0 => RevIn::Disable,
            1 => RevIn::RevByte,
            2 => RevIn::RevHalfword,
            3 => RevIn::RevWord,
            _ => unreachable!(),
        }
    }
    #[doc = "Bit order not affected(default)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RevIn::Disable
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn is_rev_byte(&self) -> bool {
        *self == RevIn::RevByte
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn is_rev_halfword(&self) -> bool {
        *self == RevIn::RevHalfword
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn is_rev_word(&self) -> bool {
        *self == RevIn::RevWord
    }
}
#[doc = "Field `REV_IN` writer - Reverse input data. These bits control the reversal of the bit order of the input data"]
pub type RevInW<'a, REG> = crate::FieldWriter<'a, REG, 2, RevIn, crate::Safe>;
impl<'a, REG> RevInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bit order not affected(default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RevIn::Disable)
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn rev_byte(self) -> &'a mut crate::W<REG> {
        self.variant(RevIn::RevByte)
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn rev_halfword(self) -> &'a mut crate::W<REG> {
        self.variant(RevIn::RevHalfword)
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn rev_word(self) -> &'a mut crate::W<REG> {
        self.variant(RevIn::RevWord)
    }
}
#[doc = "Reverse output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RevOut {
    #[doc = "0: Bit order not affected"]
    Disable = 0,
    #[doc = "1: Bit-reversed output format"]
    Enable = 1,
}
impl From<RevOut> for bool {
    #[inline(always)]
    fn from(variant: RevOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV_OUT` reader - Reverse output data"]
pub type RevOutR = crate::BitReader<RevOut>;
impl RevOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RevOut {
        match self.bits {
            false => RevOut::Disable,
            true => RevOut::Enable,
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RevOut::Disable
    }
    #[doc = "Bit-reversed output format"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RevOut::Enable
    }
}
#[doc = "Field `REV_OUT` writer - Reverse output data"]
pub type RevOutW<'a, REG> = crate::BitWriter<'a, REG, RevOut>;
impl<'a, REG> RevOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RevOut::Disable)
    }
    #[doc = "Bit-reversed output format"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RevOut::Enable)
    }
}
impl R {
    #[doc = "Bit 1 - Mode calc CRC"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOR for data out"]
    #[inline(always)]
    pub fn xorout(&self) -> XoroutR {
        XoroutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Polynomial size. These bits control the size of the polynomial"]
    #[inline(always)]
    pub fn polysize(&self) -> PolysizeR {
        PolysizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data. These bits control the reversal of the bit order of the input data"]
    #[inline(always)]
    pub fn rev_in(&self) -> RevInR {
        RevInR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_out(&self) -> RevOutR {
        RevOutR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET bit"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, CrSpec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bit 1 - Mode calc CRC"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CrSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - XOR for data out"]
    #[inline(always)]
    pub fn xorout(&mut self) -> XoroutW<'_, CrSpec> {
        XoroutW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Reverse input data. These bits control the reversal of the bit order of the input data"]
    #[inline(always)]
    pub fn rev_in(&mut self) -> RevInW<'_, CrSpec> {
        RevInW::new(self, 5)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_out(&mut self) -> RevOutW<'_, CrSpec> {
        RevOutW::new(self, 7)
    }
}
#[doc = "CRC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
