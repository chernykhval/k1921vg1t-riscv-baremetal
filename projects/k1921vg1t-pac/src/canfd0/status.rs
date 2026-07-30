#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `ERRP` reader - Passive mode error"]
pub type ErrpR = crate::BitReader;
#[doc = "Field `BOFF` reader - CAN Bus Disable Flag"]
pub type BoffR = crate::BitReader;
#[doc = "Field `ENFG` reader - Enable Flag"]
pub type EnfgR = crate::BitReader;
#[doc = "Field `RXBSY` reader - Receiver Busy Flag"]
pub type RxbsyR = crate::BitReader;
#[doc = "Field `TXBSY` reader - Transmitter Busy Flag"]
pub type TxbsyR = crate::BitReader;
#[doc = "Field `OVRG` reader - CAN Timer Overrun Interrupt"]
pub type OvrgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Passive mode error"]
    #[inline(always)]
    pub fn errp(&self) -> ErrpR {
        ErrpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Bus Disable Flag"]
    #[inline(always)]
    pub fn boff(&self) -> BoffR {
        BoffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Flag"]
    #[inline(always)]
    pub fn enfg(&self) -> EnfgR {
        EnfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Busy Flag"]
    #[inline(always)]
    pub fn rxbsy(&self) -> RxbsyR {
        RxbsyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter Busy Flag"]
    #[inline(always)]
    pub fn txbsy(&self) -> TxbsyR {
        TxbsyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CAN Timer Overrun Interrupt"]
    #[inline(always)]
    pub fn ovrg(&self) -> OvrgR {
        OvrgR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "CAN General Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
