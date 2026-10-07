#[doc = "Register `I2SCTRL` reader"]
pub type R = crate::R<I2sctrlSpec>;
#[doc = "Register `I2SCTRL` writer"]
pub type W = crate::W<I2sctrlSpec>;
#[doc = "Field `EN` reader - Enable I2S block"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable I2S block"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EN` reader - I2S Transmitter enable"]
pub type TxEnR = crate::BitReader;
#[doc = "Field `TX_EN` writer - I2S Transmitter enable"]
pub type TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EN` reader - I2S Receiver enable"]
pub type RxEnR = crate::BitReader;
#[doc = "Field `RX_EN` writer - I2S Receiver enable"]
pub type RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "I2S transmitter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxMode {
    #[doc = "0: I2S Philips protocol mode"]
    Phillips = 0,
    #[doc = "1: MSB-justified protocol mode"]
    Msbjustified = 1,
    #[doc = "2: LSB-justified protocol mode"]
    Lsbjustified = 2,
    #[doc = "3: PCM Short Frame protocol mode"]
    PcmshortFrame = 3,
    #[doc = "7: PCM Long Frame protocol mode"]
    PcmlongFrame = 7,
}
impl From<TxMode> for u8 {
    #[inline(always)]
    fn from(variant: TxMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxMode {
    type Ux = u8;
}
impl crate::IsEnum for TxMode {}
#[doc = "Field `TX_MODE` reader - I2S transmitter mode"]
pub type TxModeR = crate::FieldReader<TxMode>;
impl TxModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxMode> {
        match self.bits {
            0 => Some(TxMode::Phillips),
            1 => Some(TxMode::Msbjustified),
            2 => Some(TxMode::Lsbjustified),
            3 => Some(TxMode::PcmshortFrame),
            7 => Some(TxMode::PcmlongFrame),
            _ => None,
        }
    }
    #[doc = "I2S Philips protocol mode"]
    #[inline(always)]
    pub fn is_phillips(&self) -> bool {
        *self == TxMode::Phillips
    }
    #[doc = "MSB-justified protocol mode"]
    #[inline(always)]
    pub fn is_msbjustified(&self) -> bool {
        *self == TxMode::Msbjustified
    }
    #[doc = "LSB-justified protocol mode"]
    #[inline(always)]
    pub fn is_lsbjustified(&self) -> bool {
        *self == TxMode::Lsbjustified
    }
    #[doc = "PCM Short Frame protocol mode"]
    #[inline(always)]
    pub fn is_pcmshort_frame(&self) -> bool {
        *self == TxMode::PcmshortFrame
    }
    #[doc = "PCM Long Frame protocol mode"]
    #[inline(always)]
    pub fn is_pcmlong_frame(&self) -> bool {
        *self == TxMode::PcmlongFrame
    }
}
#[doc = "Field `TX_MODE` writer - I2S transmitter mode"]
pub type TxModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, TxMode>;
impl<'a, REG> TxModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S Philips protocol mode"]
    #[inline(always)]
    pub fn phillips(self) -> &'a mut crate::W<REG> {
        self.variant(TxMode::Phillips)
    }
    #[doc = "MSB-justified protocol mode"]
    #[inline(always)]
    pub fn msbjustified(self) -> &'a mut crate::W<REG> {
        self.variant(TxMode::Msbjustified)
    }
    #[doc = "LSB-justified protocol mode"]
    #[inline(always)]
    pub fn lsbjustified(self) -> &'a mut crate::W<REG> {
        self.variant(TxMode::Lsbjustified)
    }
    #[doc = "PCM Short Frame protocol mode"]
    #[inline(always)]
    pub fn pcmshort_frame(self) -> &'a mut crate::W<REG> {
        self.variant(TxMode::PcmshortFrame)
    }
    #[doc = "PCM Long Frame protocol mode"]
    #[inline(always)]
    pub fn pcmlong_frame(self) -> &'a mut crate::W<REG> {
        self.variant(TxMode::PcmlongFrame)
    }
}
#[doc = "I2S receiver mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxMode {
    #[doc = "0: I2S Philips protocol mode"]
    Phillips = 0,
    #[doc = "1: MSB-justified protocol mode"]
    Msbjustified = 1,
    #[doc = "2: LSB-justified protocol mode"]
    Lsbjustified = 2,
    #[doc = "3: PCM Short Frame protocol mode"]
    PcmshortFrame = 3,
    #[doc = "7: PCM Long Frame protocol mode"]
    PcmlongFrame = 7,
}
impl From<RxMode> for u8 {
    #[inline(always)]
    fn from(variant: RxMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxMode {
    type Ux = u8;
}
impl crate::IsEnum for RxMode {}
#[doc = "Field `RX_MODE` reader - I2S receiver mode"]
pub type RxModeR = crate::FieldReader<RxMode>;
impl RxModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxMode> {
        match self.bits {
            0 => Some(RxMode::Phillips),
            1 => Some(RxMode::Msbjustified),
            2 => Some(RxMode::Lsbjustified),
            3 => Some(RxMode::PcmshortFrame),
            7 => Some(RxMode::PcmlongFrame),
            _ => None,
        }
    }
    #[doc = "I2S Philips protocol mode"]
    #[inline(always)]
    pub fn is_phillips(&self) -> bool {
        *self == RxMode::Phillips
    }
    #[doc = "MSB-justified protocol mode"]
    #[inline(always)]
    pub fn is_msbjustified(&self) -> bool {
        *self == RxMode::Msbjustified
    }
    #[doc = "LSB-justified protocol mode"]
    #[inline(always)]
    pub fn is_lsbjustified(&self) -> bool {
        *self == RxMode::Lsbjustified
    }
    #[doc = "PCM Short Frame protocol mode"]
    #[inline(always)]
    pub fn is_pcmshort_frame(&self) -> bool {
        *self == RxMode::PcmshortFrame
    }
    #[doc = "PCM Long Frame protocol mode"]
    #[inline(always)]
    pub fn is_pcmlong_frame(&self) -> bool {
        *self == RxMode::PcmlongFrame
    }
}
#[doc = "Field `RX_MODE` writer - I2S receiver mode"]
pub type RxModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, RxMode>;
impl<'a, REG> RxModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S Philips protocol mode"]
    #[inline(always)]
    pub fn phillips(self) -> &'a mut crate::W<REG> {
        self.variant(RxMode::Phillips)
    }
    #[doc = "MSB-justified protocol mode"]
    #[inline(always)]
    pub fn msbjustified(self) -> &'a mut crate::W<REG> {
        self.variant(RxMode::Msbjustified)
    }
    #[doc = "LSB-justified protocol mode"]
    #[inline(always)]
    pub fn lsbjustified(self) -> &'a mut crate::W<REG> {
        self.variant(RxMode::Lsbjustified)
    }
    #[doc = "PCM Short Frame protocol mode"]
    #[inline(always)]
    pub fn pcmshort_frame(self) -> &'a mut crate::W<REG> {
        self.variant(RxMode::PcmshortFrame)
    }
    #[doc = "PCM Long Frame protocol mode"]
    #[inline(always)]
    pub fn pcmlong_frame(self) -> &'a mut crate::W<REG> {
        self.variant(RxMode::PcmlongFrame)
    }
}
#[doc = "Field `TXEMPTYINTEN` reader - Transmit buffer empty interrupt enable"]
pub type TxemptyintenR = crate::BitReader;
#[doc = "Field `TXEMPTYINTEN` writer - Transmit buffer empty interrupt enable"]
pub type TxemptyintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULLINTEN` reader - Receiver buffer full interrupt enable"]
pub type RxfullintenR = crate::BitReader;
#[doc = "Field `RXFULLINTEN` writer - Receiver buffer full interrupt enable"]
pub type RxfullintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CLK_SEL` reader - Transmitter clock source select"]
pub type TxClkSelR = crate::FieldReader;
#[doc = "Field `TX_CLK_SEL` writer - Transmitter clock source select"]
pub type TxClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_CLK_SEL` reader - Receiver clock source select"]
pub type RxClkSelR = crate::FieldReader;
#[doc = "Field `RX_CLK_SEL` writer - Receiver clock source select"]
pub type RxClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable I2S block"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2S Transmitter enable"]
    #[inline(always)]
    pub fn tx_en(&self) -> TxEnR {
        TxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2S Receiver enable"]
    #[inline(always)]
    pub fn rx_en(&self) -> RxEnR {
        RxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - I2S transmitter mode"]
    #[inline(always)]
    pub fn tx_mode(&self) -> TxModeR {
        TxModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - I2S receiver mode"]
    #[inline(always)]
    pub fn rx_mode(&self) -> RxModeR {
        RxModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txemptyinten(&self) -> TxemptyintenR {
        TxemptyintenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receiver buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxfullinten(&self) -> RxfullintenR {
        RxfullintenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Transmitter clock source select"]
    #[inline(always)]
    pub fn tx_clk_sel(&self) -> TxClkSelR {
        TxClkSelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receiver clock source select"]
    #[inline(always)]
    pub fn rx_clk_sel(&self) -> RxClkSelR {
        RxClkSelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable I2S block"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, I2sctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - I2S Transmitter enable"]
    #[inline(always)]
    pub fn tx_en(&mut self) -> TxEnW<'_, I2sctrlSpec> {
        TxEnW::new(self, 1)
    }
    #[doc = "Bit 2 - I2S Receiver enable"]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RxEnW<'_, I2sctrlSpec> {
        RxEnW::new(self, 2)
    }
    #[doc = "Bits 4:6 - I2S transmitter mode"]
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TxModeW<'_, I2sctrlSpec> {
        TxModeW::new(self, 4)
    }
    #[doc = "Bits 8:10 - I2S receiver mode"]
    #[inline(always)]
    pub fn rx_mode(&mut self) -> RxModeW<'_, I2sctrlSpec> {
        RxModeW::new(self, 8)
    }
    #[doc = "Bit 12 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txemptyinten(&mut self) -> TxemptyintenW<'_, I2sctrlSpec> {
        TxemptyintenW::new(self, 12)
    }
    #[doc = "Bit 13 - Receiver buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxfullinten(&mut self) -> RxfullintenW<'_, I2sctrlSpec> {
        RxfullintenW::new(self, 13)
    }
    #[doc = "Bits 16:23 - Transmitter clock source select"]
    #[inline(always)]
    pub fn tx_clk_sel(&mut self) -> TxClkSelW<'_, I2sctrlSpec> {
        TxClkSelW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Receiver clock source select"]
    #[inline(always)]
    pub fn rx_clk_sel(&mut self) -> RxClkSelW<'_, I2sctrlSpec> {
        RxClkSelW::new(self, 24)
    }
}
#[doc = "I2S Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sctrlSpec;
impl crate::RegisterSpec for I2sctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sctrl::R`](R) reader structure"]
impl crate::Readable for I2sctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sctrl::W`](W) writer structure"]
impl crate::Writable for I2sctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SCTRL to value 0"]
impl crate::Resettable for I2sctrlSpec {}
