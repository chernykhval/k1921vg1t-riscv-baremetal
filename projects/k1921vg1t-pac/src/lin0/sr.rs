#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Current state of the device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frs {
    #[doc = "1: IDLE state (only in transmit mode)"]
    Idle = 1,
    #[doc = "2: BREAK state"]
    Break = 2,
    #[doc = "3: SYNC state"]
    Sync = 3,
    #[doc = "4: PID state"]
    Pid = 4,
    #[doc = "5: DATA state"]
    Data = 5,
    #[doc = "6: CSUM state"]
    Csum = 6,
    #[doc = "7: send a signal WAKE state"]
    SendWake = 7,
}
impl From<Frs> for u8 {
    #[inline(always)]
    fn from(variant: Frs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frs {
    type Ux = u8;
}
impl crate::IsEnum for Frs {}
#[doc = "Field `FRS` reader - Current state of the device"]
pub type FrsR = crate::FieldReader<Frs>;
impl FrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frs> {
        match self.bits {
            1 => Some(Frs::Idle),
            2 => Some(Frs::Break),
            3 => Some(Frs::Sync),
            4 => Some(Frs::Pid),
            5 => Some(Frs::Data),
            6 => Some(Frs::Csum),
            7 => Some(Frs::SendWake),
            _ => None,
        }
    }
    #[doc = "IDLE state (only in transmit mode)"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Frs::Idle
    }
    #[doc = "BREAK state"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == Frs::Break
    }
    #[doc = "SYNC state"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Frs::Sync
    }
    #[doc = "PID state"]
    #[inline(always)]
    pub fn is_pid(&self) -> bool {
        *self == Frs::Pid
    }
    #[doc = "DATA state"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Frs::Data
    }
    #[doc = "CSUM state"]
    #[inline(always)]
    pub fn is_csum(&self) -> bool {
        *self == Frs::Csum
    }
    #[doc = "send a signal WAKE state"]
    #[inline(always)]
    pub fn is_send_wake(&self) -> bool {
        *self == Frs::SendWake
    }
}
#[doc = "Field `RV` reader - The activity bit of the receiving part of the interface"]
pub type RvR = crate::BitReader;
#[doc = "Field `TR` reader - The activity bit of the transmitting part of the interface"]
pub type TrR = crate::BitReader;
#[doc = "Field `WFSLP` reader - During sleep, a wake-up signal is received"]
pub type WfslpR = crate::BitReader;
#[doc = "Field `WFSLP` writer - During sleep, a wake-up signal is received"]
pub type WfslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTSLP` reader - Received message frame is a sleep signal"]
pub type GtslpR = crate::BitReader;
#[doc = "Field `GTSLP` writer - Received message frame is a sleep signal"]
pub type GtslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTXFR` reader - End of transmission bit"]
pub type EndtxfrR = crate::BitReader;
#[doc = "Field `ENDTXFR` writer - End of transmission bit"]
pub type EndtxfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIDRV` reader - PID received bit"]
pub type PidrvR = crate::BitReader;
#[doc = "Field `PIDRV` writer - PID received bit"]
pub type PidrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATARV` reader - Data received bit"]
pub type DatarvR = crate::BitReader;
#[doc = "Field `DATARV` writer - Data received bit"]
pub type DatarvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSUMRV` reader - Checksum received bit"]
pub type CsumrvR = crate::BitReader;
#[doc = "Field `CSUMRV` writer - Checksum received bit"]
pub type CsumrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLCONF` reader - Line conflict bit"]
pub type ErlconfR = crate::BitReader;
#[doc = "Field `ERLCONF` writer - Line conflict bit"]
pub type ErlconfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVCSUM` reader - Checksum error bit"]
pub type InvcsumR = crate::BitReader;
#[doc = "Field `INVCSUM` writer - Checksum error bit"]
pub type InvcsumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVPID` reader - PID error bit"]
pub type InvpidR = crate::BitReader;
#[doc = "Field `INVPID` writer - PID error bit"]
pub type InvpidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBITM` reader - No stop symbol bit"]
pub type StbitmR = crate::BitReader;
#[doc = "Field `STBITM` writer - No stop symbol bit"]
pub type StbitmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Current state of the device"]
    #[inline(always)]
    pub fn frs(&self) -> FrsR {
        FrsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - The activity bit of the receiving part of the interface"]
    #[inline(always)]
    pub fn rv(&self) -> RvR {
        RvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The activity bit of the transmitting part of the interface"]
    #[inline(always)]
    pub fn tr(&self) -> TrR {
        TrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - During sleep, a wake-up signal is received"]
    #[inline(always)]
    pub fn wfslp(&self) -> WfslpR {
        WfslpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received message frame is a sleep signal"]
    #[inline(always)]
    pub fn gtslp(&self) -> GtslpR {
        GtslpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of transmission bit"]
    #[inline(always)]
    pub fn endtxfr(&self) -> EndtxfrR {
        EndtxfrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PID received bit"]
    #[inline(always)]
    pub fn pidrv(&self) -> PidrvR {
        PidrvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data received bit"]
    #[inline(always)]
    pub fn datarv(&self) -> DatarvR {
        DatarvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Checksum received bit"]
    #[inline(always)]
    pub fn csumrv(&self) -> CsumrvR {
        CsumrvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Line conflict bit"]
    #[inline(always)]
    pub fn erlconf(&self) -> ErlconfR {
        ErlconfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Checksum error bit"]
    #[inline(always)]
    pub fn invcsum(&self) -> InvcsumR {
        InvcsumR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PID error bit"]
    #[inline(always)]
    pub fn invpid(&self) -> InvpidR {
        InvpidR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - No stop symbol bit"]
    #[inline(always)]
    pub fn stbitm(&self) -> StbitmR {
        StbitmR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - During sleep, a wake-up signal is received"]
    #[inline(always)]
    pub fn wfslp(&mut self) -> WfslpW<'_, SrSpec> {
        WfslpW::new(self, 6)
    }
    #[doc = "Bit 7 - Received message frame is a sleep signal"]
    #[inline(always)]
    pub fn gtslp(&mut self) -> GtslpW<'_, SrSpec> {
        GtslpW::new(self, 7)
    }
    #[doc = "Bit 8 - End of transmission bit"]
    #[inline(always)]
    pub fn endtxfr(&mut self) -> EndtxfrW<'_, SrSpec> {
        EndtxfrW::new(self, 8)
    }
    #[doc = "Bit 9 - PID received bit"]
    #[inline(always)]
    pub fn pidrv(&mut self) -> PidrvW<'_, SrSpec> {
        PidrvW::new(self, 9)
    }
    #[doc = "Bit 10 - Data received bit"]
    #[inline(always)]
    pub fn datarv(&mut self) -> DatarvW<'_, SrSpec> {
        DatarvW::new(self, 10)
    }
    #[doc = "Bit 11 - Checksum received bit"]
    #[inline(always)]
    pub fn csumrv(&mut self) -> CsumrvW<'_, SrSpec> {
        CsumrvW::new(self, 11)
    }
    #[doc = "Bit 12 - Line conflict bit"]
    #[inline(always)]
    pub fn erlconf(&mut self) -> ErlconfW<'_, SrSpec> {
        ErlconfW::new(self, 12)
    }
    #[doc = "Bit 13 - Checksum error bit"]
    #[inline(always)]
    pub fn invcsum(&mut self) -> InvcsumW<'_, SrSpec> {
        InvcsumW::new(self, 13)
    }
    #[doc = "Bit 14 - PID error bit"]
    #[inline(always)]
    pub fn invpid(&mut self) -> InvpidW<'_, SrSpec> {
        InvpidW::new(self, 14)
    }
    #[doc = "Bit 15 - No stop symbol bit"]
    #[inline(always)]
    pub fn stbitm(&mut self) -> StbitmW<'_, SrSpec> {
        StbitmW::new(self, 15)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
