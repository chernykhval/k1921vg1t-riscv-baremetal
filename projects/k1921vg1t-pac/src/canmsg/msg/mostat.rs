#[doc = "Register `MOSTAT` reader"]
pub type R = crate::R<MostatSpec>;
#[doc = "Field `RXPND` reader - Indicator deadline"]
pub type RxpndR = crate::BitReader;
#[doc = "Field `TXPND` reader - Indicator end of transmission"]
pub type TxpndR = crate::BitReader;
#[doc = "Field `RXUPD` reader - Indicator changes"]
pub type RxupdR = crate::BitReader;
#[doc = "Field `NEWDAT` reader - New data indicator"]
pub type NewdatR = crate::BitReader;
#[doc = "Field `MSGLST` reader - Bit message loss"]
pub type MsglstR = crate::BitReader;
#[doc = "Field `MSGVAL` reader - Activity bit of the message object <i>"]
pub type MsgvalR = crate::BitReader;
#[doc = "Field `RTSEL` reader - The indication of transmission / reception"]
pub type RtselR = crate::BitReader;
#[doc = "Field `RXEN` reader - Bits allow reception"]
pub type RxenR = crate::BitReader;
#[doc = "Field `TXRQ` reader - Initiate transmission"]
pub type TxrqR = crate::BitReader;
#[doc = "Field `TXEN0` reader - Enable bit transmission frame"]
pub type Txen0R = crate::BitReader;
#[doc = "Field `TXEN1` reader - Enable bit transmission frame"]
pub type Txen1R = crate::BitReader;
#[doc = "Field `DIR` reader - Bit allocation"]
pub type DirR = crate::BitReader;
#[doc = "Field `LIST` reader - Number list the message object <i>"]
pub type ListR = crate::FieldReader;
#[doc = "Field `PPREV` reader - Pointer to the previous entry"]
pub type PprevR = crate::FieldReader;
#[doc = "Field `PNEXT` reader - Pointer to the next item in the list"]
pub type PnextR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicator deadline"]
    #[inline(always)]
    pub fn rxpnd(&self) -> RxpndR {
        RxpndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicator end of transmission"]
    #[inline(always)]
    pub fn txpnd(&self) -> TxpndR {
        TxpndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicator changes"]
    #[inline(always)]
    pub fn rxupd(&self) -> RxupdR {
        RxupdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New data indicator"]
    #[inline(always)]
    pub fn newdat(&self) -> NewdatR {
        NewdatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit message loss"]
    #[inline(always)]
    pub fn msglst(&self) -> MsglstR {
        MsglstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Activity bit of the message object <i>"]
    #[inline(always)]
    pub fn msgval(&self) -> MsgvalR {
        MsgvalR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The indication of transmission / reception"]
    #[inline(always)]
    pub fn rtsel(&self) -> RtselR {
        RtselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bits allow reception"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Initiate transmission"]
    #[inline(always)]
    pub fn txrq(&self) -> TxrqR {
        TxrqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable bit transmission frame"]
    #[inline(always)]
    pub fn txen0(&self) -> Txen0R {
        Txen0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable bit transmission frame"]
    #[inline(always)]
    pub fn txen1(&self) -> Txen1R {
        Txen1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit allocation"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Number list the message object <i>"]
    #[inline(always)]
    pub fn list(&self) -> ListR {
        ListR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Pointer to the previous entry"]
    #[inline(always)]
    pub fn pprev(&self) -> PprevR {
        PprevR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Pointer to the next item in the list"]
    #[inline(always)]
    pub fn pnext(&self) -> PnextR {
        PnextR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Status register of the message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mostat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MostatSpec;
impl crate::RegisterSpec for MostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mostat::R`](R) reader structure"]
impl crate::Readable for MostatSpec {}
#[doc = "`reset()` method sets MOSTAT to value 0"]
impl crate::Resettable for MostatSpec {}
