#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Size of data is (value + 1) bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    #[doc = "3: data size 4 bits"]
    _4bit = 3,
    #[doc = "4: data size 5 bits"]
    _5bit = 4,
    #[doc = "5: data size 6 bits"]
    _6bit = 5,
    #[doc = "6: data size 7 bits"]
    _7bit = 6,
    #[doc = "7: data size 8 bits"]
    _8bit = 7,
    #[doc = "8: data size 9 bits"]
    _9bit = 8,
    #[doc = "9: data size 10 bits"]
    _10bit = 9,
    #[doc = "10: data size 11 bits"]
    _11bit = 10,
    #[doc = "11: data size 12 bits"]
    _12bit = 11,
    #[doc = "12: data size 13 bits"]
    _13bit = 12,
    #[doc = "13: data size 14 bits"]
    _14bit = 13,
    #[doc = "14: data size 15 bits"]
    _15bit = 14,
    #[doc = "15: data size 16 bits"]
    _16bit = 15,
    #[doc = "16: data size 17 bits"]
    _17bit = 16,
    #[doc = "17: data size 18 bits"]
    _18bit = 17,
    #[doc = "18: data size 19 bits"]
    _19bit = 18,
    #[doc = "19: data size 20 bits"]
    _20bit = 19,
    #[doc = "20: data size 21 bits"]
    _21bit = 20,
    #[doc = "21: data size 22 bits"]
    _22bit = 21,
    #[doc = "22: data size 23 bits"]
    _23bit = 22,
    #[doc = "23: data size 24 bits"]
    _24bit = 23,
    #[doc = "24: data size 25 bits"]
    _25bit = 24,
    #[doc = "25: data size 26 bits"]
    _26bit = 25,
    #[doc = "26: data size 27 bits"]
    _27bit = 26,
    #[doc = "27: data size 28 bits"]
    _28bit = 27,
    #[doc = "28: data size 29 bits"]
    _29bit = 28,
    #[doc = "29: data size 30 bits"]
    _30bit = 29,
    #[doc = "30: data size 31 bits"]
    _31bit = 30,
    #[doc = "31: data size 32 bits"]
    _32bit = 31,
}
impl From<Dss> for u8 {
    #[inline(always)]
    fn from(variant: Dss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dss {
    type Ux = u8;
}
impl crate::IsEnum for Dss {}
#[doc = "Field `DSS` reader - Size of data is (value + 1) bits"]
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            3 => Some(Dss::_4bit),
            4 => Some(Dss::_5bit),
            5 => Some(Dss::_6bit),
            6 => Some(Dss::_7bit),
            7 => Some(Dss::_8bit),
            8 => Some(Dss::_9bit),
            9 => Some(Dss::_10bit),
            10 => Some(Dss::_11bit),
            11 => Some(Dss::_12bit),
            12 => Some(Dss::_13bit),
            13 => Some(Dss::_14bit),
            14 => Some(Dss::_15bit),
            15 => Some(Dss::_16bit),
            16 => Some(Dss::_17bit),
            17 => Some(Dss::_18bit),
            18 => Some(Dss::_19bit),
            19 => Some(Dss::_20bit),
            20 => Some(Dss::_21bit),
            21 => Some(Dss::_22bit),
            22 => Some(Dss::_23bit),
            23 => Some(Dss::_24bit),
            24 => Some(Dss::_25bit),
            25 => Some(Dss::_26bit),
            26 => Some(Dss::_27bit),
            27 => Some(Dss::_28bit),
            28 => Some(Dss::_29bit),
            29 => Some(Dss::_30bit),
            30 => Some(Dss::_31bit),
            31 => Some(Dss::_32bit),
            _ => None,
        }
    }
    #[doc = "data size 4 bits"]
    #[inline(always)]
    pub fn is_4bit(&self) -> bool {
        *self == Dss::_4bit
    }
    #[doc = "data size 5 bits"]
    #[inline(always)]
    pub fn is_5bit(&self) -> bool {
        *self == Dss::_5bit
    }
    #[doc = "data size 6 bits"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == Dss::_6bit
    }
    #[doc = "data size 7 bits"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Dss::_7bit
    }
    #[doc = "data size 8 bits"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Dss::_8bit
    }
    #[doc = "data size 9 bits"]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == Dss::_9bit
    }
    #[doc = "data size 10 bits"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Dss::_10bit
    }
    #[doc = "data size 11 bits"]
    #[inline(always)]
    pub fn is_11bit(&self) -> bool {
        *self == Dss::_11bit
    }
    #[doc = "data size 12 bits"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Dss::_12bit
    }
    #[doc = "data size 13 bits"]
    #[inline(always)]
    pub fn is_13bit(&self) -> bool {
        *self == Dss::_13bit
    }
    #[doc = "data size 14 bits"]
    #[inline(always)]
    pub fn is_14bit(&self) -> bool {
        *self == Dss::_14bit
    }
    #[doc = "data size 15 bits"]
    #[inline(always)]
    pub fn is_15bit(&self) -> bool {
        *self == Dss::_15bit
    }
    #[doc = "data size 16 bits"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Dss::_16bit
    }
    #[doc = "data size 17 bits"]
    #[inline(always)]
    pub fn is_17bit(&self) -> bool {
        *self == Dss::_17bit
    }
    #[doc = "data size 18 bits"]
    #[inline(always)]
    pub fn is_18bit(&self) -> bool {
        *self == Dss::_18bit
    }
    #[doc = "data size 19 bits"]
    #[inline(always)]
    pub fn is_19bit(&self) -> bool {
        *self == Dss::_19bit
    }
    #[doc = "data size 20 bits"]
    #[inline(always)]
    pub fn is_20bit(&self) -> bool {
        *self == Dss::_20bit
    }
    #[doc = "data size 21 bits"]
    #[inline(always)]
    pub fn is_21bit(&self) -> bool {
        *self == Dss::_21bit
    }
    #[doc = "data size 22 bits"]
    #[inline(always)]
    pub fn is_22bit(&self) -> bool {
        *self == Dss::_22bit
    }
    #[doc = "data size 23 bits"]
    #[inline(always)]
    pub fn is_23bit(&self) -> bool {
        *self == Dss::_23bit
    }
    #[doc = "data size 24 bits"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == Dss::_24bit
    }
    #[doc = "data size 25 bits"]
    #[inline(always)]
    pub fn is_25bit(&self) -> bool {
        *self == Dss::_25bit
    }
    #[doc = "data size 26 bits"]
    #[inline(always)]
    pub fn is_26bit(&self) -> bool {
        *self == Dss::_26bit
    }
    #[doc = "data size 27 bits"]
    #[inline(always)]
    pub fn is_27bit(&self) -> bool {
        *self == Dss::_27bit
    }
    #[doc = "data size 28 bits"]
    #[inline(always)]
    pub fn is_28bit(&self) -> bool {
        *self == Dss::_28bit
    }
    #[doc = "data size 29 bits"]
    #[inline(always)]
    pub fn is_29bit(&self) -> bool {
        *self == Dss::_29bit
    }
    #[doc = "data size 30 bits"]
    #[inline(always)]
    pub fn is_30bit(&self) -> bool {
        *self == Dss::_30bit
    }
    #[doc = "data size 31 bits"]
    #[inline(always)]
    pub fn is_31bit(&self) -> bool {
        *self == Dss::_31bit
    }
    #[doc = "data size 32 bits"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == Dss::_32bit
    }
}
#[doc = "Field `DSS` writer - Size of data is (value + 1) bits"]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "data size 4 bits"]
    #[inline(always)]
    pub fn _4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_4bit)
    }
    #[doc = "data size 5 bits"]
    #[inline(always)]
    pub fn _5bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_5bit)
    }
    #[doc = "data size 6 bits"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_6bit)
    }
    #[doc = "data size 7 bits"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_7bit)
    }
    #[doc = "data size 8 bits"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_8bit)
    }
    #[doc = "data size 9 bits"]
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_9bit)
    }
    #[doc = "data size 10 bits"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_10bit)
    }
    #[doc = "data size 11 bits"]
    #[inline(always)]
    pub fn _11bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_11bit)
    }
    #[doc = "data size 12 bits"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_12bit)
    }
    #[doc = "data size 13 bits"]
    #[inline(always)]
    pub fn _13bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_13bit)
    }
    #[doc = "data size 14 bits"]
    #[inline(always)]
    pub fn _14bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_14bit)
    }
    #[doc = "data size 15 bits"]
    #[inline(always)]
    pub fn _15bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_15bit)
    }
    #[doc = "data size 16 bits"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_16bit)
    }
    #[doc = "data size 17 bits"]
    #[inline(always)]
    pub fn _17bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_17bit)
    }
    #[doc = "data size 18 bits"]
    #[inline(always)]
    pub fn _18bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_18bit)
    }
    #[doc = "data size 19 bits"]
    #[inline(always)]
    pub fn _19bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_19bit)
    }
    #[doc = "data size 20 bits"]
    #[inline(always)]
    pub fn _20bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_20bit)
    }
    #[doc = "data size 21 bits"]
    #[inline(always)]
    pub fn _21bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_21bit)
    }
    #[doc = "data size 22 bits"]
    #[inline(always)]
    pub fn _22bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_22bit)
    }
    #[doc = "data size 23 bits"]
    #[inline(always)]
    pub fn _23bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_23bit)
    }
    #[doc = "data size 24 bits"]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_24bit)
    }
    #[doc = "data size 25 bits"]
    #[inline(always)]
    pub fn _25bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_25bit)
    }
    #[doc = "data size 26 bits"]
    #[inline(always)]
    pub fn _26bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_26bit)
    }
    #[doc = "data size 27 bits"]
    #[inline(always)]
    pub fn _27bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_27bit)
    }
    #[doc = "data size 28 bits"]
    #[inline(always)]
    pub fn _28bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_28bit)
    }
    #[doc = "data size 29 bits"]
    #[inline(always)]
    pub fn _29bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_29bit)
    }
    #[doc = "data size 30 bits"]
    #[inline(always)]
    pub fn _30bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_30bit)
    }
    #[doc = "data size 31 bits"]
    #[inline(always)]
    pub fn _31bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_31bit)
    }
    #[doc = "data size 32 bits"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_32bit)
    }
}
#[doc = "Field `SCR` reader - Value divider"]
pub type ScrR = crate::FieldReader;
#[doc = "Field `SCR` writer - Value divider"]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Select protocol\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "0: SPI of Motorola"]
    Spi = 0,
    #[doc = "1: SSI of Texas Instruments"]
    Ssi = 1,
    #[doc = "2: Microwire of National Semiconductor"]
    Microwire = 2,
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frf {
    type Ux = u8;
}
impl crate::IsEnum for Frf {}
#[doc = "Field `FRF` reader - Select protocol"]
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frf> {
        match self.bits {
            0 => Some(Frf::Spi),
            1 => Some(Frf::Ssi),
            2 => Some(Frf::Microwire),
            _ => None,
        }
    }
    #[doc = "SPI of Motorola"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Frf::Spi
    }
    #[doc = "SSI of Texas Instruments"]
    #[inline(always)]
    pub fn is_ssi(&self) -> bool {
        *self == Frf::Ssi
    }
    #[doc = "Microwire of National Semiconductor"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == Frf::Microwire
    }
}
#[doc = "Field `FRF` writer - Select protocol"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI of Motorola"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Spi)
    }
    #[doc = "SSI of Texas Instruments"]
    #[inline(always)]
    pub fn ssi(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Ssi)
    }
    #[doc = "Microwire of National Semiconductor"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Microwire)
    }
}
#[doc = "Field `SPO` reader - Polarity SSPCLKOUT"]
pub type SpoR = crate::BitReader;
#[doc = "Field `SPO` writer - Polarity SSPCLKOUT"]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPH` reader - Phase SSPCLKOUT"]
pub type SphR = crate::BitReader;
#[doc = "Field `SPH` writer - Phase SSPCLKOUT"]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBM` reader - Test loopback mode"]
pub type LbmR = crate::BitReader;
#[doc = "Field `LBM` writer - Test loopback mode"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE` reader - Enable transceiver"]
pub type SseR = crate::BitReader;
#[doc = "Field `SSE` writer - Enable transceiver"]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - Select mode"]
pub type MsR = crate::BitReader;
#[doc = "Field `MS` writer - Select mode"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOD` reader - Disable bit data"]
pub type SodR = crate::BitReader;
#[doc = "Field `SOD` writer - Disable bit data"]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIFLSEL` reader - Receive interrupt FIFO level select"]
pub type RxiflselR = crate::FieldReader;
#[doc = "Field `RXIFLSEL` writer - Receive interrupt FIFO level select"]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXIFLSEL` reader - Transmit interrupt FIFO level select"]
pub type TxiflselR = crate::FieldReader;
#[doc = "Field `TXIFLSEL` writer - Transmit interrupt FIFO level select"]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Size of data is (value + 1) bits"]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Value divider"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Select protocol"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Polarity SSPCLKOUT"]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Phase SSPCLKOUT"]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Test loopback mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable transceiver"]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Select mode"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable bit data"]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Size of data is (value + 1) bits"]
    #[inline(always)]
    pub fn dss(&mut self) -> DssW<'_, CrSpec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Value divider"]
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<'_, CrSpec> {
        ScrW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Select protocol"]
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<'_, CrSpec> {
        FrfW::new(self, 16)
    }
    #[doc = "Bit 18 - Polarity SSPCLKOUT"]
    #[inline(always)]
    pub fn spo(&mut self) -> SpoW<'_, CrSpec> {
        SpoW::new(self, 18)
    }
    #[doc = "Bit 19 - Phase SSPCLKOUT"]
    #[inline(always)]
    pub fn sph(&mut self) -> SphW<'_, CrSpec> {
        SphW::new(self, 19)
    }
    #[doc = "Bit 20 - Test loopback mode"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<'_, CrSpec> {
        LbmW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable transceiver"]
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<'_, CrSpec> {
        SseW::new(self, 21)
    }
    #[doc = "Bit 22 - Select mode"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, CrSpec> {
        MsW::new(self, 22)
    }
    #[doc = "Bit 23 - Disable bit data"]
    #[inline(always)]
    pub fn sod(&mut self) -> SodW<'_, CrSpec> {
        SodW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RxiflselW<'_, CrSpec> {
        RxiflselW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TxiflselW<'_, CrSpec> {
        TxiflselW::new(self, 28)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x4400_0000"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x4400_0000;
}
