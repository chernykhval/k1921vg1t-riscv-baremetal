#[doc = "Register `TACC0` reader"]
pub type R = crate::R<Tacc0Spec>;
#[doc = "Register `TACC0` writer"]
pub type W = crate::W<Tacc0Spec>;
#[doc = "Field `SOC` reader - Start ADC conversion"]
pub type SocR = crate::BitReader;
#[doc = "Field `SOC` writer - Start ADC conversion"]
pub type SocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REARM` reader - Rearm for ADC module"]
pub type RearmR = crate::BitReader;
#[doc = "Field `REARM` writer - Rearm for ADC module"]
pub type RearmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL` reader - Select channel for testing"]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - Select channel for testing"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "ADC resolution select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selres {
    #[doc = "0: 6 bit mode"]
    _6bit = 0,
    #[doc = "1: 8 bit mode"]
    _8bit = 1,
    #[doc = "2: 10 bit mode"]
    _10bit = 2,
    #[doc = "3: 12 bit mode"]
    _12bit = 3,
}
impl From<Selres> for u8 {
    #[inline(always)]
    fn from(variant: Selres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selres {
    type Ux = u8;
}
impl crate::IsEnum for Selres {}
#[doc = "Field `SELRES` reader - ADC resolution select"]
pub type SelresR = crate::FieldReader<Selres>;
impl SelresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selres {
        match self.bits {
            0 => Selres::_6bit,
            1 => Selres::_8bit,
            2 => Selres::_10bit,
            3 => Selres::_12bit,
            _ => unreachable!(),
        }
    }
    #[doc = "6 bit mode"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Selres::_6bit
    }
    #[doc = "8 bit mode"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Selres::_8bit
    }
    #[doc = "10 bit mode"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Selres::_10bit
    }
    #[doc = "12 bit mode"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Selres::_12bit
    }
}
#[doc = "Field `SELRES` writer - ADC resolution select"]
pub type SelresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Selres, crate::Safe>;
impl<'a, REG> SelresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "6 bit mode"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Selres::_6bit)
    }
    #[doc = "8 bit mode"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Selres::_8bit)
    }
    #[doc = "10 bit mode"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Selres::_10bit)
    }
    #[doc = "12 bit mode"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Selres::_12bit)
    }
}
#[doc = "Field `RSTADC` reader - Reset ADC module"]
pub type RstadcR = crate::BitReader;
#[doc = "Field `RSTADC` writer - Reset ADC module"]
pub type RstadcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPCAL` reader - Disable bypass calibration of ADC module"]
pub type BpcalR = crate::BitReader;
#[doc = "Field `BPCAL` writer - Disable bypass calibration of ADC module"]
pub type BpcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTCAL` reader - Reset ADC calibration scheme impulse"]
pub type RstcalR = crate::BitReader;
#[doc = "Field `RSTCAL` writer - Reset ADC calibration scheme impulse"]
pub type RstcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOADCAL` reader - Load value to ADC internal calibration scheme"]
pub type LoadcalR = crate::BitReader;
#[doc = "Field `LOADCAL` writer - Load value to ADC internal calibration scheme"]
pub type LoadcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTCAL` reader - Start ADC internal calibration"]
pub type StartcalR = crate::BitReader;
#[doc = "Field `STARTCAL` writer - Start ADC internal calibration"]
pub type StartcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - Write key 0xFAAC to get access to TACC0 register"]
pub type KeyR = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - Write key 0xFAAC to get access to TACC0 register"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Start ADC conversion"]
    #[inline(always)]
    pub fn soc(&self) -> SocR {
        SocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rearm for ADC module"]
    #[inline(always)]
    pub fn rearm(&self) -> RearmR {
        RearmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Select channel for testing"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - ADC resolution select"]
    #[inline(always)]
    pub fn selres(&self) -> SelresR {
        SelresR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Reset ADC module"]
    #[inline(always)]
    pub fn rstadc(&self) -> RstadcR {
        RstadcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable bypass calibration of ADC module"]
    #[inline(always)]
    pub fn bpcal(&self) -> BpcalR {
        BpcalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset ADC calibration scheme impulse"]
    #[inline(always)]
    pub fn rstcal(&self) -> RstcalR {
        RstcalR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Load value to ADC internal calibration scheme"]
    #[inline(always)]
    pub fn loadcal(&self) -> LoadcalR {
        LoadcalR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Start ADC internal calibration"]
    #[inline(always)]
    pub fn startcal(&self) -> StartcalR {
        StartcalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Write key 0xFAAC to get access to TACC0 register"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Start ADC conversion"]
    #[inline(always)]
    pub fn soc(&mut self) -> SocW<'_, Tacc0Spec> {
        SocW::new(self, 0)
    }
    #[doc = "Bit 1 - Rearm for ADC module"]
    #[inline(always)]
    pub fn rearm(&mut self) -> RearmW<'_, Tacc0Spec> {
        RearmW::new(self, 1)
    }
    #[doc = "Bits 4:7 - Select channel for testing"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, Tacc0Spec> {
        SelW::new(self, 4)
    }
    #[doc = "Bits 8:9 - ADC resolution select"]
    #[inline(always)]
    pub fn selres(&mut self) -> SelresW<'_, Tacc0Spec> {
        SelresW::new(self, 8)
    }
    #[doc = "Bit 10 - Reset ADC module"]
    #[inline(always)]
    pub fn rstadc(&mut self) -> RstadcW<'_, Tacc0Spec> {
        RstadcW::new(self, 10)
    }
    #[doc = "Bit 11 - Disable bypass calibration of ADC module"]
    #[inline(always)]
    pub fn bpcal(&mut self) -> BpcalW<'_, Tacc0Spec> {
        BpcalW::new(self, 11)
    }
    #[doc = "Bit 12 - Reset ADC calibration scheme impulse"]
    #[inline(always)]
    pub fn rstcal(&mut self) -> RstcalW<'_, Tacc0Spec> {
        RstcalW::new(self, 12)
    }
    #[doc = "Bit 13 - Load value to ADC internal calibration scheme"]
    #[inline(always)]
    pub fn loadcal(&mut self) -> LoadcalW<'_, Tacc0Spec> {
        LoadcalW::new(self, 13)
    }
    #[doc = "Bit 14 - Start ADC internal calibration"]
    #[inline(always)]
    pub fn startcal(&mut self) -> StartcalW<'_, Tacc0Spec> {
        StartcalW::new(self, 14)
    }
    #[doc = "Bits 16:31 - Write key 0xFAAC to get access to TACC0 register"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Tacc0Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "ADC test access register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tacc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tacc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tacc0Spec;
impl crate::RegisterSpec for Tacc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tacc0::R`](R) reader structure"]
impl crate::Readable for Tacc0Spec {}
#[doc = "`write(|w| ..)` method takes [`tacc0::W`](W) writer structure"]
impl crate::Writable for Tacc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TACC0 to value 0"]
impl crate::Resettable for Tacc0Spec {}
