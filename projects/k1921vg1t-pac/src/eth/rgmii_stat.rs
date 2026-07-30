#[doc = "Register `RGMII_STAT` reader"]
pub type R = crate::R<RgmiiStatSpec>;
#[doc = "Link Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ls {
    #[doc = "0: down"]
    Down = 0,
    #[doc = "1: up"]
    Up = 1,
}
impl From<Ls> for bool {
    #[inline(always)]
    fn from(variant: Ls) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LS` reader - Link Status"]
pub type LsR = crate::BitReader<Ls>;
impl LsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ls {
        match self.bits {
            false => Ls::Down,
            true => Ls::Up,
        }
    }
    #[doc = "down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Ls::Down
    }
    #[doc = "up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Ls::Up
    }
}
#[doc = "Line Clock Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcs {
    #[doc = "0: 2.5 MHz RXC Clock speed"]
    _10mbit = 0,
    #[doc = "1: 25 MHz RXC Clock speed"]
    _100mbit = 1,
    #[doc = "2: 125MHz RXC Clock speed"]
    _1g = 2,
}
impl From<Lcs> for u8 {
    #[inline(always)]
    fn from(variant: Lcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcs {
    type Ux = u8;
}
impl crate::IsEnum for Lcs {}
#[doc = "Field `LCS` reader - Line Clock Speed"]
pub type LcsR = crate::FieldReader<Lcs>;
impl LcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcs {
        match self.bits {
            0 => Lcs::_10mbit,
            1 => Lcs::_100mbit,
            2 => Lcs::_1g,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 MHz RXC Clock speed"]
    #[inline(always)]
    pub fn is_10mbit(&self) -> bool {
        *self == Lcs::_10mbit
    }
    #[doc = "25 MHz RXC Clock speed"]
    #[inline(always)]
    pub fn is_100mbit(&self) -> bool {
        *self == Lcs::_100mbit
    }
    #[doc = "125MHz RXC Clock speed"]
    #[inline(always)]
    pub fn is_1g(&self) -> bool {
        *self == Lcs::_1g
    }
}
#[doc = "Duplex Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ds {
    #[doc = "0: Half-duplex status"]
    HalfDuplex = 0,
    #[doc = "1: Full-duplex status"]
    FullDuplex = 1,
}
impl From<Ds> for bool {
    #[inline(always)]
    fn from(variant: Ds) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS` reader - Duplex Status"]
pub type DsR = crate::BitReader<Ds>;
impl DsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds {
        match self.bits {
            false => Ds::HalfDuplex,
            true => Ds::FullDuplex,
        }
    }
    #[doc = "Half-duplex status"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == Ds::HalfDuplex
    }
    #[doc = "Full-duplex status"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == Ds::FullDuplex
    }
}
#[doc = "Field `FC` reader - False Carrier is detected"]
pub type FcR = crate::BitReader;
#[doc = "Field `CX` reader - Carrier extended"]
pub type CxR = crate::BitReader;
#[doc = "Field `CE` reader - Carrier extended frames error"]
pub type CeR = crate::BitReader;
#[doc = "Field `CS` reader - Carrier sence"]
pub type CsR = crate::BitReader;
#[doc = "Field `SPEED` reader - MAC Speed"]
pub type SpeedR = crate::BitReader;
#[doc = "Field `GB` reader - Gigabit"]
pub type GbR = crate::BitReader;
#[doc = "Field `GA` reader - Gigabit MAC available"]
pub type GaR = crate::BitReader;
#[doc = "Field `RD` reader - RAM debug enable"]
pub type RdR = crate::BitReader;
#[doc = "Field `NM` reader - No Clock Mux mode"]
pub type NmR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Link Status"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Line Clock Speed"]
    #[inline(always)]
    pub fn lcs(&self) -> LcsR {
        LcsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Duplex Status"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - False Carrier is detected"]
    #[inline(always)]
    pub fn fc(&self) -> FcR {
        FcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Carrier extended"]
    #[inline(always)]
    pub fn cx(&self) -> CxR {
        CxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Carrier extended frames error"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Carrier sence"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MAC Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Gigabit"]
    #[inline(always)]
    pub fn gb(&self) -> GbR {
        GbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Gigabit MAC available"]
    #[inline(always)]
    pub fn ga(&self) -> GaR {
        GaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RAM debug enable"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - No Clock Mux mode"]
    #[inline(always)]
    pub fn nm(&self) -> NmR {
        NmR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "RGMII Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgmii_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgmiiStatSpec;
impl crate::RegisterSpec for RgmiiStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgmii_stat::R`](R) reader structure"]
impl crate::Readable for RgmiiStatSpec {}
#[doc = "`reset()` method sets RGMII_STAT to value 0"]
impl crate::Resettable for RgmiiStatSpec {}
