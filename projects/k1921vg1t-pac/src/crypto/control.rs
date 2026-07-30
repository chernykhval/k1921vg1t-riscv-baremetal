#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `UPDATE_KEY` writer - "]
pub type UpdateKeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` writer - "]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    #[doc = "0: Do encryption"]
    Encrypt = 0,
    #[doc = "1: Do decryption"]
    Decrypt = 1,
}
impl From<Direction> for bool {
    #[inline(always)]
    fn from(variant: Direction) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECTION` reader - "]
pub type DirectionR = crate::BitReader<Direction>;
impl DirectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Direction {
        match self.bits {
            false => Direction::Encrypt,
            true => Direction::Decrypt,
        }
    }
    #[doc = "Do encryption"]
    #[inline(always)]
    pub fn is_encrypt(&self) -> bool {
        *self == Direction::Encrypt
    }
    #[doc = "Do decryption"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == Direction::Decrypt
    }
}
#[doc = "Field `DIRECTION` writer - "]
pub type DirectionW<'a, REG> = crate::BitWriter<'a, REG, Direction>;
impl<'a, REG> DirectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do encryption"]
    #[inline(always)]
    pub fn encrypt(self) -> &'a mut crate::W<REG> {
        self.variant(Direction::Encrypt)
    }
    #[doc = "Do decryption"]
    #[inline(always)]
    pub fn decrypt(self) -> &'a mut crate::W<REG> {
        self.variant(Direction::Decrypt)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Algorithm {
    #[doc = "0: AES 128 bit key"]
    Aes128 = 0,
    #[doc = "1: AES 256 bit key"]
    Aes256 = 1,
    #[doc = "3: GOST 34.12-2018 cipher with 128 bit block (Kuznechik)"]
    Kuznechik = 3,
}
impl From<Algorithm> for u8 {
    #[inline(always)]
    fn from(variant: Algorithm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Algorithm {
    type Ux = u8;
}
impl crate::IsEnum for Algorithm {}
#[doc = "Field `ALGORITHM` reader - "]
pub type AlgorithmR = crate::FieldReader<Algorithm>;
impl AlgorithmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Algorithm> {
        match self.bits {
            0 => Some(Algorithm::Aes128),
            1 => Some(Algorithm::Aes256),
            3 => Some(Algorithm::Kuznechik),
            _ => None,
        }
    }
    #[doc = "AES 128 bit key"]
    #[inline(always)]
    pub fn is_aes_128(&self) -> bool {
        *self == Algorithm::Aes128
    }
    #[doc = "AES 256 bit key"]
    #[inline(always)]
    pub fn is_aes_256(&self) -> bool {
        *self == Algorithm::Aes256
    }
    #[doc = "GOST 34.12-2018 cipher with 128 bit block (Kuznechik)"]
    #[inline(always)]
    pub fn is_kuznechik(&self) -> bool {
        *self == Algorithm::Kuznechik
    }
}
#[doc = "Field `ALGORITHM` writer - "]
pub type AlgorithmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Algorithm>;
impl<'a, REG> AlgorithmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AES 128 bit key"]
    #[inline(always)]
    pub fn aes_128(self) -> &'a mut crate::W<REG> {
        self.variant(Algorithm::Aes128)
    }
    #[doc = "AES 256 bit key"]
    #[inline(always)]
    pub fn aes_256(self) -> &'a mut crate::W<REG> {
        self.variant(Algorithm::Aes256)
    }
    #[doc = "GOST 34.12-2018 cipher with 128 bit block (Kuznechik)"]
    #[inline(always)]
    pub fn kuznechik(self) -> &'a mut crate::W<REG> {
        self.variant(Algorithm::Kuznechik)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Electronic Codebook mode (ECB)"]
    Ecb = 0,
    #[doc = "1: Cipher Block Chaining mode (CBC)"]
    Cbc = 1,
    #[doc = "2: Counter mode (CTR)"]
    Ctr = 2,
    #[doc = "3: Galois/Counter mode (GCM)"]
    Gcm = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - "]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Ecb,
            1 => Mode::Cbc,
            2 => Mode::Ctr,
            3 => Mode::Gcm,
            _ => unreachable!(),
        }
    }
    #[doc = "Electronic Codebook mode (ECB)"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == Mode::Ecb
    }
    #[doc = "Cipher Block Chaining mode (CBC)"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == Mode::Cbc
    }
    #[doc = "Counter mode (CTR)"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == Mode::Ctr
    }
    #[doc = "Galois/Counter mode (GCM)"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == Mode::Gcm
    }
}
#[doc = "Field `MODE` writer - "]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Electronic Codebook mode (ECB)"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ecb)
    }
    #[doc = "Cipher Block Chaining mode (CBC)"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Cbc)
    }
    #[doc = "Counter mode (CTR)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ctr)
    }
    #[doc = "Galois/Counter mode (GCM)"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Gcm)
    }
}
#[doc = "Field `SELF_UPDATE` reader - "]
pub type SelfUpdateR = crate::BitReader;
#[doc = "Field `SELF_UPDATE` writer - "]
pub type SelfUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GcmPhase {
    #[doc = "0: GCM initialization phase"]
    GcmInit = 0,
    #[doc = "1: GCM header phase"]
    GcmHeader = 1,
    #[doc = "2: GCM payload phase"]
    GcmPayload = 2,
    #[doc = "3: GCM last block phase"]
    GcmLastBlock = 3,
}
impl From<GcmPhase> for u8 {
    #[inline(always)]
    fn from(variant: GcmPhase) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GcmPhase {
    type Ux = u8;
}
impl crate::IsEnum for GcmPhase {}
#[doc = "Field `GCM_PHASE` reader - "]
pub type GcmPhaseR = crate::FieldReader<GcmPhase>;
impl GcmPhaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GcmPhase {
        match self.bits {
            0 => GcmPhase::GcmInit,
            1 => GcmPhase::GcmHeader,
            2 => GcmPhase::GcmPayload,
            3 => GcmPhase::GcmLastBlock,
            _ => unreachable!(),
        }
    }
    #[doc = "GCM initialization phase"]
    #[inline(always)]
    pub fn is_gcm_init(&self) -> bool {
        *self == GcmPhase::GcmInit
    }
    #[doc = "GCM header phase"]
    #[inline(always)]
    pub fn is_gcm_header(&self) -> bool {
        *self == GcmPhase::GcmHeader
    }
    #[doc = "GCM payload phase"]
    #[inline(always)]
    pub fn is_gcm_payload(&self) -> bool {
        *self == GcmPhase::GcmPayload
    }
    #[doc = "GCM last block phase"]
    #[inline(always)]
    pub fn is_gcm_last_block(&self) -> bool {
        *self == GcmPhase::GcmLastBlock
    }
}
#[doc = "Field `GCM_PHASE` writer - "]
pub type GcmPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 2, GcmPhase, crate::Safe>;
impl<'a, REG> GcmPhaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GCM initialization phase"]
    #[inline(always)]
    pub fn gcm_init(self) -> &'a mut crate::W<REG> {
        self.variant(GcmPhase::GcmInit)
    }
    #[doc = "GCM header phase"]
    #[inline(always)]
    pub fn gcm_header(self) -> &'a mut crate::W<REG> {
        self.variant(GcmPhase::GcmHeader)
    }
    #[doc = "GCM payload phase"]
    #[inline(always)]
    pub fn gcm_payload(self) -> &'a mut crate::W<REG> {
        self.variant(GcmPhase::GcmPayload)
    }
    #[doc = "GCM last block phase"]
    #[inline(always)]
    pub fn gcm_last_block(self) -> &'a mut crate::W<REG> {
        self.variant(GcmPhase::GcmLastBlock)
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn direction(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn algorithm(&self) -> AlgorithmR {
        AlgorithmR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn self_update(&self) -> SelfUpdateR {
        SelfUpdateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn gcm_phase(&self) -> GcmPhaseR {
        GcmPhaseR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn update_key(&mut self) -> UpdateKeyW<'_, ControlSpec> {
        UpdateKeyW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, ControlSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn direction(&mut self) -> DirectionW<'_, ControlSpec> {
        DirectionW::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn algorithm(&mut self) -> AlgorithmW<'_, ControlSpec> {
        AlgorithmW::new(self, 3)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ControlSpec> {
        ModeW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn self_update(&mut self) -> SelfUpdateW<'_, ControlSpec> {
        SelfUpdateW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn gcm_phase(&mut self) -> GcmPhaseW<'_, ControlSpec> {
        GcmPhaseW::new(self, 9)
    }
}
#[doc = "Operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {}
