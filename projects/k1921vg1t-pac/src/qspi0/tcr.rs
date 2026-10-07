#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `LEN` reader - Size of data"]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - Size of data"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "LSB First ( 0 - MSB First; 1 - LSB First )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsbf {
    #[doc = "0: MSB First"]
    Msb = 0,
    #[doc = "1: LSB First"]
    Lsb = 1,
}
impl From<Lsbf> for bool {
    #[inline(always)]
    fn from(variant: Lsbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBF` reader - LSB First ( 0 - MSB First; 1 - LSB First )"]
pub type LsbfR = crate::BitReader<Lsbf>;
impl LsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsbf {
        match self.bits {
            false => Lsbf::Msb,
            true => Lsbf::Lsb,
        }
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Lsbf::Msb
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Lsbf::Lsb
    }
}
#[doc = "Field `LSBF` writer - LSB First ( 0 - MSB First; 1 - LSB First )"]
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG, Lsbf>;
impl<'a, REG> LsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB First"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::Msb)
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::Lsb)
    }
}
#[doc = "Double Data Rate ( 0 - SDR ; 1 - DDR )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddr {
    #[doc = "0: Single Data Rate"]
    Sdr = 0,
    #[doc = "1: Double Data Rate"]
    Ddr = 1,
}
impl From<Ddr> for bool {
    #[inline(always)]
    fn from(variant: Ddr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR` reader - Double Data Rate ( 0 - SDR ; 1 - DDR )"]
pub type DdrR = crate::BitReader<Ddr>;
impl DdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr {
        match self.bits {
            false => Ddr::Sdr,
            true => Ddr::Ddr,
        }
    }
    #[doc = "Single Data Rate"]
    #[inline(always)]
    pub fn is_sdr(&self) -> bool {
        *self == Ddr::Sdr
    }
    #[doc = "Double Data Rate"]
    #[inline(always)]
    pub fn is_ddr(&self) -> bool {
        *self == Ddr::Ddr
    }
}
#[doc = "Field `DDR` writer - Double Data Rate ( 0 - SDR ; 1 - DDR )"]
pub type DdrW<'a, REG> = crate::BitWriter<'a, REG, Ddr>;
impl<'a, REG> DdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single Data Rate"]
    #[inline(always)]
    pub fn sdr(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr::Sdr)
    }
    #[doc = "Double Data Rate"]
    #[inline(always)]
    pub fn ddr(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr::Ddr)
    }
}
#[doc = "Field `TXE` reader - Serial Transmitter Enable"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - Serial Transmitter Enable"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - Serial Receiver Enable"]
pub type RxeR = crate::BitReader;
#[doc = "Field `RXE` writer - Serial Receiver Enable"]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IO Mode ( SIO/DIO/QIO ) Single/Double/Quad lines for serial IO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iom {
    #[doc = "0: Single lines for serial IO"]
    Single = 0,
    #[doc = "1: Double lines for serial IO"]
    Double = 1,
    #[doc = "2: Quad lines for serial IO"]
    Quad = 2,
}
impl From<Iom> for u8 {
    #[inline(always)]
    fn from(variant: Iom) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iom {
    type Ux = u8;
}
impl crate::IsEnum for Iom {}
#[doc = "Field `IOM` reader - IO Mode ( SIO/DIO/QIO ) Single/Double/Quad lines for serial IO"]
pub type IomR = crate::FieldReader<Iom>;
impl IomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iom> {
        match self.bits {
            0 => Some(Iom::Single),
            1 => Some(Iom::Double),
            2 => Some(Iom::Quad),
            _ => None,
        }
    }
    #[doc = "Single lines for serial IO"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Iom::Single
    }
    #[doc = "Double lines for serial IO"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == Iom::Double
    }
    #[doc = "Quad lines for serial IO"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == Iom::Quad
    }
}
#[doc = "Field `IOM` writer - IO Mode ( SIO/DIO/QIO ) Single/Double/Quad lines for serial IO"]
pub type IomW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iom>;
impl<'a, REG> IomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single lines for serial IO"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Iom::Single)
    }
    #[doc = "Double lines for serial IO"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(Iom::Double)
    }
    #[doc = "Quad lines for serial IO"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(Iom::Quad)
    }
}
impl R {
    #[doc = "Bits 0:5 - Size of data"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - LSB First ( 0 - MSB First; 1 - LSB First )"]
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Double Data Rate ( 0 - SDR ; 1 - DDR )"]
    #[inline(always)]
    pub fn ddr(&self) -> DdrR {
        DdrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Serial Transmitter Enable"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Serial Receiver Enable"]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - IO Mode ( SIO/DIO/QIO ) Single/Double/Quad lines for serial IO"]
    #[inline(always)]
    pub fn iom(&self) -> IomR {
        IomR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Size of data"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<'_, TcrSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bit 6 - LSB First ( 0 - MSB First; 1 - LSB First )"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LsbfW<'_, TcrSpec> {
        LsbfW::new(self, 6)
    }
    #[doc = "Bit 7 - Double Data Rate ( 0 - SDR ; 1 - DDR )"]
    #[inline(always)]
    pub fn ddr(&mut self) -> DdrW<'_, TcrSpec> {
        DdrW::new(self, 7)
    }
    #[doc = "Bit 8 - Serial Transmitter Enable"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, TcrSpec> {
        TxeW::new(self, 8)
    }
    #[doc = "Bit 9 - Serial Receiver Enable"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RxeW<'_, TcrSpec> {
        RxeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - IO Mode ( SIO/DIO/QIO ) Single/Double/Quad lines for serial IO"]
    #[inline(always)]
    pub fn iom(&mut self) -> IomW<'_, TcrSpec> {
        IomW::new(self, 10)
    }
}
#[doc = "Transfer Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {}
