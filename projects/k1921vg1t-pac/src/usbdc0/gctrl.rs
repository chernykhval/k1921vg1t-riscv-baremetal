#[doc = "Register `GCTRL` reader"]
pub type R = crate::R<GctrlSpec>;
#[doc = "Register `GCTRL` writer"]
pub type W = crate::W<GctrlSpec>;
#[doc = "Field `SU` writer - Set USB address"]
pub type SuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UA` reader - USB address"]
pub type UaR = crate::FieldReader;
#[doc = "Field `UA` writer - USB address"]
pub type UaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TM` reader - Enable test mode"]
pub type TmR = crate::BitReader;
#[doc = "Field `TM` writer - Enable test mode"]
pub type TmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Testmode selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ts {
    #[doc = "0: Reserved"]
    Reserved0 = 0,
    #[doc = "1: Test Mode J"]
    TestJ = 1,
    #[doc = "2: Test Mode K"]
    TestK = 2,
    #[doc = "3: Test Mode SE0 Not ACK"]
    TestSe0Nak = 3,
    #[doc = "4: Test Mode packet"]
    TestPacket = 4,
    #[doc = "5: Reserved"]
    Reserved5 = 5,
    #[doc = "6: Reserved"]
    Reserved6 = 6,
    #[doc = "7: Reserved"]
    Reserved7 = 7,
}
impl From<Ts> for u8 {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ts {
    type Ux = u8;
}
impl crate::IsEnum for Ts {}
#[doc = "Field `TS` reader - Testmode selector"]
pub type TsR = crate::FieldReader<Ts>;
impl TsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts {
        match self.bits {
            0 => Ts::Reserved0,
            1 => Ts::TestJ,
            2 => Ts::TestK,
            3 => Ts::TestSe0Nak,
            4 => Ts::TestPacket,
            5 => Ts::Reserved5,
            6 => Ts::Reserved6,
            7 => Ts::Reserved7,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Ts::Reserved0
    }
    #[doc = "Test Mode J"]
    #[inline(always)]
    pub fn is_test_j(&self) -> bool {
        *self == Ts::TestJ
    }
    #[doc = "Test Mode K"]
    #[inline(always)]
    pub fn is_test_k(&self) -> bool {
        *self == Ts::TestK
    }
    #[doc = "Test Mode SE0 Not ACK"]
    #[inline(always)]
    pub fn is_test_se0_nak(&self) -> bool {
        *self == Ts::TestSe0Nak
    }
    #[doc = "Test Mode packet"]
    #[inline(always)]
    pub fn is_test_packet(&self) -> bool {
        *self == Ts::TestPacket
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Ts::Reserved5
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved6(&self) -> bool {
        *self == Ts::Reserved6
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserved7(&self) -> bool {
        *self == Ts::Reserved7
    }
}
#[doc = "Field `TS` writer - Testmode selector"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ts, crate::Safe>;
impl<'a, REG> TsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Reserved0)
    }
    #[doc = "Test Mode J"]
    #[inline(always)]
    pub fn test_j(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::TestJ)
    }
    #[doc = "Test Mode K"]
    #[inline(always)]
    pub fn test_k(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::TestK)
    }
    #[doc = "Test Mode SE0 Not ACK"]
    #[inline(always)]
    pub fn test_se0_nak(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::TestSe0Nak)
    }
    #[doc = "Test Mode packet"]
    #[inline(always)]
    pub fn test_packet(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::TestPacket)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Reserved5)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved6(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Reserved6)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserved7(self) -> &'a mut crate::W<REG> {
        self.variant(Ts::Reserved7)
    }
}
#[doc = "Field `RW` reader - Remote wakeup"]
pub type RwR = crate::BitReader;
#[doc = "Field `RW` writer - Remote wakeup"]
pub type RwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DH` reader - Disable High-speed"]
pub type DhR = crate::BitReader;
#[doc = "Field `DH` writer - Disable High-speed"]
pub type DhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP` reader - Enable Pull-Up"]
pub type EpR = crate::BitReader;
#[doc = "Field `EP` writer - Enable Pull-Up"]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT` reader - Functional test mode"]
pub type FtR = crate::BitReader;
#[doc = "Field `FT` writer - Functional test mode"]
pub type FtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FI` reader - Frame number received interrupt"]
pub type FiR = crate::BitReader;
#[doc = "Field `FI` writer - Frame number received interrupt"]
pub type FiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP` reader - Speed mode interrupt"]
pub type SpR = crate::BitReader;
#[doc = "Field `SP` writer - Speed mode interrupt"]
pub type SpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VI` reader - VBUS valid interrupt"]
pub type ViR = crate::BitReader;
#[doc = "Field `VI` writer - VBUS valid interrupt"]
pub type ViW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UI` reader - USB reset"]
pub type UiR = crate::BitReader;
#[doc = "Field `UI` writer - USB reset"]
pub type UiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SI` reader - Suspend interrupt"]
pub type SiR = crate::BitReader;
#[doc = "Field `SI` writer - Suspend interrupt"]
pub type SiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:7 - USB address"]
    #[inline(always)]
    pub fn ua(&self) -> UaR {
        UaR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Enable test mode"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Testmode selector"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Remote wakeup"]
    #[inline(always)]
    pub fn rw(&self) -> RwR {
        RwR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable High-speed"]
    #[inline(always)]
    pub fn dh(&self) -> DhR {
        DhR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Pull-Up"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Functional test mode"]
    #[inline(always)]
    pub fn ft(&self) -> FtR {
        FtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 27 - Frame number received interrupt"]
    #[inline(always)]
    pub fn fi(&self) -> FiR {
        FiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Speed mode interrupt"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - VBUS valid interrupt"]
    #[inline(always)]
    pub fn vi(&self) -> ViR {
        ViR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB reset"]
    #[inline(always)]
    pub fn ui(&self) -> UiR {
        UiR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Suspend interrupt"]
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set USB address"]
    #[inline(always)]
    pub fn su(&mut self) -> SuW<'_, GctrlSpec> {
        SuW::new(self, 0)
    }
    #[doc = "Bits 1:7 - USB address"]
    #[inline(always)]
    pub fn ua(&mut self) -> UaW<'_, GctrlSpec> {
        UaW::new(self, 1)
    }
    #[doc = "Bit 8 - Enable test mode"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<'_, GctrlSpec> {
        TmW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Testmode selector"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, GctrlSpec> {
        TsW::new(self, 9)
    }
    #[doc = "Bit 12 - Remote wakeup"]
    #[inline(always)]
    pub fn rw(&mut self) -> RwW<'_, GctrlSpec> {
        RwW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable High-speed"]
    #[inline(always)]
    pub fn dh(&mut self) -> DhW<'_, GctrlSpec> {
        DhW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Pull-Up"]
    #[inline(always)]
    pub fn ep(&mut self) -> EpW<'_, GctrlSpec> {
        EpW::new(self, 14)
    }
    #[doc = "Bit 15 - Functional test mode"]
    #[inline(always)]
    pub fn ft(&mut self) -> FtW<'_, GctrlSpec> {
        FtW::new(self, 15)
    }
    #[doc = "Bit 27 - Frame number received interrupt"]
    #[inline(always)]
    pub fn fi(&mut self) -> FiW<'_, GctrlSpec> {
        FiW::new(self, 27)
    }
    #[doc = "Bit 28 - Speed mode interrupt"]
    #[inline(always)]
    pub fn sp(&mut self) -> SpW<'_, GctrlSpec> {
        SpW::new(self, 28)
    }
    #[doc = "Bit 29 - VBUS valid interrupt"]
    #[inline(always)]
    pub fn vi(&mut self) -> ViW<'_, GctrlSpec> {
        ViW::new(self, 29)
    }
    #[doc = "Bit 30 - USB reset"]
    #[inline(always)]
    pub fn ui(&mut self) -> UiW<'_, GctrlSpec> {
        UiW::new(self, 30)
    }
    #[doc = "Bit 31 - Suspend interrupt"]
    #[inline(always)]
    pub fn si(&mut self) -> SiW<'_, GctrlSpec> {
        SiW::new(self, 31)
    }
}
#[doc = "USBDC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GctrlSpec;
impl crate::RegisterSpec for GctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gctrl::R`](R) reader structure"]
impl crate::Readable for GctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gctrl::W`](W) writer structure"]
impl crate::Writable for GctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCTRL to value 0"]
impl crate::Resettable for GctrlSpec {}
