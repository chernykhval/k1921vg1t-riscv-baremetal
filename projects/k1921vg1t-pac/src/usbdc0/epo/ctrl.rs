#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EV` reader - Endpoint valid"]
pub type EvR = crate::BitReader;
#[doc = "Field `EV` writer - Endpoint valid"]
pub type EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ED` reader - Endpoint disabled"]
pub type EdR = crate::BitReader;
#[doc = "Field `ED` writer - Endpoint disabled"]
pub type EdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EH` reader - Endpoint halted"]
pub type EhR = crate::BitReader;
#[doc = "Field `EH` writer - Endpoint halted"]
pub type EhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tt {
    #[doc = "0: Control type of EPO"]
    Ctrl = 0,
    #[doc = "1: Isochronous type of EPO"]
    Isoch = 1,
    #[doc = "2: Bulk type of EPO"]
    Bulk = 2,
    #[doc = "3: Interrupt type of EPO"]
    Interrupt = 3,
}
impl From<Tt> for u8 {
    #[inline(always)]
    fn from(variant: Tt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tt {
    type Ux = u8;
}
impl crate::IsEnum for Tt {}
#[doc = "Field `TT` reader - Transfer type"]
pub type TtR = crate::FieldReader<Tt>;
impl TtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tt {
        match self.bits {
            0 => Tt::Ctrl,
            1 => Tt::Isoch,
            2 => Tt::Bulk,
            3 => Tt::Interrupt,
            _ => unreachable!(),
        }
    }
    #[doc = "Control type of EPO"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == Tt::Ctrl
    }
    #[doc = "Isochronous type of EPO"]
    #[inline(always)]
    pub fn is_isoch(&self) -> bool {
        *self == Tt::Isoch
    }
    #[doc = "Bulk type of EPO"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == Tt::Bulk
    }
    #[doc = "Interrupt type of EPO"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Tt::Interrupt
    }
}
#[doc = "Field `TT` writer - Transfer type"]
pub type TtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tt, crate::Safe>;
impl<'a, REG> TtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control type of EPO"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Tt::Ctrl)
    }
    #[doc = "Isochronous type of EPO"]
    #[inline(always)]
    pub fn isoch(self) -> &'a mut crate::W<REG> {
        self.variant(Tt::Isoch)
    }
    #[doc = "Bulk type of EPO"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(Tt::Bulk)
    }
    #[doc = "Interrupt type of EPO"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Tt::Interrupt)
    }
}
#[doc = "Field `NT` reader - Number of transactions"]
pub type NtR = crate::FieldReader;
#[doc = "Field `NT` writer - Number of transactions"]
pub type NtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAXPL` reader - Maximum payload"]
pub type MaxplR = crate::FieldReader<u16>;
#[doc = "Field `MAXPL` writer - Maximum payload"]
pub type MaxplW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CS` reader - Control stall"]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Control stall"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CB` reader - Clear buffers"]
pub type CbR = crate::BitReader;
#[doc = "Field `CB` writer - Clear buffers"]
pub type CbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI` reader - Packet received interrupt"]
pub type PiR = crate::BitReader;
#[doc = "Field `PI` writer - Packet received interrupt"]
pub type PiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFSZ` reader - Buffer size"]
pub type BufszR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Endpoint valid"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    pub fn ed(&self) -> EdR {
        EdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint halted"]
    #[inline(always)]
    pub fn eh(&self) -> EhR {
        EhR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Transfer type"]
    #[inline(always)]
    pub fn tt(&self) -> TtR {
        TtR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Number of transactions"]
    #[inline(always)]
    pub fn nt(&self) -> NtR {
        NtR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:17 - Maximum payload"]
    #[inline(always)]
    pub fn maxpl(&self) -> MaxplR {
        MaxplR::new(((self.bits >> 7) & 0x07ff) as u16)
    }
    #[doc = "Bit 18 - Control stall"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear buffers"]
    #[inline(always)]
    pub fn cb(&self) -> CbR {
        CbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Packet received interrupt"]
    #[inline(always)]
    pub fn pi(&self) -> PiR {
        PiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Buffer size"]
    #[inline(always)]
    pub fn bufsz(&self) -> BufszR {
        BufszR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint valid"]
    #[inline(always)]
    pub fn ev(&mut self) -> EvW<'_, CtrlSpec> {
        EvW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    pub fn ed(&mut self) -> EdW<'_, CtrlSpec> {
        EdW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint halted"]
    #[inline(always)]
    pub fn eh(&mut self) -> EhW<'_, CtrlSpec> {
        EhW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Transfer type"]
    #[inline(always)]
    pub fn tt(&mut self) -> TtW<'_, CtrlSpec> {
        TtW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Number of transactions"]
    #[inline(always)]
    pub fn nt(&mut self) -> NtW<'_, CtrlSpec> {
        NtW::new(self, 5)
    }
    #[doc = "Bits 7:17 - Maximum payload"]
    #[inline(always)]
    pub fn maxpl(&mut self) -> MaxplW<'_, CtrlSpec> {
        MaxplW::new(self, 7)
    }
    #[doc = "Bit 18 - Control stall"]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<'_, CtrlSpec> {
        CsW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear buffers"]
    #[inline(always)]
    pub fn cb(&mut self) -> CbW<'_, CtrlSpec> {
        CbW::new(self, 19)
    }
    #[doc = "Bit 20 - Packet received interrupt"]
    #[inline(always)]
    pub fn pi(&mut self) -> PiW<'_, CtrlSpec> {
        PiW::new(self, 20)
    }
}
#[doc = "OUT Endpoint control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
