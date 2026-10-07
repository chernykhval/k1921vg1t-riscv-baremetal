#[doc = "Register `MOBSTATUS` reader"]
pub type R = crate::R<MobstatusSpec>;
#[doc = "Register `MOBSTATUS` writer"]
pub type W = crate::W<MobstatusSpec>;
#[doc = "Field `AERR` reader - Acknowledge Error Interrupt"]
pub type AerrR = crate::BitReader;
#[doc = "Field `AERR` writer - Acknowledge Error Interrupt"]
pub type AerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Frame Error Interrupt"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - Frame Error Interrupt"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERR` reader - CRC Error Interrupt"]
pub type CerrR = crate::BitReader;
#[doc = "Field `CERR` writer - CRC Error Interrupt"]
pub type CerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERR` reader - Serial Error"]
pub type SerrR = crate::BitReader;
#[doc = "Field `SERR` writer - Serial Error"]
pub type SerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Bit Error while Transmit"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - Bit Error while Transmit"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOK` reader - Receive OK"]
pub type RxokR = crate::BitReader;
#[doc = "Field `RXOK` writer - Receive OK"]
pub type RxokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOK` reader - Transmit OK"]
pub type TxokR = crate::BitReader;
#[doc = "Field `TXOK` writer - Transmit OK"]
pub type TxokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLCW` reader - Data Length Code Warning"]
pub type DlcwR = crate::BitReader;
#[doc = "Field `DLCW` writer - Data Length Code Warning"]
pub type DlcwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Error State Indicator flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esi {
    #[doc = "0: A CANFD frame was transmitted by node in the Error Active state. Or a CAN2.0 frame was transmitted"]
    ErrorActive = 0,
    #[doc = "1: A CANFD frame was transmitted by node in the Error Passive state"]
    ErrorPassive = 1,
}
impl From<Esi> for bool {
    #[inline(always)]
    fn from(variant: Esi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESI` reader - Error State Indicator flag"]
pub type EsiR = crate::BitReader<Esi>;
impl EsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esi {
        match self.bits {
            false => Esi::ErrorActive,
            true => Esi::ErrorPassive,
        }
    }
    #[doc = "A CANFD frame was transmitted by node in the Error Active state. Or a CAN2.0 frame was transmitted"]
    #[inline(always)]
    pub fn is_error_active(&self) -> bool {
        *self == Esi::ErrorActive
    }
    #[doc = "A CANFD frame was transmitted by node in the Error Passive state"]
    #[inline(always)]
    pub fn is_error_passive(&self) -> bool {
        *self == Esi::ErrorPassive
    }
}
#[doc = "Field `ESI` writer - Error State Indicator flag"]
pub type EsiW<'a, REG> = crate::BitWriter<'a, REG, Esi>;
impl<'a, REG> EsiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A CANFD frame was transmitted by node in the Error Active state. Or a CAN2.0 frame was transmitted"]
    #[inline(always)]
    pub fn error_active(self) -> &'a mut crate::W<REG> {
        self.variant(Esi::ErrorActive)
    }
    #[doc = "A CANFD frame was transmitted by node in the Error Passive state"]
    #[inline(always)]
    pub fn error_passive(self) -> &'a mut crate::W<REG> {
        self.variant(Esi::ErrorPassive)
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge Error Interrupt"]
    #[inline(always)]
    pub fn aerr(&self) -> AerrR {
        AerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Error Interrupt"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn cerr(&self) -> CerrR {
        CerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Serial Error"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit Error while Transmit"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive OK"]
    #[inline(always)]
    pub fn rxok(&self) -> RxokR {
        RxokR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit OK"]
    #[inline(always)]
    pub fn txok(&self) -> TxokR {
        TxokR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Length Code Warning"]
    #[inline(always)]
    pub fn dlcw(&self) -> DlcwR {
        DlcwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error State Indicator flag"]
    #[inline(always)]
    pub fn esi(&self) -> EsiR {
        EsiR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledge Error Interrupt"]
    #[inline(always)]
    pub fn aerr(&mut self) -> AerrW<'_, MobstatusSpec> {
        AerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Error Interrupt"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, MobstatusSpec> {
        FerrW::new(self, 1)
    }
    #[doc = "Bit 2 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn cerr(&mut self) -> CerrW<'_, MobstatusSpec> {
        CerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Serial Error"]
    #[inline(always)]
    pub fn serr(&mut self) -> SerrW<'_, MobstatusSpec> {
        SerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Bit Error while Transmit"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, MobstatusSpec> {
        BerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive OK"]
    #[inline(always)]
    pub fn rxok(&mut self) -> RxokW<'_, MobstatusSpec> {
        RxokW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit OK"]
    #[inline(always)]
    pub fn txok(&mut self) -> TxokW<'_, MobstatusSpec> {
        TxokW::new(self, 6)
    }
    #[doc = "Bit 7 - Data Length Code Warning"]
    #[inline(always)]
    pub fn dlcw(&mut self) -> DlcwW<'_, MobstatusSpec> {
        DlcwW::new(self, 7)
    }
    #[doc = "Bit 8 - Error State Indicator flag"]
    #[inline(always)]
    pub fn esi(&mut self) -> EsiW<'_, MobstatusSpec> {
        EsiW::new(self, 8)
    }
}
#[doc = "CAN MOb Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MobstatusSpec;
impl crate::RegisterSpec for MobstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mobstatus::R`](R) reader structure"]
impl crate::Readable for MobstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mobstatus::W`](W) writer structure"]
impl crate::Writable for MobstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOBSTATUS to value 0"]
impl crate::Resettable for MobstatusSpec {}
