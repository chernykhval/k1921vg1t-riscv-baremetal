#[doc = "Register `MOCTR` writer"]
pub type W = crate::W<MoctrSpec>;
#[doc = "Field `RESRXPND` writer - Reset bit RXPND"]
pub type ResrxpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXPND` writer - Reset bit TXPND"]
pub type RestxpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRXUPD` writer - Reset bit RXUPD"]
pub type ResrxupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESNEWDAT` writer - Reset bit NEWDAT"]
pub type ResnewdatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESMSGLST` writer - Reset bit MSGLST"]
pub type ResmsglstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESMSGVAL` writer - Reset bit MSGVAL"]
pub type ResmsgvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRTSEL` writer - Reset bit RTSEL"]
pub type ResrtselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRXEN` writer - Reset bit RXEN"]
pub type ResrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXRQ` writer - Reset bit TXRQ"]
pub type RestxrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXEN0` writer - Reset bit TXEN0"]
pub type Restxen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTXEN1` writer - Reset bit TXEN1"]
pub type Restxen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESDIR` writer - Reset bit DIR"]
pub type ResdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXPND` writer - Set bit RXPND"]
pub type SetrxpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXPND` writer - Set bit TXPND"]
pub type SettxpndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXUPD` writer - Set bit RXUPD"]
pub type SetrxupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETNEWDAT` writer - Set bit NEWDAT"]
pub type SetnewdatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETMSGLST` writer - Set bit MSGLST"]
pub type SetmsglstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETMSGVAL` writer - Set bit MSGVAL"]
pub type SetmsgvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRTSEL` writer - Set bit RTSEL"]
pub type SetrtselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXEN` writer - Set bit RXEN"]
pub type SetrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXRQ` writer - Set bit TXRQ"]
pub type SettxrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXEN0` writer - Set bit TXEN0"]
pub type Settxen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXEN1` writer - Set bit TXEN1"]
pub type Settxen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDIR` writer - Set bit DIR"]
pub type SetdirW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Reset bit RXPND"]
    #[inline(always)]
    pub fn resrxpnd(&mut self) -> ResrxpndW<'_, MoctrSpec> {
        ResrxpndW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset bit TXPND"]
    #[inline(always)]
    pub fn restxpnd(&mut self) -> RestxpndW<'_, MoctrSpec> {
        RestxpndW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset bit RXUPD"]
    #[inline(always)]
    pub fn resrxupd(&mut self) -> ResrxupdW<'_, MoctrSpec> {
        ResrxupdW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset bit NEWDAT"]
    #[inline(always)]
    pub fn resnewdat(&mut self) -> ResnewdatW<'_, MoctrSpec> {
        ResnewdatW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset bit MSGLST"]
    #[inline(always)]
    pub fn resmsglst(&mut self) -> ResmsglstW<'_, MoctrSpec> {
        ResmsglstW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset bit MSGVAL"]
    #[inline(always)]
    pub fn resmsgval(&mut self) -> ResmsgvalW<'_, MoctrSpec> {
        ResmsgvalW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset bit RTSEL"]
    #[inline(always)]
    pub fn resrtsel(&mut self) -> ResrtselW<'_, MoctrSpec> {
        ResrtselW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset bit RXEN"]
    #[inline(always)]
    pub fn resrxen(&mut self) -> ResrxenW<'_, MoctrSpec> {
        ResrxenW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset bit TXRQ"]
    #[inline(always)]
    pub fn restxrq(&mut self) -> RestxrqW<'_, MoctrSpec> {
        RestxrqW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset bit TXEN0"]
    #[inline(always)]
    pub fn restxen0(&mut self) -> Restxen0W<'_, MoctrSpec> {
        Restxen0W::new(self, 9)
    }
    #[doc = "Bit 10 - Reset bit TXEN1"]
    #[inline(always)]
    pub fn restxen1(&mut self) -> Restxen1W<'_, MoctrSpec> {
        Restxen1W::new(self, 10)
    }
    #[doc = "Bit 11 - Reset bit DIR"]
    #[inline(always)]
    pub fn resdir(&mut self) -> ResdirW<'_, MoctrSpec> {
        ResdirW::new(self, 11)
    }
    #[doc = "Bit 16 - Set bit RXPND"]
    #[inline(always)]
    pub fn setrxpnd(&mut self) -> SetrxpndW<'_, MoctrSpec> {
        SetrxpndW::new(self, 16)
    }
    #[doc = "Bit 17 - Set bit TXPND"]
    #[inline(always)]
    pub fn settxpnd(&mut self) -> SettxpndW<'_, MoctrSpec> {
        SettxpndW::new(self, 17)
    }
    #[doc = "Bit 18 - Set bit RXUPD"]
    #[inline(always)]
    pub fn setrxupd(&mut self) -> SetrxupdW<'_, MoctrSpec> {
        SetrxupdW::new(self, 18)
    }
    #[doc = "Bit 19 - Set bit NEWDAT"]
    #[inline(always)]
    pub fn setnewdat(&mut self) -> SetnewdatW<'_, MoctrSpec> {
        SetnewdatW::new(self, 19)
    }
    #[doc = "Bit 20 - Set bit MSGLST"]
    #[inline(always)]
    pub fn setmsglst(&mut self) -> SetmsglstW<'_, MoctrSpec> {
        SetmsglstW::new(self, 20)
    }
    #[doc = "Bit 21 - Set bit MSGVAL"]
    #[inline(always)]
    pub fn setmsgval(&mut self) -> SetmsgvalW<'_, MoctrSpec> {
        SetmsgvalW::new(self, 21)
    }
    #[doc = "Bit 22 - Set bit RTSEL"]
    #[inline(always)]
    pub fn setrtsel(&mut self) -> SetrtselW<'_, MoctrSpec> {
        SetrtselW::new(self, 22)
    }
    #[doc = "Bit 23 - Set bit RXEN"]
    #[inline(always)]
    pub fn setrxen(&mut self) -> SetrxenW<'_, MoctrSpec> {
        SetrxenW::new(self, 23)
    }
    #[doc = "Bit 24 - Set bit TXRQ"]
    #[inline(always)]
    pub fn settxrq(&mut self) -> SettxrqW<'_, MoctrSpec> {
        SettxrqW::new(self, 24)
    }
    #[doc = "Bit 25 - Set bit TXEN0"]
    #[inline(always)]
    pub fn settxen0(&mut self) -> Settxen0W<'_, MoctrSpec> {
        Settxen0W::new(self, 25)
    }
    #[doc = "Bit 26 - Set bit TXEN1"]
    #[inline(always)]
    pub fn settxen1(&mut self) -> Settxen1W<'_, MoctrSpec> {
        Settxen1W::new(self, 26)
    }
    #[doc = "Bit 27 - Set bit DIR"]
    #[inline(always)]
    pub fn setdir(&mut self) -> SetdirW<'_, MoctrSpec> {
        SetdirW::new(self, 27)
    }
}
#[doc = "Control register Message object 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moctr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoctrSpec;
impl crate::RegisterSpec for MoctrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`moctr::W`](W) writer structure"]
impl crate::Writable for MoctrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOCTR to value 0"]
impl crate::Resettable for MoctrSpec {}
