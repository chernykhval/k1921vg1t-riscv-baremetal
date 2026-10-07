#[doc = "Register `ACMPCTL` reader"]
pub type R = crate::R<AcmpctlSpec>;
#[doc = "Register `ACMPCTL` writer"]
pub type W = crate::W<AcmpctlSpec>;
#[doc = "Field `DIS` reader - Disable comparator"]
pub type DisR = crate::BitReader;
#[doc = "Field `DIS` writer - Disable comparator"]
pub type DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTN` reader - Reset comparator (active low)"]
pub type RstnR = crate::BitReader;
#[doc = "Field `RSTN` writer - Reset comparator (active low)"]
pub type RstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - Inverse output"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - Inverse output"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select reference voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selref {
    #[doc = "0: Select IN_N as reference voltage"]
    Ext = 0,
    #[doc = "1: Select DAC channel 0 as reference voltage"]
    DacCh0 = 1,
    #[doc = "2: Select DAC channel 1 as reference voltage"]
    DacCh1 = 2,
    #[doc = "3: Reserved (unpredicted result)"]
    Rsrv = 3,
}
impl From<Selref> for u8 {
    #[inline(always)]
    fn from(variant: Selref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selref {
    type Ux = u8;
}
impl crate::IsEnum for Selref {}
#[doc = "Field `SELREF` reader - Select reference voltage"]
pub type SelrefR = crate::FieldReader<Selref>;
impl SelrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selref {
        match self.bits {
            0 => Selref::Ext,
            1 => Selref::DacCh0,
            2 => Selref::DacCh1,
            3 => Selref::Rsrv,
            _ => unreachable!(),
        }
    }
    #[doc = "Select IN_N as reference voltage"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == Selref::Ext
    }
    #[doc = "Select DAC channel 0 as reference voltage"]
    #[inline(always)]
    pub fn is_dac_ch0(&self) -> bool {
        *self == Selref::DacCh0
    }
    #[doc = "Select DAC channel 1 as reference voltage"]
    #[inline(always)]
    pub fn is_dac_ch1(&self) -> bool {
        *self == Selref::DacCh1
    }
    #[doc = "Reserved (unpredicted result)"]
    #[inline(always)]
    pub fn is_rsrv(&self) -> bool {
        *self == Selref::Rsrv
    }
}
#[doc = "Field `SELREF` writer - Select reference voltage"]
pub type SelrefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Selref, crate::Safe>;
impl<'a, REG> SelrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select IN_N as reference voltage"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Ext)
    }
    #[doc = "Select DAC channel 0 as reference voltage"]
    #[inline(always)]
    pub fn dac_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::DacCh0)
    }
    #[doc = "Select DAC channel 1 as reference voltage"]
    #[inline(always)]
    pub fn dac_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::DacCh1)
    }
    #[doc = "Reserved (unpredicted result)"]
    #[inline(always)]
    pub fn rsrv(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Rsrv)
    }
}
#[doc = "Select input for IN0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selin {
    #[doc = "0: Selects IN_P"]
    Input = 0,
    #[doc = "1: Select test out from DAC"]
    Test = 1,
}
impl From<Selin> for bool {
    #[inline(always)]
    fn from(variant: Selin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELIN` reader - Select input for IN0"]
pub type SelinR = crate::BitReader<Selin>;
impl SelinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selin {
        match self.bits {
            false => Selin::Input,
            true => Selin::Test,
        }
    }
    #[doc = "Selects IN_P"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Selin::Input
    }
    #[doc = "Select test out from DAC"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == Selin::Test
    }
}
#[doc = "Field `SELIN` writer - Select input for IN0"]
pub type SelinW<'a, REG> = crate::BitWriter<'a, REG, Selin>;
impl<'a, REG> SelinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects IN_P"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Selin::Input)
    }
    #[doc = "Select test out from DAC"]
    #[inline(always)]
    pub fn test(self) -> &'a mut crate::W<REG> {
        self.variant(Selin::Test)
    }
}
#[doc = "Field `INTSRC` reader - Select interrupt source"]
pub type IntsrcR = crate::FieldReader;
#[doc = "Field `INTSRC` writer - Select interrupt source"]
pub type IntsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIGSRC` reader - Select trigger event"]
pub type TrigsrcR = crate::FieldReader;
#[doc = "Field `TRIGSRC` writer - Select trigger event"]
pub type TrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Disable comparator"]
    #[inline(always)]
    pub fn dis(&self) -> DisR {
        DisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset comparator (active low)"]
    #[inline(always)]
    pub fn rstn(&self) -> RstnR {
        RstnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Inverse output"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select reference voltage"]
    #[inline(always)]
    pub fn selref(&self) -> SelrefR {
        SelrefR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Select input for IN0"]
    #[inline(always)]
    pub fn selin(&self) -> SelinR {
        SelinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Select interrupt source"]
    #[inline(always)]
    pub fn intsrc(&self) -> IntsrcR {
        IntsrcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select trigger event"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable comparator"]
    #[inline(always)]
    pub fn dis(&mut self) -> DisW<'_, AcmpctlSpec> {
        DisW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset comparator (active low)"]
    #[inline(always)]
    pub fn rstn(&mut self) -> RstnW<'_, AcmpctlSpec> {
        RstnW::new(self, 1)
    }
    #[doc = "Bit 2 - Inverse output"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, AcmpctlSpec> {
        InvW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Select reference voltage"]
    #[inline(always)]
    pub fn selref(&mut self) -> SelrefW<'_, AcmpctlSpec> {
        SelrefW::new(self, 4)
    }
    #[doc = "Bit 6 - Select input for IN0"]
    #[inline(always)]
    pub fn selin(&mut self) -> SelinW<'_, AcmpctlSpec> {
        SelinW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Select interrupt source"]
    #[inline(always)]
    pub fn intsrc(&mut self) -> IntsrcW<'_, AcmpctlSpec> {
        IntsrcW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Select trigger event"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TrigsrcW<'_, AcmpctlSpec> {
        TrigsrcW::new(self, 12)
    }
}
#[doc = "ACMP control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acmpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcmpctlSpec;
impl crate::RegisterSpec for AcmpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acmpctl::R`](R) reader structure"]
impl crate::Readable for AcmpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`acmpctl::W`](W) writer structure"]
impl crate::Writable for AcmpctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACMPCTL to value 0x01"]
impl crate::Resettable for AcmpctlSpec {
    const RESET_VALUE: u32 = 0x01;
}
