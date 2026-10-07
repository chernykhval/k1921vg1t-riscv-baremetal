#[doc = "Register `RGMII_IM` reader"]
pub type R = crate::R<RgmiiImSpec>;
#[doc = "Register `RGMII_IM` writer"]
pub type W = crate::W<RgmiiImSpec>;
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
#[doc = "Field `LS` writer - Link Status"]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG, Ls>;
impl<'a, REG> LsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Ls::Down)
    }
    #[doc = "up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Ls::Up)
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
#[doc = "Field `LCS` writer - Line Clock Speed"]
pub type LcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lcs>;
impl<'a, REG> LcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 MHz RXC Clock speed"]
    #[inline(always)]
    pub fn _10mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Lcs::_10mbit)
    }
    #[doc = "25 MHz RXC Clock speed"]
    #[inline(always)]
    pub fn _100mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Lcs::_100mbit)
    }
    #[doc = "125MHz RXC Clock speed"]
    #[inline(always)]
    pub fn _1g(self) -> &'a mut crate::W<REG> {
        self.variant(Lcs::_1g)
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
#[doc = "Field `DS` writer - Duplex Status"]
pub type DsW<'a, REG> = crate::BitWriter<'a, REG, Ds>;
impl<'a, REG> DsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half-duplex status"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::HalfDuplex)
    }
    #[doc = "Full-duplex status"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(Ds::FullDuplex)
    }
}
#[doc = "Field `FC` reader - False Carrier is detected"]
pub type FcR = crate::BitReader;
#[doc = "Field `FC` writer - False Carrier is detected"]
pub type FcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CX` reader - Carrier extended"]
pub type CxR = crate::BitReader;
#[doc = "Field `CX` writer - Carrier extended"]
pub type CxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE` reader - Carrier extended frames error"]
pub type CeR = crate::BitReader;
#[doc = "Field `CE` writer - Carrier extended frames error"]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Carrier sence"]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Carrier sence"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - MAC Speed"]
pub type SpeedR = crate::BitReader;
#[doc = "Field `SPEED` writer - MAC Speed"]
pub type SpeedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GB` reader - Gigabit"]
pub type GbR = crate::BitReader;
#[doc = "Field `GB` writer - Gigabit"]
pub type GbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GA` reader - Gigabit MAC available"]
pub type GaR = crate::BitReader;
#[doc = "Field `GA` writer - Gigabit MAC available"]
pub type GaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - RAM debug enable"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - RAM debug enable"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NM` reader - No Clock Mux mode"]
pub type NmR = crate::BitReader;
#[doc = "Field `NM` writer - No Clock Mux mode"]
pub type NmW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - Link Status"]
    #[inline(always)]
    pub fn ls(&mut self) -> LsW<'_, RgmiiImSpec> {
        LsW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Line Clock Speed"]
    #[inline(always)]
    pub fn lcs(&mut self) -> LcsW<'_, RgmiiImSpec> {
        LcsW::new(self, 1)
    }
    #[doc = "Bit 3 - Duplex Status"]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<'_, RgmiiImSpec> {
        DsW::new(self, 3)
    }
    #[doc = "Bit 4 - False Carrier is detected"]
    #[inline(always)]
    pub fn fc(&mut self) -> FcW<'_, RgmiiImSpec> {
        FcW::new(self, 4)
    }
    #[doc = "Bit 5 - Carrier extended"]
    #[inline(always)]
    pub fn cx(&mut self) -> CxW<'_, RgmiiImSpec> {
        CxW::new(self, 5)
    }
    #[doc = "Bit 6 - Carrier extended frames error"]
    #[inline(always)]
    pub fn ce(&mut self) -> CeW<'_, RgmiiImSpec> {
        CeW::new(self, 6)
    }
    #[doc = "Bit 7 - Carrier sence"]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<'_, RgmiiImSpec> {
        CsW::new(self, 7)
    }
    #[doc = "Bit 8 - MAC Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, RgmiiImSpec> {
        SpeedW::new(self, 8)
    }
    #[doc = "Bit 9 - Gigabit"]
    #[inline(always)]
    pub fn gb(&mut self) -> GbW<'_, RgmiiImSpec> {
        GbW::new(self, 9)
    }
    #[doc = "Bit 13 - Gigabit MAC available"]
    #[inline(always)]
    pub fn ga(&mut self) -> GaW<'_, RgmiiImSpec> {
        GaW::new(self, 13)
    }
    #[doc = "Bit 14 - RAM debug enable"]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<'_, RgmiiImSpec> {
        RdW::new(self, 14)
    }
    #[doc = "Bit 15 - No Clock Mux mode"]
    #[inline(always)]
    pub fn nm(&mut self) -> NmW<'_, RgmiiImSpec> {
        NmW::new(self, 15)
    }
}
#[doc = "RGMII interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgmii_im::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgmii_im::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgmiiImSpec;
impl crate::RegisterSpec for RgmiiImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgmii_im::R`](R) reader structure"]
impl crate::Readable for RgmiiImSpec {}
#[doc = "`write(|w| ..)` method takes [`rgmii_im::W`](W) writer structure"]
impl crate::Writable for RgmiiImSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RGMII_IM to value 0"]
impl crate::Resettable for RgmiiImSpec {}
