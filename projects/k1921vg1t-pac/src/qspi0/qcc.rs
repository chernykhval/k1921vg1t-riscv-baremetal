#[doc = "Register `QCC` reader"]
pub type R = crate::R<QccSpec>;
#[doc = "Register `QCC` writer"]
pub type W = crate::W<QccSpec>;
#[doc = "Field `INST` reader - Instruction to be sent to the external SPI defice"]
pub type InstR = crate::FieldReader;
#[doc = "Field `INST` writer - Instruction to be sent to the external SPI defice"]
pub type InstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Instruction mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Imod {
    #[doc = "0: No instruction"]
    No = 0,
    #[doc = "1: Instruction on a single line"]
    Sio = 1,
    #[doc = "2: Instruction on two lines"]
    Dio = 2,
    #[doc = "3: Instruction on four lines"]
    Qio = 3,
}
impl From<Imod> for u8 {
    #[inline(always)]
    fn from(variant: Imod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Imod {
    type Ux = u8;
}
impl crate::IsEnum for Imod {}
#[doc = "Field `IMOD` reader - Instruction mode"]
pub type ImodR = crate::FieldReader<Imod>;
impl ImodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imod {
        match self.bits {
            0 => Imod::No,
            1 => Imod::Sio,
            2 => Imod::Dio,
            3 => Imod::Qio,
            _ => unreachable!(),
        }
    }
    #[doc = "No instruction"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Imod::No
    }
    #[doc = "Instruction on a single line"]
    #[inline(always)]
    pub fn is_sio(&self) -> bool {
        *self == Imod::Sio
    }
    #[doc = "Instruction on two lines"]
    #[inline(always)]
    pub fn is_dio(&self) -> bool {
        *self == Imod::Dio
    }
    #[doc = "Instruction on four lines"]
    #[inline(always)]
    pub fn is_qio(&self) -> bool {
        *self == Imod::Qio
    }
}
#[doc = "Field `IMOD` writer - Instruction mode"]
pub type ImodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Imod, crate::Safe>;
impl<'a, REG> ImodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No instruction"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Imod::No)
    }
    #[doc = "Instruction on a single line"]
    #[inline(always)]
    pub fn sio(self) -> &'a mut crate::W<REG> {
        self.variant(Imod::Sio)
    }
    #[doc = "Instruction on two lines"]
    #[inline(always)]
    pub fn dio(self) -> &'a mut crate::W<REG> {
        self.variant(Imod::Dio)
    }
    #[doc = "Instruction on four lines"]
    #[inline(always)]
    pub fn qio(self) -> &'a mut crate::W<REG> {
        self.variant(Imod::Qio)
    }
}
#[doc = "Address mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Admod {
    #[doc = "0: No address"]
    No = 0,
    #[doc = "1: Address on a single line"]
    AddrSio = 1,
    #[doc = "2: Address on two lines"]
    AddrDio = 2,
    #[doc = "3: Address on four lines"]
    AddrQio = 3,
}
impl From<Admod> for u8 {
    #[inline(always)]
    fn from(variant: Admod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Admod {
    type Ux = u8;
}
impl crate::IsEnum for Admod {}
#[doc = "Field `ADMOD` reader - Address mode"]
pub type AdmodR = crate::FieldReader<Admod>;
impl AdmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admod {
        match self.bits {
            0 => Admod::No,
            1 => Admod::AddrSio,
            2 => Admod::AddrDio,
            3 => Admod::AddrQio,
            _ => unreachable!(),
        }
    }
    #[doc = "No address"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Admod::No
    }
    #[doc = "Address on a single line"]
    #[inline(always)]
    pub fn is_addr_sio(&self) -> bool {
        *self == Admod::AddrSio
    }
    #[doc = "Address on two lines"]
    #[inline(always)]
    pub fn is_addr_dio(&self) -> bool {
        *self == Admod::AddrDio
    }
    #[doc = "Address on four lines"]
    #[inline(always)]
    pub fn is_addr_qio(&self) -> bool {
        *self == Admod::AddrQio
    }
}
#[doc = "Field `ADMOD` writer - Address mode"]
pub type AdmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Admod, crate::Safe>;
impl<'a, REG> AdmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No address"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Admod::No)
    }
    #[doc = "Address on a single line"]
    #[inline(always)]
    pub fn addr_sio(self) -> &'a mut crate::W<REG> {
        self.variant(Admod::AddrSio)
    }
    #[doc = "Address on two lines"]
    #[inline(always)]
    pub fn addr_dio(self) -> &'a mut crate::W<REG> {
        self.variant(Admod::AddrDio)
    }
    #[doc = "Address on four lines"]
    #[inline(always)]
    pub fn addr_qio(self) -> &'a mut crate::W<REG> {
        self.variant(Admod::AddrQio)
    }
}
#[doc = "Address size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adsiz {
    #[doc = "0: 8-bit address"]
    AddrSize8 = 0,
    #[doc = "1: 16-bit address"]
    AddrSize16 = 1,
    #[doc = "2: 24-bit address"]
    AddrSize24 = 2,
    #[doc = "3: 32-bit address"]
    AddrSize32 = 3,
}
impl From<Adsiz> for u8 {
    #[inline(always)]
    fn from(variant: Adsiz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adsiz {
    type Ux = u8;
}
impl crate::IsEnum for Adsiz {}
#[doc = "Field `ADSIZ` reader - Address size"]
pub type AdsizR = crate::FieldReader<Adsiz>;
impl AdsizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adsiz {
        match self.bits {
            0 => Adsiz::AddrSize8,
            1 => Adsiz::AddrSize16,
            2 => Adsiz::AddrSize24,
            3 => Adsiz::AddrSize32,
            _ => unreachable!(),
        }
    }
    #[doc = "8-bit address"]
    #[inline(always)]
    pub fn is_addr_size_8(&self) -> bool {
        *self == Adsiz::AddrSize8
    }
    #[doc = "16-bit address"]
    #[inline(always)]
    pub fn is_addr_size_16(&self) -> bool {
        *self == Adsiz::AddrSize16
    }
    #[doc = "24-bit address"]
    #[inline(always)]
    pub fn is_addr_size_24(&self) -> bool {
        *self == Adsiz::AddrSize24
    }
    #[doc = "32-bit address"]
    #[inline(always)]
    pub fn is_addr_size_32(&self) -> bool {
        *self == Adsiz::AddrSize32
    }
}
#[doc = "Field `ADSIZ` writer - Address size"]
pub type AdsizW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adsiz, crate::Safe>;
impl<'a, REG> AdsizW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit address"]
    #[inline(always)]
    pub fn addr_size_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adsiz::AddrSize8)
    }
    #[doc = "16-bit address"]
    #[inline(always)]
    pub fn addr_size_16(self) -> &'a mut crate::W<REG> {
        self.variant(Adsiz::AddrSize16)
    }
    #[doc = "24-bit address"]
    #[inline(always)]
    pub fn addr_size_24(self) -> &'a mut crate::W<REG> {
        self.variant(Adsiz::AddrSize24)
    }
    #[doc = "32-bit address"]
    #[inline(always)]
    pub fn addr_size_32(self) -> &'a mut crate::W<REG> {
        self.variant(Adsiz::AddrSize32)
    }
}
#[doc = "Alternate bytes mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Abmod {
    #[doc = "0: No alternate bytes"]
    AltBytesNo = 0,
    #[doc = "1: Alternate bytes on a single line"]
    AltBytesSio = 1,
    #[doc = "2: Alternate bytes on two lines"]
    AltBytesDio = 2,
    #[doc = "3: Alternate bytes on four lines"]
    AltBytesQio = 3,
}
impl From<Abmod> for u8 {
    #[inline(always)]
    fn from(variant: Abmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Abmod {
    type Ux = u8;
}
impl crate::IsEnum for Abmod {}
#[doc = "Field `ABMOD` reader - Alternate bytes mode"]
pub type AbmodR = crate::FieldReader<Abmod>;
impl AbmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abmod {
        match self.bits {
            0 => Abmod::AltBytesNo,
            1 => Abmod::AltBytesSio,
            2 => Abmod::AltBytesDio,
            3 => Abmod::AltBytesQio,
            _ => unreachable!(),
        }
    }
    #[doc = "No alternate bytes"]
    #[inline(always)]
    pub fn is_alt_bytes_no(&self) -> bool {
        *self == Abmod::AltBytesNo
    }
    #[doc = "Alternate bytes on a single line"]
    #[inline(always)]
    pub fn is_alt_bytes_sio(&self) -> bool {
        *self == Abmod::AltBytesSio
    }
    #[doc = "Alternate bytes on two lines"]
    #[inline(always)]
    pub fn is_alt_bytes_dio(&self) -> bool {
        *self == Abmod::AltBytesDio
    }
    #[doc = "Alternate bytes on four lines"]
    #[inline(always)]
    pub fn is_alt_bytes_qio(&self) -> bool {
        *self == Abmod::AltBytesQio
    }
}
#[doc = "Field `ABMOD` writer - Alternate bytes mode"]
pub type AbmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Abmod, crate::Safe>;
impl<'a, REG> AbmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No alternate bytes"]
    #[inline(always)]
    pub fn alt_bytes_no(self) -> &'a mut crate::W<REG> {
        self.variant(Abmod::AltBytesNo)
    }
    #[doc = "Alternate bytes on a single line"]
    #[inline(always)]
    pub fn alt_bytes_sio(self) -> &'a mut crate::W<REG> {
        self.variant(Abmod::AltBytesSio)
    }
    #[doc = "Alternate bytes on two lines"]
    #[inline(always)]
    pub fn alt_bytes_dio(self) -> &'a mut crate::W<REG> {
        self.variant(Abmod::AltBytesDio)
    }
    #[doc = "Alternate bytes on four lines"]
    #[inline(always)]
    pub fn alt_bytes_qio(self) -> &'a mut crate::W<REG> {
        self.variant(Abmod::AltBytesQio)
    }
}
#[doc = "Alternate bytes size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Absiz {
    #[doc = "0: 8-bit alternate byte"]
    AltBytesSize8 = 0,
    #[doc = "1: 16-bit alternate byte"]
    AltBytesSize16 = 1,
    #[doc = "2: 24-bit alternate byte"]
    AltBytesSize24 = 2,
    #[doc = "3: 32-bit alternate byte"]
    AltBytesSize32 = 3,
}
impl From<Absiz> for u8 {
    #[inline(always)]
    fn from(variant: Absiz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Absiz {
    type Ux = u8;
}
impl crate::IsEnum for Absiz {}
#[doc = "Field `ABSIZ` reader - Alternate bytes size"]
pub type AbsizR = crate::FieldReader<Absiz>;
impl AbsizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Absiz {
        match self.bits {
            0 => Absiz::AltBytesSize8,
            1 => Absiz::AltBytesSize16,
            2 => Absiz::AltBytesSize24,
            3 => Absiz::AltBytesSize32,
            _ => unreachable!(),
        }
    }
    #[doc = "8-bit alternate byte"]
    #[inline(always)]
    pub fn is_alt_bytes_size_8(&self) -> bool {
        *self == Absiz::AltBytesSize8
    }
    #[doc = "16-bit alternate byte"]
    #[inline(always)]
    pub fn is_alt_bytes_size_16(&self) -> bool {
        *self == Absiz::AltBytesSize16
    }
    #[doc = "24-bit alternate byte"]
    #[inline(always)]
    pub fn is_alt_bytes_size_24(&self) -> bool {
        *self == Absiz::AltBytesSize24
    }
    #[doc = "32-bit alternate byte"]
    #[inline(always)]
    pub fn is_alt_bytes_size_32(&self) -> bool {
        *self == Absiz::AltBytesSize32
    }
}
#[doc = "Field `ABSIZ` writer - Alternate bytes size"]
pub type AbsizW<'a, REG> = crate::FieldWriter<'a, REG, 2, Absiz, crate::Safe>;
impl<'a, REG> AbsizW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit alternate byte"]
    #[inline(always)]
    pub fn alt_bytes_size_8(self) -> &'a mut crate::W<REG> {
        self.variant(Absiz::AltBytesSize8)
    }
    #[doc = "16-bit alternate byte"]
    #[inline(always)]
    pub fn alt_bytes_size_16(self) -> &'a mut crate::W<REG> {
        self.variant(Absiz::AltBytesSize16)
    }
    #[doc = "24-bit alternate byte"]
    #[inline(always)]
    pub fn alt_bytes_size_24(self) -> &'a mut crate::W<REG> {
        self.variant(Absiz::AltBytesSize24)
    }
    #[doc = "32-bit alternate byte"]
    #[inline(always)]
    pub fn alt_bytes_size_32(self) -> &'a mut crate::W<REG> {
        self.variant(Absiz::AltBytesSize32)
    }
}
#[doc = "Field `DCYCS` reader - Number of dummy cycles ( This field defines of the dummy phase. ) It specifies a number of CLK cycles ( 0-31 )"]
pub type DcycsR = crate::FieldReader;
#[doc = "Field `DCYCS` writer - Number of dummy cycles ( This field defines of the dummy phase. ) It specifies a number of CLK cycles ( 0-31 )"]
pub type DcycsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Data mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmod {
    #[doc = "0: No data"]
    NoData = 0,
    #[doc = "1: Single lines for serial IO"]
    Single = 1,
    #[doc = "2: Double lines for serial IO"]
    Double = 2,
    #[doc = "3: Quad lines for serial IO"]
    Quad = 3,
}
impl From<Dmod> for u8 {
    #[inline(always)]
    fn from(variant: Dmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmod {
    type Ux = u8;
}
impl crate::IsEnum for Dmod {}
#[doc = "Field `DMOD` reader - Data mode"]
pub type DmodR = crate::FieldReader<Dmod>;
impl DmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmod {
        match self.bits {
            0 => Dmod::NoData,
            1 => Dmod::Single,
            2 => Dmod::Double,
            3 => Dmod::Quad,
            _ => unreachable!(),
        }
    }
    #[doc = "No data"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Dmod::NoData
    }
    #[doc = "Single lines for serial IO"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Dmod::Single
    }
    #[doc = "Double lines for serial IO"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == Dmod::Double
    }
    #[doc = "Quad lines for serial IO"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == Dmod::Quad
    }
}
#[doc = "Field `DMOD` writer - Data mode"]
pub type DmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmod, crate::Safe>;
impl<'a, REG> DmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No data"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::NoData)
    }
    #[doc = "Single lines for serial IO"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::Single)
    }
    #[doc = "Double lines for serial IO"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::Double)
    }
    #[doc = "Quad lines for serial IO"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::Quad)
    }
}
#[doc = "QSPI Data IO Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diod {
    #[doc = "0: Indirect write mode"]
    Wr = 0,
    #[doc = "1: Indirect read mode"]
    Rd = 1,
}
impl From<Diod> for u8 {
    #[inline(always)]
    fn from(variant: Diod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diod {
    type Ux = u8;
}
impl crate::IsEnum for Diod {}
#[doc = "Field `DIOD` reader - QSPI Data IO Direction"]
pub type DiodR = crate::FieldReader<Diod>;
impl DiodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Diod> {
        match self.bits {
            0 => Some(Diod::Wr),
            1 => Some(Diod::Rd),
            _ => None,
        }
    }
    #[doc = "Indirect write mode"]
    #[inline(always)]
    pub fn is_wr(&self) -> bool {
        *self == Diod::Wr
    }
    #[doc = "Indirect read mode"]
    #[inline(always)]
    pub fn is_rd(&self) -> bool {
        *self == Diod::Rd
    }
}
#[doc = "Field `DIOD` writer - QSPI Data IO Direction"]
pub type DiodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Diod>;
impl<'a, REG> DiodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Indirect write mode"]
    #[inline(always)]
    pub fn wr(self) -> &'a mut crate::W<REG> {
        self.variant(Diod::Wr)
    }
    #[doc = "Indirect read mode"]
    #[inline(always)]
    pub fn rd(self) -> &'a mut crate::W<REG> {
        self.variant(Diod::Rd)
    }
}
#[doc = "Field `DDRM` reader - QSPI Double Data Rate Mode: ( 0 - SDR; 1 - DDR ) This bit sets the DDR mode for address, alt bytes, and data phases"]
pub type DdrmR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Instruction to be sent to the external SPI defice"]
    #[inline(always)]
    pub fn inst(&self) -> InstR {
        InstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Instruction mode"]
    #[inline(always)]
    pub fn imod(&self) -> ImodR {
        ImodR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Address mode"]
    #[inline(always)]
    pub fn admod(&self) -> AdmodR {
        AdmodR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline(always)]
    pub fn adsiz(&self) -> AdsizR {
        AdsizR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Alternate bytes mode"]
    #[inline(always)]
    pub fn abmod(&self) -> AbmodR {
        AbmodR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Alternate bytes size"]
    #[inline(always)]
    pub fn absiz(&self) -> AbsizR {
        AbsizR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Number of dummy cycles ( This field defines of the dummy phase. ) It specifies a number of CLK cycles ( 0-31 )"]
    #[inline(always)]
    pub fn dcycs(&self) -> DcycsR {
        DcycsR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - Data mode"]
    #[inline(always)]
    pub fn dmod(&self) -> DmodR {
        DmodR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - QSPI Data IO Direction"]
    #[inline(always)]
    pub fn diod(&self) -> DiodR {
        DiodR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - QSPI Double Data Rate Mode: ( 0 - SDR; 1 - DDR ) This bit sets the DDR mode for address, alt bytes, and data phases"]
    #[inline(always)]
    pub fn ddrm(&self) -> DdrmR {
        DdrmR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction to be sent to the external SPI defice"]
    #[inline(always)]
    pub fn inst(&mut self) -> InstW<'_, QccSpec> {
        InstW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Instruction mode"]
    #[inline(always)]
    pub fn imod(&mut self) -> ImodW<'_, QccSpec> {
        ImodW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Address mode"]
    #[inline(always)]
    pub fn admod(&mut self) -> AdmodW<'_, QccSpec> {
        AdmodW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline(always)]
    pub fn adsiz(&mut self) -> AdsizW<'_, QccSpec> {
        AdsizW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Alternate bytes mode"]
    #[inline(always)]
    pub fn abmod(&mut self) -> AbmodW<'_, QccSpec> {
        AbmodW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Alternate bytes size"]
    #[inline(always)]
    pub fn absiz(&mut self) -> AbsizW<'_, QccSpec> {
        AbsizW::new(self, 16)
    }
    #[doc = "Bits 18:22 - Number of dummy cycles ( This field defines of the dummy phase. ) It specifies a number of CLK cycles ( 0-31 )"]
    #[inline(always)]
    pub fn dcycs(&mut self) -> DcycsW<'_, QccSpec> {
        DcycsW::new(self, 18)
    }
    #[doc = "Bits 24:25 - Data mode"]
    #[inline(always)]
    pub fn dmod(&mut self) -> DmodW<'_, QccSpec> {
        DmodW::new(self, 24)
    }
    #[doc = "Bits 26:27 - QSPI Data IO Direction"]
    #[inline(always)]
    pub fn diod(&mut self) -> DiodW<'_, QccSpec> {
        DiodW::new(self, 26)
    }
}
#[doc = "QSPI Communication Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`qcc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QccSpec;
impl crate::RegisterSpec for QccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qcc::R`](R) reader structure"]
impl crate::Readable for QccSpec {}
#[doc = "`write(|w| ..)` method takes [`qcc::W`](W) writer structure"]
impl crate::Writable for QccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QCC to value 0"]
impl crate::Resettable for QccSpec {}
