#[doc = "Register `U_CTRL` reader"]
pub type R = crate::R<UCtrlSpec>;
#[doc = "Register `U_CTRL` writer"]
pub type W = crate::W<UCtrlSpec>;
#[doc = "Upstream Frame Type (0 - 12-bit; 1 - 16bit)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft {
    #[doc = "0: Upstream without 4-bit address field"]
    Frame12 = 0,
    #[doc = "1: full length 16-bit upstream frame"]
    Frame16 = 1,
}
impl From<Ft> for bool {
    #[inline(always)]
    fn from(variant: Ft) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT` reader - Upstream Frame Type (0 - 12-bit; 1 - 16bit)"]
pub type FtR = crate::BitReader<Ft>;
impl FtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft {
        match self.bits {
            false => Ft::Frame12,
            true => Ft::Frame16,
        }
    }
    #[doc = "Upstream without 4-bit address field"]
    #[inline(always)]
    pub fn is_frame_12(&self) -> bool {
        *self == Ft::Frame12
    }
    #[doc = "full length 16-bit upstream frame"]
    #[inline(always)]
    pub fn is_frame_16(&self) -> bool {
        *self == Ft::Frame16
    }
}
#[doc = "Field `FT` writer - Upstream Frame Type (0 - 12-bit; 1 - 16bit)"]
pub type FtW<'a, REG> = crate::BitWriter<'a, REG, Ft>;
impl<'a, REG> FtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Upstream without 4-bit address field"]
    #[inline(always)]
    pub fn frame_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ft::Frame12)
    }
    #[doc = "full length 16-bit upstream frame"]
    #[inline(always)]
    pub fn frame_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ft::Frame16)
    }
}
#[doc = "Upstream Channel Receiving Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rbr {
    #[doc = "0: Upstream channel disabled"]
    TurnOff = 0,
    #[doc = "1: Baud rate = Fmsc/4"]
    Div4 = 1,
    #[doc = "2: Baud rate = Fmsc/8"]
    Div8 = 2,
    #[doc = "3: Baud rate = Fmsc/16"]
    Div16 = 3,
    #[doc = "4: 4Baud rate = Fmsc/32"]
    Div32 = 4,
    #[doc = "5: Baud rate = Fmsc/64"]
    Div64 = 5,
    #[doc = "6: Baud rate = Fmsc/128"]
    Div128 = 6,
    #[doc = "7: Baud rate = Fmsc/256"]
    Div256 = 7,
}
impl From<Rbr> for u8 {
    #[inline(always)]
    fn from(variant: Rbr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rbr {
    type Ux = u8;
}
impl crate::IsEnum for Rbr {}
#[doc = "Field `RBR` reader - Upstream Channel Receiving Rate"]
pub type RbrR = crate::FieldReader<Rbr>;
impl RbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbr {
        match self.bits {
            0 => Rbr::TurnOff,
            1 => Rbr::Div4,
            2 => Rbr::Div8,
            3 => Rbr::Div16,
            4 => Rbr::Div32,
            5 => Rbr::Div64,
            6 => Rbr::Div128,
            7 => Rbr::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "Upstream channel disabled"]
    #[inline(always)]
    pub fn is_turn_off(&self) -> bool {
        *self == Rbr::TurnOff
    }
    #[doc = "Baud rate = Fmsc/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Rbr::Div4
    }
    #[doc = "Baud rate = Fmsc/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Rbr::Div8
    }
    #[doc = "Baud rate = Fmsc/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Rbr::Div16
    }
    #[doc = "4Baud rate = Fmsc/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Rbr::Div32
    }
    #[doc = "Baud rate = Fmsc/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Rbr::Div64
    }
    #[doc = "Baud rate = Fmsc/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Rbr::Div128
    }
    #[doc = "Baud rate = Fmsc/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Rbr::Div256
    }
}
#[doc = "Field `RBR` writer - Upstream Channel Receiving Rate"]
pub type RbrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rbr, crate::Safe>;
impl<'a, REG> RbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Upstream channel disabled"]
    #[inline(always)]
    pub fn turn_off(self) -> &'a mut crate::W<REG> {
        self.variant(Rbr::TurnOff)
    }
    #[doc = "Baud rate = Fmsc/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Rbr::Div4)
    }
    #[doc = "Baud rate = Fmsc/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Rbr::Div8)
    }
    #[doc = "Baud rate = Fmsc/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Rbr::Div16)
    }
    #[doc = "4Baud rate = Fmsc/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Rbr::Div32)
    }
    #[doc = "Baud rate = Fmsc/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Rbr::Div64)
    }
    #[doc = "Baud rate = Fmsc/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Rbr::Div128)
    }
    #[doc = "Baud rate = Fmsc/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Rbr::Div256)
    }
}
#[doc = "Parity type (0 - Even parity; 1 - Odd parity)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pt {
    #[doc = "0: Even parity mode - a parity bit is set on an odd number of 1s in the serial address/data stream"]
    Even = 0,
    #[doc = "1: Odd parity mode - a parity bit is set on an even number of 1s in the serial address/data stream"]
    Odd = 1,
}
impl From<Pt> for bool {
    #[inline(always)]
    fn from(variant: Pt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PT` reader - Parity type (0 - Even parity; 1 - Odd parity)"]
pub type PtR = crate::BitReader<Pt>;
impl PtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pt {
        match self.bits {
            false => Pt::Even,
            true => Pt::Odd,
        }
    }
    #[doc = "Even parity mode - a parity bit is set on an odd number of 1s in the serial address/data stream"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Pt::Even
    }
    #[doc = "Odd parity mode - a parity bit is set on an even number of 1s in the serial address/data stream"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Pt::Odd
    }
}
#[doc = "Field `PT` writer - Parity type (0 - Even parity; 1 - Odd parity)"]
pub type PtW<'a, REG> = crate::BitWriter<'a, REG, Pt>;
impl<'a, REG> PtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity mode - a parity bit is set on an odd number of 1s in the serial address/data stream"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Pt::Even)
    }
    #[doc = "Odd parity mode - a parity bit is set on an even number of 1s in the serial address/data stream"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Pt::Odd)
    }
}
#[doc = "Upstream time out timer prescaler of receive baud rate frequency (Fbr)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Top {
    #[doc = "0: Upstream timer prescaler disabled"]
    TurnOff = 0,
    #[doc = "1: Fbr/1"]
    Div1 = 1,
    #[doc = "2: Fbr/2"]
    Div2 = 2,
    #[doc = "3: Fbr/4"]
    Div4 = 3,
    #[doc = "4: Fbr/8"]
    Div8 = 4,
    #[doc = "5: Fbr/16"]
    Div16 = 5,
    #[doc = "6: Fbr/32"]
    Div32 = 6,
    #[doc = "7: Fbr/64"]
    Div64 = 7,
    #[doc = "8: Fbr/128"]
    Div128 = 8,
    #[doc = "9: Fbr/256"]
    Div256 = 9,
    #[doc = "10: Fbr/512"]
    Div512 = 10,
    #[doc = "11: Fbr/1024"]
    Div1024 = 11,
    #[doc = "12: Fbr/2048"]
    Div2048 = 12,
    #[doc = "13: Fbr/4096"]
    Div4096 = 13,
    #[doc = "14: Fbr/8192"]
    Div8192 = 14,
    #[doc = "15: Fbr/16384"]
    Div16384 = 15,
}
impl From<Top> for u8 {
    #[inline(always)]
    fn from(variant: Top) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Top {
    type Ux = u8;
}
impl crate::IsEnum for Top {}
#[doc = "Field `TOP` reader - Upstream time out timer prescaler of receive baud rate frequency (Fbr)"]
pub type TopR = crate::FieldReader<Top>;
impl TopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Top {
        match self.bits {
            0 => Top::TurnOff,
            1 => Top::Div1,
            2 => Top::Div2,
            3 => Top::Div4,
            4 => Top::Div8,
            5 => Top::Div16,
            6 => Top::Div32,
            7 => Top::Div64,
            8 => Top::Div128,
            9 => Top::Div256,
            10 => Top::Div512,
            11 => Top::Div1024,
            12 => Top::Div2048,
            13 => Top::Div4096,
            14 => Top::Div8192,
            15 => Top::Div16384,
            _ => unreachable!(),
        }
    }
    #[doc = "Upstream timer prescaler disabled"]
    #[inline(always)]
    pub fn is_turn_off(&self) -> bool {
        *self == Top::TurnOff
    }
    #[doc = "Fbr/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Top::Div1
    }
    #[doc = "Fbr/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Top::Div2
    }
    #[doc = "Fbr/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Top::Div4
    }
    #[doc = "Fbr/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Top::Div8
    }
    #[doc = "Fbr/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Top::Div16
    }
    #[doc = "Fbr/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Top::Div32
    }
    #[doc = "Fbr/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Top::Div64
    }
    #[doc = "Fbr/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Top::Div128
    }
    #[doc = "Fbr/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Top::Div256
    }
    #[doc = "Fbr/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Top::Div512
    }
    #[doc = "Fbr/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Top::Div1024
    }
    #[doc = "Fbr/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Top::Div2048
    }
    #[doc = "Fbr/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == Top::Div4096
    }
    #[doc = "Fbr/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == Top::Div8192
    }
    #[doc = "Fbr/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == Top::Div16384
    }
}
#[doc = "Field `TOP` writer - Upstream time out timer prescaler of receive baud rate frequency (Fbr)"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 4, Top, crate::Safe>;
impl<'a, REG> TopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Upstream timer prescaler disabled"]
    #[inline(always)]
    pub fn turn_off(self) -> &'a mut crate::W<REG> {
        self.variant(Top::TurnOff)
    }
    #[doc = "Fbr/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div1)
    }
    #[doc = "Fbr/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div2)
    }
    #[doc = "Fbr/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div4)
    }
    #[doc = "Fbr/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div8)
    }
    #[doc = "Fbr/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div16)
    }
    #[doc = "Fbr/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div32)
    }
    #[doc = "Fbr/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div64)
    }
    #[doc = "Fbr/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div128)
    }
    #[doc = "Fbr/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div256)
    }
    #[doc = "Fbr/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div512)
    }
    #[doc = "Fbr/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div1024)
    }
    #[doc = "Fbr/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div2048)
    }
    #[doc = "Fbr/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div4096)
    }
    #[doc = "Fbr/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div8192)
    }
    #[doc = "Fbr/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(Top::Div16384)
    }
}
#[doc = "Field `TOV` reader - Upstream time out timer restart value"]
pub type TovR = crate::FieldReader;
#[doc = "Field `TOV` writer - Upstream time out timer restart value"]
pub type TovW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UC` reader - Upstream bits down counter"]
pub type UcR = crate::FieldReader;
#[doc = "Field `L_ADDR` reader - Last received frame address"]
pub type LAddrR = crate::FieldReader;
#[doc = "Field `TOC` reader - Upstream time out timer counter current value"]
pub type TocR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Upstream Frame Type (0 - 12-bit; 1 - 16bit)"]
    #[inline(always)]
    pub fn ft(&self) -> FtR {
        FtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Upstream Channel Receiving Rate"]
    #[inline(always)]
    pub fn rbr(&self) -> RbrR {
        RbrR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Parity type (0 - Even parity; 1 - Odd parity)"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Upstream time out timer prescaler of receive baud rate frequency (Fbr)"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Upstream time out timer restart value"]
    #[inline(always)]
    pub fn tov(&self) -> TovR {
        TovR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Upstream bits down counter"]
    #[inline(always)]
    pub fn uc(&self) -> UcR {
        UcR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Last received frame address"]
    #[inline(always)]
    pub fn l_addr(&self) -> LAddrR {
        LAddrR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Upstream time out timer counter current value"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Upstream Frame Type (0 - 12-bit; 1 - 16bit)"]
    #[inline(always)]
    pub fn ft(&mut self) -> FtW<'_, UCtrlSpec> {
        FtW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Upstream Channel Receiving Rate"]
    #[inline(always)]
    pub fn rbr(&mut self) -> RbrW<'_, UCtrlSpec> {
        RbrW::new(self, 1)
    }
    #[doc = "Bit 4 - Parity type (0 - Even parity; 1 - Odd parity)"]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<'_, UCtrlSpec> {
        PtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Upstream time out timer prescaler of receive baud rate frequency (Fbr)"]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<'_, UCtrlSpec> {
        TopW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Upstream time out timer restart value"]
    #[inline(always)]
    pub fn tov(&mut self) -> TovW<'_, UCtrlSpec> {
        TovW::new(self, 12)
    }
}
#[doc = "Upstream Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`u_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCtrlSpec;
impl crate::RegisterSpec for UCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_ctrl::R`](R) reader structure"]
impl crate::Readable for UCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`u_ctrl::W`](W) writer structure"]
impl crate::Writable for UCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U_CTRL to value 0"]
impl crate::Resettable for UCtrlSpec {}
