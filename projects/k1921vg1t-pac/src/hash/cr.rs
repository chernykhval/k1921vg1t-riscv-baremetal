#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `INIT` writer - Initialize message digest calculation"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Hash mode selected"]
    Hash = 0,
    #[doc = "1: HMAC mode selected. LKEY must be set if the key being used is longer than 64 bytes. SAMK option for long key must be set if the keys used for inner/outer hash functions are identical"]
    Hmac = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Mode selection"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Hash,
            true => Mode::Hmac,
        }
    }
    #[doc = "Hash mode selected"]
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == Mode::Hash
    }
    #[doc = "HMAC mode selected. LKEY must be set if the key being used is longer than 64 bytes. SAMK option for long key must be set if the keys used for inner/outer hash functions are identical"]
    #[inline(always)]
    pub fn is_hmac(&self) -> bool {
        *self == Mode::Hmac
    }
}
#[doc = "Field `MODE` writer - Mode selection"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hash mode selected"]
    #[inline(always)]
    pub fn hash(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Hash)
    }
    #[doc = "HMAC mode selected. LKEY must be set if the key being used is longer than 64 bytes. SAMK option for long key must be set if the keys used for inner/outer hash functions are identical"]
    #[inline(always)]
    pub fn hmac(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Hmac)
    }
}
#[doc = "Long key selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lkey {
    #[doc = "0: Short key (≤ 64 bytes)"]
    HmacShortKey = 0,
    #[doc = "1: Long key (> 64 bytes)"]
    HmacLongKey = 1,
}
impl From<Lkey> for bool {
    #[inline(always)]
    fn from(variant: Lkey) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LKEY` reader - Long key selection"]
pub type LkeyR = crate::BitReader<Lkey>;
impl LkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lkey {
        match self.bits {
            false => Lkey::HmacShortKey,
            true => Lkey::HmacLongKey,
        }
    }
    #[doc = "Short key (≤ 64 bytes)"]
    #[inline(always)]
    pub fn is_hmac_short_key(&self) -> bool {
        *self == Lkey::HmacShortKey
    }
    #[doc = "Long key (> 64 bytes)"]
    #[inline(always)]
    pub fn is_hmac_long_key(&self) -> bool {
        *self == Lkey::HmacLongKey
    }
}
#[doc = "Field `LKEY` writer - Long key selection"]
pub type LkeyW<'a, REG> = crate::BitWriter<'a, REG, Lkey>;
impl<'a, REG> LkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short key (≤ 64 bytes)"]
    #[inline(always)]
    pub fn hmac_short_key(self) -> &'a mut crate::W<REG> {
        self.variant(Lkey::HmacShortKey)
    }
    #[doc = "Long key (> 64 bytes)"]
    #[inline(always)]
    pub fn hmac_long_key(self) -> &'a mut crate::W<REG> {
        self.variant(Lkey::HmacLongKey)
    }
}
#[doc = "Field `SAMK` reader - Same key option"]
pub type SamkR = crate::BitReader;
#[doc = "Field `SAMK` writer - Same key option"]
pub type SamkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Datatype selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datatype {
    #[doc = "0: 32-bit data. The data written into HASH_DIN are directly used by the HASH processing, without reordering"]
    Word = 0,
    #[doc = "1: 16-bit data, or half-word. The data written into HASH_DIN are considered as 2 half-words, and are swapped before being used by the HASH processing"]
    Hword = 1,
    #[doc = "2: 8-bit data, or bytes. The data written into HASH_DIN are considered as 4 bytes, and are swapped before being used by the HASH processing"]
    Byte = 2,
    #[doc = "3: bit data, or bit-string. The data written into HASH_DIN are considered as 32bits (1st bit of the sting at position 0), and are swapped before being used by the HASH processing (1st bit of the string at position 31)"]
    Bit = 3,
}
impl From<Datatype> for u8 {
    #[inline(always)]
    fn from(variant: Datatype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datatype {
    type Ux = u8;
}
impl crate::IsEnum for Datatype {}
#[doc = "Field `DATATYPE` reader - Datatype selection"]
pub type DatatypeR = crate::FieldReader<Datatype>;
impl DatatypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datatype {
        match self.bits {
            0 => Datatype::Word,
            1 => Datatype::Hword,
            2 => Datatype::Byte,
            3 => Datatype::Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit data. The data written into HASH_DIN are directly used by the HASH processing, without reordering"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Datatype::Word
    }
    #[doc = "16-bit data, or half-word. The data written into HASH_DIN are considered as 2 half-words, and are swapped before being used by the HASH processing"]
    #[inline(always)]
    pub fn is_hword(&self) -> bool {
        *self == Datatype::Hword
    }
    #[doc = "8-bit data, or bytes. The data written into HASH_DIN are considered as 4 bytes, and are swapped before being used by the HASH processing"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Datatype::Byte
    }
    #[doc = "bit data, or bit-string. The data written into HASH_DIN are considered as 32bits (1st bit of the sting at position 0), and are swapped before being used by the HASH processing (1st bit of the string at position 31)"]
    #[inline(always)]
    pub fn is_bit(&self) -> bool {
        *self == Datatype::Bit
    }
}
#[doc = "Field `DATATYPE` writer - Datatype selection"]
pub type DatatypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datatype, crate::Safe>;
impl<'a, REG> DatatypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit data. The data written into HASH_DIN are directly used by the HASH processing, without reordering"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::Word)
    }
    #[doc = "16-bit data, or half-word. The data written into HASH_DIN are considered as 2 half-words, and are swapped before being used by the HASH processing"]
    #[inline(always)]
    pub fn hword(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::Hword)
    }
    #[doc = "8-bit data, or bytes. The data written into HASH_DIN are considered as 4 bytes, and are swapped before being used by the HASH processing"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::Byte)
    }
    #[doc = "bit data, or bit-string. The data written into HASH_DIN are considered as 32bits (1st bit of the sting at position 0), and are swapped before being used by the HASH processing (1st bit of the string at position 31)"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(Datatype::Bit)
    }
}
#[doc = "Field `DMAE` reader - DMA enable"]
pub type DmaeR = crate::BitReader;
#[doc = "Field `DMAE` writer - DMA enable"]
pub type DmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMAT` reader - Multiple DMA transfers"]
pub type MdmatR = crate::BitReader;
#[doc = "Field `MDMAT` writer - Multiple DMA transfers"]
pub type MdmatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBW` reader - Number of words already pushed"]
pub type NbwR = crate::FieldReader;
#[doc = "Field `DINNE` reader - DIN not empty"]
pub type DinneR = crate::BitReader;
#[doc = "Algorithm selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Algo {
    #[doc = "0: SHA-1 algorithm selected"]
    Sha1 = 0,
    #[doc = "1: MD5 algorithm selected"]
    Md5 = 1,
    #[doc = "2: SHA224 algorithm selected"]
    Sha224 = 2,
    #[doc = "3: SHA256 algorithm selected"]
    Sha256 = 3,
}
impl From<Algo> for u8 {
    #[inline(always)]
    fn from(variant: Algo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Algo {
    type Ux = u8;
}
impl crate::IsEnum for Algo {}
#[doc = "Field `ALGO` reader - Algorithm selection"]
pub type AlgoR = crate::FieldReader<Algo>;
impl AlgoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Algo {
        match self.bits {
            0 => Algo::Sha1,
            1 => Algo::Md5,
            2 => Algo::Sha224,
            3 => Algo::Sha256,
            _ => unreachable!(),
        }
    }
    #[doc = "SHA-1 algorithm selected"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == Algo::Sha1
    }
    #[doc = "MD5 algorithm selected"]
    #[inline(always)]
    pub fn is_md5(&self) -> bool {
        *self == Algo::Md5
    }
    #[doc = "SHA224 algorithm selected"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        *self == Algo::Sha224
    }
    #[doc = "SHA256 algorithm selected"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == Algo::Sha256
    }
}
#[doc = "Field `ALGO` writer - Algorithm selection"]
pub type AlgoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Algo, crate::Safe>;
impl<'a, REG> AlgoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SHA-1 algorithm selected"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut crate::W<REG> {
        self.variant(Algo::Sha1)
    }
    #[doc = "MD5 algorithm selected"]
    #[inline(always)]
    pub fn md5(self) -> &'a mut crate::W<REG> {
        self.variant(Algo::Md5)
    }
    #[doc = "SHA224 algorithm selected"]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut crate::W<REG> {
        self.variant(Algo::Sha224)
    }
    #[doc = "SHA256 algorithm selected"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut crate::W<REG> {
        self.variant(Algo::Sha256)
    }
}
impl R {
    #[doc = "Bit 1 - Mode selection"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Long key selection"]
    #[inline(always)]
    pub fn lkey(&self) -> LkeyR {
        LkeyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Same key option"]
    #[inline(always)]
    pub fn samk(&self) -> SamkR {
        SamkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Datatype selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DatatypeR {
        DatatypeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&self) -> DmaeR {
        DmaeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Multiple DMA transfers"]
    #[inline(always)]
    pub fn mdmat(&self) -> MdmatR {
        MdmatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words already pushed"]
    #[inline(always)]
    pub fn nbw(&self) -> NbwR {
        NbwR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DIN not empty"]
    #[inline(always)]
    pub fn dinne(&self) -> DinneR {
        DinneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Algorithm selection"]
    #[inline(always)]
    pub fn algo(&self) -> AlgoR {
        AlgoR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Initialize message digest calculation"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, CrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - Mode selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CrSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Long key selection"]
    #[inline(always)]
    pub fn lkey(&mut self) -> LkeyW<'_, CrSpec> {
        LkeyW::new(self, 2)
    }
    #[doc = "Bit 3 - Same key option"]
    #[inline(always)]
    pub fn samk(&mut self) -> SamkW<'_, CrSpec> {
        SamkW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Datatype selection"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DatatypeW<'_, CrSpec> {
        DatatypeW::new(self, 4)
    }
    #[doc = "Bit 6 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DmaeW<'_, CrSpec> {
        DmaeW::new(self, 6)
    }
    #[doc = "Bit 7 - Multiple DMA transfers"]
    #[inline(always)]
    pub fn mdmat(&mut self) -> MdmatW<'_, CrSpec> {
        MdmatW::new(self, 7)
    }
    #[doc = "Bits 14:15 - Algorithm selection"]
    #[inline(always)]
    pub fn algo(&mut self) -> AlgoW<'_, CrSpec> {
        AlgoW::new(self, 14)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
