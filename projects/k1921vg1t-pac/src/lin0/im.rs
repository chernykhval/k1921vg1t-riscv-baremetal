#[doc = "Register `IM` reader"]
pub type R = crate::R<ImSpec>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<ImSpec>;
#[doc = "Field `FSEN` reader - Enable interrupt when message frame field set"]
pub type FsenR = crate::FieldReader;
#[doc = "Field `FSEN` writer - Enable interrupt when message frame field set"]
pub type FsenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RVEN` reader - Enable interrupt by activating the receiving part of the device"]
pub type RvenR = crate::BitReader;
#[doc = "Field `RVEN` writer - Enable interrupt by activating the receiving part of the device"]
pub type RvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TREN` reader - Enable interrupt by activating the transmitting part of the device"]
pub type TrenR = crate::BitReader;
#[doc = "Field `TREN` writer - Enable interrupt by activating the transmitting part of the device"]
pub type TrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFSLPEN` reader - Enable interrupt when receiving a wake-up signal"]
pub type WfslpenR = crate::BitReader;
#[doc = "Field `WFSLPEN` writer - Enable interrupt when receiving a wake-up signal"]
pub type WfslpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTSLPEN` reader - Enable interrupt when receiving a sleep signal"]
pub type GtslpenR = crate::BitReader;
#[doc = "Field `GTSLPEN` writer - Enable interrupt when receiving a sleep signal"]
pub type GtslpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTXFREN` reader - Enable interrupt when the transmission of the next message frame is completed"]
pub type EndtxfrenR = crate::BitReader;
#[doc = "Field `ENDTXFREN` writer - Enable interrupt when the transmission of the next message frame is completed"]
pub type EndtxfrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIDRVEN` reader - Enable interrupt by receiving the PID field"]
pub type PidrvenR = crate::BitReader;
#[doc = "Field `PIDRVEN` writer - Enable interrupt by receiving the PID field"]
pub type PidrvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATARVEN` reader - Enable interrupt by receiving a data field"]
pub type DatarvenR = crate::BitReader;
#[doc = "Field `DATARVEN` writer - Enable interrupt by receiving a data field"]
pub type DatarvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSUMRVEN` reader - Enable interrupt by receiving of the checksum field"]
pub type CsumrvenR = crate::BitReader;
#[doc = "Field `CSUMRVEN` writer - Enable interrupt by receiving of the checksum field"]
pub type CsumrvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLCONFEN` reader - Enable interrupt when a line conflict occurs"]
pub type ErlconfenR = crate::BitReader;
#[doc = "Field `ERLCONFEN` writer - Enable interrupt when a line conflict occurs"]
pub type ErlconfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVCSUMEN` reader - Enable interrupt to receive an incorrect checksum"]
pub type InvcsumenR = crate::BitReader;
#[doc = "Field `INVCSUMEN` writer - Enable interrupt to receive an incorrect checksum"]
pub type InvcsumenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVPIDEN` reader - Enable interrupt to receive a PID with broken parity"]
pub type InvpidenR = crate::BitReader;
#[doc = "Field `INVPIDEN` writer - Enable interrupt to receive a PID with broken parity"]
pub type InvpidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBITMEN` reader - Enable interrupt by the failure of a stop bit"]
pub type StbitmenR = crate::BitReader;
#[doc = "Field `STBITMEN` writer - Enable interrupt by the failure of a stop bit"]
pub type StbitmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Enable interrupt when message frame field set"]
    #[inline(always)]
    pub fn fsen(&self) -> FsenR {
        FsenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable interrupt by activating the receiving part of the device"]
    #[inline(always)]
    pub fn rven(&self) -> RvenR {
        RvenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable interrupt by activating the transmitting part of the device"]
    #[inline(always)]
    pub fn tren(&self) -> TrenR {
        TrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable interrupt when receiving a wake-up signal"]
    #[inline(always)]
    pub fn wfslpen(&self) -> WfslpenR {
        WfslpenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable interrupt when receiving a sleep signal"]
    #[inline(always)]
    pub fn gtslpen(&self) -> GtslpenR {
        GtslpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable interrupt when the transmission of the next message frame is completed"]
    #[inline(always)]
    pub fn endtxfren(&self) -> EndtxfrenR {
        EndtxfrenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable interrupt by receiving the PID field"]
    #[inline(always)]
    pub fn pidrven(&self) -> PidrvenR {
        PidrvenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable interrupt by receiving a data field"]
    #[inline(always)]
    pub fn datarven(&self) -> DatarvenR {
        DatarvenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable interrupt by receiving of the checksum field"]
    #[inline(always)]
    pub fn csumrven(&self) -> CsumrvenR {
        CsumrvenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable interrupt when a line conflict occurs"]
    #[inline(always)]
    pub fn erlconfen(&self) -> ErlconfenR {
        ErlconfenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable interrupt to receive an incorrect checksum"]
    #[inline(always)]
    pub fn invcsumen(&self) -> InvcsumenR {
        InvcsumenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable interrupt to receive a PID with broken parity"]
    #[inline(always)]
    pub fn invpiden(&self) -> InvpidenR {
        InvpidenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable interrupt by the failure of a stop bit"]
    #[inline(always)]
    pub fn stbitmen(&self) -> StbitmenR {
        StbitmenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Enable interrupt when message frame field set"]
    #[inline(always)]
    pub fn fsen(&mut self) -> FsenW<'_, ImSpec> {
        FsenW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable interrupt by activating the receiving part of the device"]
    #[inline(always)]
    pub fn rven(&mut self) -> RvenW<'_, ImSpec> {
        RvenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable interrupt by activating the transmitting part of the device"]
    #[inline(always)]
    pub fn tren(&mut self) -> TrenW<'_, ImSpec> {
        TrenW::new(self, 4)
    }
    #[doc = "Bit 6 - Enable interrupt when receiving a wake-up signal"]
    #[inline(always)]
    pub fn wfslpen(&mut self) -> WfslpenW<'_, ImSpec> {
        WfslpenW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable interrupt when receiving a sleep signal"]
    #[inline(always)]
    pub fn gtslpen(&mut self) -> GtslpenW<'_, ImSpec> {
        GtslpenW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable interrupt when the transmission of the next message frame is completed"]
    #[inline(always)]
    pub fn endtxfren(&mut self) -> EndtxfrenW<'_, ImSpec> {
        EndtxfrenW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable interrupt by receiving the PID field"]
    #[inline(always)]
    pub fn pidrven(&mut self) -> PidrvenW<'_, ImSpec> {
        PidrvenW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable interrupt by receiving a data field"]
    #[inline(always)]
    pub fn datarven(&mut self) -> DatarvenW<'_, ImSpec> {
        DatarvenW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable interrupt by receiving of the checksum field"]
    #[inline(always)]
    pub fn csumrven(&mut self) -> CsumrvenW<'_, ImSpec> {
        CsumrvenW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable interrupt when a line conflict occurs"]
    #[inline(always)]
    pub fn erlconfen(&mut self) -> ErlconfenW<'_, ImSpec> {
        ErlconfenW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable interrupt to receive an incorrect checksum"]
    #[inline(always)]
    pub fn invcsumen(&mut self) -> InvcsumenW<'_, ImSpec> {
        InvcsumenW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable interrupt to receive a PID with broken parity"]
    #[inline(always)]
    pub fn invpiden(&mut self) -> InvpidenW<'_, ImSpec> {
        InvpidenW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable interrupt by the failure of a stop bit"]
    #[inline(always)]
    pub fn stbitmen(&mut self) -> StbitmenW<'_, ImSpec> {
        StbitmenW::new(self, 15)
    }
}
#[doc = "Mask interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`im::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImSpec;
impl crate::RegisterSpec for ImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for ImSpec {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for ImSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for ImSpec {}
