#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Message frame type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mestp {
    #[doc = "1: IDLE state (only in transmit mode)"]
    Idle = 1,
    #[doc = "2: HEADER state"]
    Header = 2,
    #[doc = "3: HEADER+RESPONSE state"]
    HeaderResponse = 3,
    #[doc = "4: RESPONSE RESEIVE state"]
    ResponseReseive = 4,
    #[doc = "5: RESPONSE TRANSMIT state"]
    ResponseTransmit = 5,
    #[doc = "7: send a signal WAKE"]
    SendWake = 7,
}
impl From<Mestp> for u8 {
    #[inline(always)]
    fn from(variant: Mestp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mestp {
    type Ux = u8;
}
impl crate::IsEnum for Mestp {}
#[doc = "Field `MESTP` reader - Message frame type"]
pub type MestpR = crate::FieldReader<Mestp>;
impl MestpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mestp> {
        match self.bits {
            1 => Some(Mestp::Idle),
            2 => Some(Mestp::Header),
            3 => Some(Mestp::HeaderResponse),
            4 => Some(Mestp::ResponseReseive),
            5 => Some(Mestp::ResponseTransmit),
            7 => Some(Mestp::SendWake),
            _ => None,
        }
    }
    #[doc = "IDLE state (only in transmit mode)"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Mestp::Idle
    }
    #[doc = "HEADER state"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == Mestp::Header
    }
    #[doc = "HEADER+RESPONSE state"]
    #[inline(always)]
    pub fn is_header_response(&self) -> bool {
        *self == Mestp::HeaderResponse
    }
    #[doc = "RESPONSE RESEIVE state"]
    #[inline(always)]
    pub fn is_response_reseive(&self) -> bool {
        *self == Mestp::ResponseReseive
    }
    #[doc = "RESPONSE TRANSMIT state"]
    #[inline(always)]
    pub fn is_response_transmit(&self) -> bool {
        *self == Mestp::ResponseTransmit
    }
    #[doc = "send a signal WAKE"]
    #[inline(always)]
    pub fn is_send_wake(&self) -> bool {
        *self == Mestp::SendWake
    }
}
#[doc = "Field `MESTP` writer - Message frame type"]
pub type MestpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mestp>;
impl<'a, REG> MestpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IDLE state (only in transmit mode)"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Mestp::Idle)
    }
    #[doc = "HEADER state"]
    #[inline(always)]
    pub fn header(self) -> &'a mut crate::W<REG> {
        self.variant(Mestp::Header)
    }
    #[doc = "HEADER+RESPONSE state"]
    #[inline(always)]
    pub fn header_response(self) -> &'a mut crate::W<REG> {
        self.variant(Mestp::HeaderResponse)
    }
    #[doc = "RESPONSE RESEIVE state"]
    #[inline(always)]
    pub fn response_reseive(self) -> &'a mut crate::W<REG> {
        self.variant(Mestp::ResponseReseive)
    }
    #[doc = "RESPONSE TRANSMIT state"]
    #[inline(always)]
    pub fn response_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(Mestp::ResponseTransmit)
    }
    #[doc = "send a signal WAKE"]
    #[inline(always)]
    pub fn send_wake(self) -> &'a mut crate::W<REG> {
        self.variant(Mestp::SendWake)
    }
}
#[doc = "Field `UDL` reader - Number of data fields in one message frame"]
pub type UdlR = crate::FieldReader;
#[doc = "Field `UDL` writer - Number of data fields in one message frame"]
pub type UdlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MS` reader - Device type bit master/slave"]
pub type MsR = crate::BitReader;
#[doc = "Field `MS` writer - Device type bit master/slave"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTVR` reader - Protocol type bit"]
pub type ProtvrR = crate::BitReader;
#[doc = "Field `PROTVR` writer - Protocol type bit"]
pub type ProtvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSUMTP` reader - Checksum type bit"]
pub type CsumtpR = crate::BitReader;
#[doc = "Field `CSUMTP` writer - Checksum type bit"]
pub type CsumtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP` reader - Sleep state bit"]
pub type SlpR = crate::BitReader;
#[doc = "Field `SLP` writer - Sleep state bit"]
pub type SlpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Message frame type"]
    #[inline(always)]
    pub fn mestp(&self) -> MestpR {
        MestpR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Number of data fields in one message frame"]
    #[inline(always)]
    pub fn udl(&self) -> UdlR {
        UdlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Device type bit master/slave"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protocol type bit"]
    #[inline(always)]
    pub fn protvr(&self) -> ProtvrR {
        ProtvrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Checksum type bit"]
    #[inline(always)]
    pub fn csumtp(&self) -> CsumtpR {
        CsumtpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sleep state bit"]
    #[inline(always)]
    pub fn slp(&self) -> SlpR {
        SlpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Message frame type"]
    #[inline(always)]
    pub fn mestp(&mut self) -> MestpW<'_, CrSpec> {
        MestpW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Number of data fields in one message frame"]
    #[inline(always)]
    pub fn udl(&mut self) -> UdlW<'_, CrSpec> {
        UdlW::new(self, 8)
    }
    #[doc = "Bit 12 - Device type bit master/slave"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, CrSpec> {
        MsW::new(self, 12)
    }
    #[doc = "Bit 13 - Protocol type bit"]
    #[inline(always)]
    pub fn protvr(&mut self) -> ProtvrW<'_, CrSpec> {
        ProtvrW::new(self, 13)
    }
    #[doc = "Bit 14 - Checksum type bit"]
    #[inline(always)]
    pub fn csumtp(&mut self) -> CsumtpW<'_, CrSpec> {
        CsumtpW::new(self, 14)
    }
    #[doc = "Bit 15 - Sleep state bit"]
    #[inline(always)]
    pub fn slp(&mut self) -> SlpW<'_, CrSpec> {
        SlpW::new(self, 15)
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
#[doc = "`reset()` method sets CR to value 0x0801"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x0801;
}
