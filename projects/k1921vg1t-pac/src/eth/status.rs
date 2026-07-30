#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `RXERR` reader - Receiver error"]
pub type RxerrR = crate::BitReader;
#[doc = "Field `RXERR` writer - Receiver error"]
pub type RxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` reader - Transmitter error"]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TXERR` writer - Transmitter error"]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSUC` reader - Receive successful"]
pub type RxsucR = crate::BitReader;
#[doc = "Field `RXSUC` writer - Receive successful"]
pub type RxsucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSUC` reader - Transmit successful"]
pub type TxsucR = crate::BitReader;
#[doc = "Field `TXSUC` writer - Transmit successful"]
pub type TxsucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXAHBERR` reader - Receiver AHB error"]
pub type RxahberrR = crate::BitReader;
#[doc = "Field `RXAHBERR` writer - Receiver AHB error"]
pub type RxahberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXAHBERR` reader - Transmitter AHB error"]
pub type TxahberrR = crate::BitReader;
#[doc = "Field `TXAHBERR` writer - Transmitter AHB error"]
pub type TxahberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOOSMALL` reader - Too small"]
pub type ToosmallR = crate::BitReader;
#[doc = "Field `TOOSMALL` writer - Too small"]
pub type ToosmallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IA` reader - Invalid address"]
pub type IaR = crate::BitReader;
#[doc = "Field `IA` writer - Invalid address"]
pub type IaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSTAT` reader - PHY status changes"]
pub type PhystatR = crate::BitReader;
#[doc = "Field `PHYSTAT` writer - PHY status changes"]
pub type PhystatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver error"]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter error"]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive successful"]
    #[inline(always)]
    pub fn rxsuc(&self) -> RxsucR {
        RxsucR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit successful"]
    #[inline(always)]
    pub fn txsuc(&self) -> TxsucR {
        TxsucR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receiver AHB error"]
    #[inline(always)]
    pub fn rxahberr(&self) -> RxahberrR {
        RxahberrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter AHB error"]
    #[inline(always)]
    pub fn txahberr(&self) -> TxahberrR {
        TxahberrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Too small"]
    #[inline(always)]
    pub fn toosmall(&self) -> ToosmallR {
        ToosmallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Invalid address"]
    #[inline(always)]
    pub fn ia(&self) -> IaR {
        IaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PHY status changes"]
    #[inline(always)]
    pub fn phystat(&self) -> PhystatR {
        PhystatR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver error"]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RxerrW<'_, StatusSpec> {
        RxerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitter error"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TxerrW<'_, StatusSpec> {
        TxerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive successful"]
    #[inline(always)]
    pub fn rxsuc(&mut self) -> RxsucW<'_, StatusSpec> {
        RxsucW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit successful"]
    #[inline(always)]
    pub fn txsuc(&mut self) -> TxsucW<'_, StatusSpec> {
        TxsucW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver AHB error"]
    #[inline(always)]
    pub fn rxahberr(&mut self) -> RxahberrW<'_, StatusSpec> {
        RxahberrW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmitter AHB error"]
    #[inline(always)]
    pub fn txahberr(&mut self) -> TxahberrW<'_, StatusSpec> {
        TxahberrW::new(self, 5)
    }
    #[doc = "Bit 6 - Too small"]
    #[inline(always)]
    pub fn toosmall(&mut self) -> ToosmallW<'_, StatusSpec> {
        ToosmallW::new(self, 6)
    }
    #[doc = "Bit 7 - Invalid address"]
    #[inline(always)]
    pub fn ia(&mut self) -> IaW<'_, StatusSpec> {
        IaW::new(self, 7)
    }
    #[doc = "Bit 8 - PHY status changes"]
    #[inline(always)]
    pub fn phystat(&mut self) -> PhystatW<'_, StatusSpec> {
        PhystatW::new(self, 8)
    }
}
#[doc = "Ethernet status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
