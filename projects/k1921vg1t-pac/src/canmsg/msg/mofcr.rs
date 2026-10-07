#[doc = "Register `MOFCR` reader"]
pub type R = crate::R<MofcrSpec>;
#[doc = "Register `MOFCR` writer"]
pub type W = crate::W<MofcrSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mmc {
    #[doc = "0: message object"]
    MsgObj = 0,
    #[doc = "1: receiver FIFO structure object"]
    Rxobj = 1,
    #[doc = "2: transmitter FIFO structure object"]
    Txobj = 2,
    #[doc = "3: transmitter FIFO structure slave object"]
    SlaveTxobj = 3,
    #[doc = "4: gateway source object"]
    SrcObj = 4,
}
impl From<Mmc> for u8 {
    #[inline(always)]
    fn from(variant: Mmc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mmc {
    type Ux = u8;
}
impl crate::IsEnum for Mmc {}
#[doc = "Field `MMC` reader - "]
pub type MmcR = crate::FieldReader<Mmc>;
impl MmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mmc> {
        match self.bits {
            0 => Some(Mmc::MsgObj),
            1 => Some(Mmc::Rxobj),
            2 => Some(Mmc::Txobj),
            3 => Some(Mmc::SlaveTxobj),
            4 => Some(Mmc::SrcObj),
            _ => None,
        }
    }
    #[doc = "message object"]
    #[inline(always)]
    pub fn is_msg_obj(&self) -> bool {
        *self == Mmc::MsgObj
    }
    #[doc = "receiver FIFO structure object"]
    #[inline(always)]
    pub fn is_rxobj(&self) -> bool {
        *self == Mmc::Rxobj
    }
    #[doc = "transmitter FIFO structure object"]
    #[inline(always)]
    pub fn is_txobj(&self) -> bool {
        *self == Mmc::Txobj
    }
    #[doc = "transmitter FIFO structure slave object"]
    #[inline(always)]
    pub fn is_slave_txobj(&self) -> bool {
        *self == Mmc::SlaveTxobj
    }
    #[doc = "gateway source object"]
    #[inline(always)]
    pub fn is_src_obj(&self) -> bool {
        *self == Mmc::SrcObj
    }
}
#[doc = "Field `MMC` writer - "]
pub type MmcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mmc>;
impl<'a, REG> MmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "message object"]
    #[inline(always)]
    pub fn msg_obj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::MsgObj)
    }
    #[doc = "receiver FIFO structure object"]
    #[inline(always)]
    pub fn rxobj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Rxobj)
    }
    #[doc = "transmitter FIFO structure object"]
    #[inline(always)]
    pub fn txobj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Txobj)
    }
    #[doc = "transmitter FIFO structure slave object"]
    #[inline(always)]
    pub fn slave_txobj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::SlaveTxobj)
    }
    #[doc = "gateway source object"]
    #[inline(always)]
    pub fn src_obj(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::SrcObj)
    }
}
#[doc = "Field `GDFS` reader - "]
pub type GdfsR = crate::BitReader;
#[doc = "Field `GDFS` writer - "]
pub type GdfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDC` reader - "]
pub type IdcR = crate::BitReader;
#[doc = "Field `IDC` writer - "]
pub type IdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLCC` reader - "]
pub type DlccR = crate::BitReader;
#[doc = "Field `DLCC` writer - "]
pub type DlccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATC` reader - "]
pub type DatcR = crate::BitReader;
#[doc = "Field `DATC` writer - "]
pub type DatcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIE` reader - Interrupt enable after taking the messages"]
pub type RxieR = crate::BitReader;
#[doc = "Field `RXIE` writer - Interrupt enable after taking the messages"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIE` reader - Interrupt enable at the end of the message"]
pub type TxieR = crate::BitReader;
#[doc = "Field `TXIE` writer - Interrupt enable at the end of the message"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVIE` reader - Interrupt enable FIFO to fill the message object <i>"]
pub type OvieR = crate::BitReader;
#[doc = "Field `OVIE` writer - Interrupt enable FIFO to fill the message object <i>"]
pub type OvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRREN` reader - Enable remote request"]
pub type FrrenR = crate::BitReader;
#[doc = "Field `FRREN` writer - Enable remote request"]
pub type FrrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMM` reader - Enable remote monitoring of the communication object"]
pub type RmmR = crate::BitReader;
#[doc = "Field `RMM` writer - Enable remote monitoring of the communication object"]
pub type RmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDT` reader - Bit single of the message object <i> participation in shipment"]
pub type SdtR = crate::BitReader;
#[doc = "Field `SDT` writer - Bit single of the message object <i> participation in shipment"]
pub type SdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STT` reader - Bit single data transfer"]
pub type SttR = crate::BitReader;
#[doc = "Field `STT` writer - Bit single data transfer"]
pub type SttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLC` reader - Data length code"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data length code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gdfs(&self) -> GdfsR {
        GdfsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idc(&self) -> IdcR {
        IdcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dlcc(&self) -> DlccR {
        DlccR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn datc(&self) -> DatcR {
        DatcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable after taking the messages"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt enable at the end of the message"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt enable FIFO to fill the message object <i>"]
    #[inline(always)]
    pub fn ovie(&self) -> OvieR {
        OvieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable remote request"]
    #[inline(always)]
    pub fn frren(&self) -> FrrenR {
        FrrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable remote monitoring of the communication object"]
    #[inline(always)]
    pub fn rmm(&self) -> RmmR {
        RmmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bit single of the message object <i> participation in shipment"]
    #[inline(always)]
    pub fn sdt(&self) -> SdtR {
        SdtR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bit single data transfer"]
    #[inline(always)]
    pub fn stt(&self) -> SttR {
        SttR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Data length code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mmc(&mut self) -> MmcW<'_, MofcrSpec> {
        MmcW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gdfs(&mut self) -> GdfsW<'_, MofcrSpec> {
        GdfsW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idc(&mut self) -> IdcW<'_, MofcrSpec> {
        IdcW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dlcc(&mut self) -> DlccW<'_, MofcrSpec> {
        DlccW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn datc(&mut self) -> DatcW<'_, MofcrSpec> {
        DatcW::new(self, 11)
    }
    #[doc = "Bit 16 - Interrupt enable after taking the messages"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RxieW<'_, MofcrSpec> {
        RxieW::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt enable at the end of the message"]
    #[inline(always)]
    pub fn txie(&mut self) -> TxieW<'_, MofcrSpec> {
        TxieW::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt enable FIFO to fill the message object <i>"]
    #[inline(always)]
    pub fn ovie(&mut self) -> OvieW<'_, MofcrSpec> {
        OvieW::new(self, 18)
    }
    #[doc = "Bit 20 - Enable remote request"]
    #[inline(always)]
    pub fn frren(&mut self) -> FrrenW<'_, MofcrSpec> {
        FrrenW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable remote monitoring of the communication object"]
    #[inline(always)]
    pub fn rmm(&mut self) -> RmmW<'_, MofcrSpec> {
        RmmW::new(self, 21)
    }
    #[doc = "Bit 22 - Bit single of the message object <i> participation in shipment"]
    #[inline(always)]
    pub fn sdt(&mut self) -> SdtW<'_, MofcrSpec> {
        SdtW::new(self, 22)
    }
    #[doc = "Bit 23 - Bit single data transfer"]
    #[inline(always)]
    pub fn stt(&mut self) -> SttW<'_, MofcrSpec> {
        SttW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Data length code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<'_, MofcrSpec> {
        DlcW::new(self, 24)
    }
}
#[doc = "Register control the operation of the message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mofcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mofcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MofcrSpec;
impl crate::RegisterSpec for MofcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mofcr::R`](R) reader structure"]
impl crate::Readable for MofcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mofcr::W`](W) writer structure"]
impl crate::Writable for MofcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOFCR to value 0"]
impl crate::Resettable for MofcrSpec {}
