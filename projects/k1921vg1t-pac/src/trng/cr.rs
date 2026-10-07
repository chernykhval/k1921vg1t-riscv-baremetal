#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `START` reader - Start/enable the NDRNG"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start/enable the NDRNG"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfsr {
    #[doc = "0: NDRNG with asynchronous free running oscillators"]
    AsyncFreeRunOsc = 0,
    #[doc = "1: Pseudo-Random generator with synchronous oscillators for simulation purpose"]
    SyncOsc = 1,
}
impl From<Lfsr> for bool {
    #[inline(always)]
    fn from(variant: Lfsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFSR` reader - Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators"]
pub type LfsrR = crate::BitReader<Lfsr>;
impl LfsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfsr {
        match self.bits {
            false => Lfsr::AsyncFreeRunOsc,
            true => Lfsr::SyncOsc,
        }
    }
    #[doc = "NDRNG with asynchronous free running oscillators"]
    #[inline(always)]
    pub fn is_async_free_run_osc(&self) -> bool {
        *self == Lfsr::AsyncFreeRunOsc
    }
    #[doc = "Pseudo-Random generator with synchronous oscillators for simulation purpose"]
    #[inline(always)]
    pub fn is_sync_osc(&self) -> bool {
        *self == Lfsr::SyncOsc
    }
}
#[doc = "Field `LFSR` writer - Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators"]
pub type LfsrW<'a, REG> = crate::BitWriter<'a, REG, Lfsr>;
impl<'a, REG> LfsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NDRNG with asynchronous free running oscillators"]
    #[inline(always)]
    pub fn async_free_run_osc(self) -> &'a mut crate::W<REG> {
        self.variant(Lfsr::AsyncFreeRunOsc)
    }
    #[doc = "Pseudo-Random generator with synchronous oscillators for simulation purpose"]
    #[inline(always)]
    pub fn sync_osc(self) -> &'a mut crate::W<REG> {
        self.variant(Lfsr::SyncOsc)
    }
}
#[doc = "Field `TESTEN` reader - Select input for conditioning function and continuous tests: 0: Noise source (normal mode) 1: Test data register (test mode)"]
pub type TestenR = crate::BitReader;
#[doc = "Field `TESTEN` writer - Select input for conditioning function and continuous tests: 0: Noise source (normal mode) 1: Test data register (test mode)"]
pub type TestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONDBYPASS` reader - 0: Conditioning function is used (normal mode); 1: Conditioning function is bypassed (to observe entropy source directly)"]
pub type CondbypassR = crate::BitReader;
#[doc = "Field `CONDBYPASS` writer - 0: Conditioning function is used (normal mode); 1: Conditioning function is bypassed (to observe entropy source directly)"]
pub type CondbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFAILINTEN` reader - Enable interrupt if any of the health test fails"]
pub type TfailintenR = crate::BitReader;
#[doc = "Field `TFAILINTEN` writer - Enable interrupt if any of the health test fails"]
pub type TfailintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFULLINTEN` reader - Enable interrupt if FIFO is full"]
pub type FfullintenR = crate::BitReader;
#[doc = "Field `FFULLINTEN` writer - Enable interrupt if FIFO is full"]
pub type FfullintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTRST` reader - Datapath content 2ush and control FSM reset: 0: Normal mode; 1: Datapath components - the health tests, the conditioning function and the FIFO - are cleared. This bit is not cleared automatically. Configuration registers preserve their values. Send control FSM to Reset state."]
pub type SoftrstR = crate::BitReader;
#[doc = "Field `SOFTRST` writer - Datapath content 2ush and control FSM reset: 0: Normal mode; 1: Datapath components - the health tests, the conditioning function and the FIFO - are cleared. This bit is not cleared automatically. Configuration registers preserve their values. Send control FSM to Reset state."]
pub type SoftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEROS` reader - Force oscillators to run when FIFO is full"]
pub type ForcerosR = crate::BitReader;
#[doc = "Field `FORCEROS` writer - Force oscillators to run when FIFO is full"]
pub type ForcerosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORHTF` reader - Results of the health tests during start-up and online test do not affect the control FSM state. It also bypass control FSM StartUp phase"]
pub type IgnorhtfR = crate::BitReader;
#[doc = "Field `IGNORHTF` writer - Results of the health tests during start-up and online test do not affect the control FSM state. It also bypass control FSM StartUp phase"]
pub type IgnorhtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTBLOCK` reader - Number of 128-bit blocks used in conditioning (AES-CBC-MAC) post-processing. Zero value is not allowed"]
pub type CountblockR = crate::FieldReader;
#[doc = "Field `COUNTBLOCK` writer - Number of 128-bit blocks used in conditioning (AES-CBC-MAC) post-processing. Zero value is not allowed"]
pub type CountblockW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIFOFILLST` reader - Enable write of the samples in the FIFO during start-up"]
pub type FifofillstR = crate::BitReader;
#[doc = "Field `FIFOFILLST` writer - Enable write of the samples in the FIFO during start-up"]
pub type FifofillstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPTSTDIS` reader - All repetition tests (each share) are disabled via this single bit"]
pub type ReptstdisR = crate::BitReader;
#[doc = "Field `REPTSTDIS` writer - All repetition tests (each share) are disabled via this single bit"]
pub type ReptstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROPTSTDIS` reader - All proportion tests (each share) are disabled via this single bit"]
pub type ProptstdisR = crate::BitReader;
#[doc = "Field `PROPTSTDIS` writer - All proportion tests (each share) are disabled via this single bit"]
pub type ProptstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORRDIS` reader - Disable speci1c delay(s) check in auto-correlation test - same RO: x1: vs. the following sample of the same RO -> (0) delay check 1x: vs. the later sample of the same RO -> (+1) delay check"]
pub type AutocorrdisR = crate::FieldReader;
#[doc = "Field `AUTOCORRDIS` writer - Disable speci1c delay(s) check in auto-correlation test - same RO: x1: vs. the following sample of the same RO -> (0) delay check 1x: vs. the later sample of the same RO -> (+1) delay check"]
pub type AutocorrdisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORRTSTDIS` reader - Disable speci1c delay(s) check in correlation test - different ROs: xx1: vs. the same sample of the other RO -> (0) delay check x1x: vs. the preceding sample of the other RO -> (-1) delay check 1xx: vs. the next sample of the other RO -> (+1) delay check"]
pub type CorrtstdisR = crate::FieldReader;
#[doc = "Field `CORRTSTDIS` writer - Disable speci1c delay(s) check in correlation test - different ROs: xx1: vs. the same sample of the other RO -> (0) delay check x1x: vs. the preceding sample of the other RO -> (-1) delay check 1xx: vs. the next sample of the other RO -> (+1) delay check"]
pub type CorrtstdisW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Select blending method: 00: Concatenation; 01: XOR_LEVEL_1; 10: XOR_LEVEL_2; 11: VON-NEUMANN debiasing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blendmethod {
    #[doc = "0: Blending method Concatenation"]
    Concatenation = 0,
    #[doc = "1: Blending method XOR_LEVEL_1"]
    XorLevel1 = 1,
    #[doc = "2: Blending method XOR_LEVEL_2"]
    XorLevel2 = 2,
    #[doc = "3: Blending method VON_NEUMANN_debiasing"]
    VonNeumannDebiasing = 3,
}
impl From<Blendmethod> for u8 {
    #[inline(always)]
    fn from(variant: Blendmethod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blendmethod {
    type Ux = u8;
}
impl crate::IsEnum for Blendmethod {}
#[doc = "Field `BLENDMETHOD` reader - Select blending method: 00: Concatenation; 01: XOR_LEVEL_1; 10: XOR_LEVEL_2; 11: VON-NEUMANN debiasing"]
pub type BlendmethodR = crate::FieldReader<Blendmethod>;
impl BlendmethodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blendmethod {
        match self.bits {
            0 => Blendmethod::Concatenation,
            1 => Blendmethod::XorLevel1,
            2 => Blendmethod::XorLevel2,
            3 => Blendmethod::VonNeumannDebiasing,
            _ => unreachable!(),
        }
    }
    #[doc = "Blending method Concatenation"]
    #[inline(always)]
    pub fn is_concatenation(&self) -> bool {
        *self == Blendmethod::Concatenation
    }
    #[doc = "Blending method XOR_LEVEL_1"]
    #[inline(always)]
    pub fn is_xor_level_1(&self) -> bool {
        *self == Blendmethod::XorLevel1
    }
    #[doc = "Blending method XOR_LEVEL_2"]
    #[inline(always)]
    pub fn is_xor_level_2(&self) -> bool {
        *self == Blendmethod::XorLevel2
    }
    #[doc = "Blending method VON_NEUMANN_debiasing"]
    #[inline(always)]
    pub fn is_von_neumann_debiasing(&self) -> bool {
        *self == Blendmethod::VonNeumannDebiasing
    }
}
#[doc = "Field `BLENDMETHOD` writer - Select blending method: 00: Concatenation; 01: XOR_LEVEL_1; 10: XOR_LEVEL_2; 11: VON-NEUMANN debiasing"]
pub type BlendmethodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Blendmethod, crate::Safe>;
impl<'a, REG> BlendmethodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blending method Concatenation"]
    #[inline(always)]
    pub fn concatenation(self) -> &'a mut crate::W<REG> {
        self.variant(Blendmethod::Concatenation)
    }
    #[doc = "Blending method XOR_LEVEL_1"]
    #[inline(always)]
    pub fn xor_level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Blendmethod::XorLevel1)
    }
    #[doc = "Blending method XOR_LEVEL_2"]
    #[inline(always)]
    pub fn xor_level_2(self) -> &'a mut crate::W<REG> {
        self.variant(Blendmethod::XorLevel2)
    }
    #[doc = "Blending method VON_NEUMANN_debiasing"]
    #[inline(always)]
    pub fn von_neumann_debiasing(self) -> &'a mut crate::W<REG> {
        self.variant(Blendmethod::VonNeumannDebiasing)
    }
}
impl R {
    #[doc = "Bit 0 - Start/enable the NDRNG"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators"]
    #[inline(always)]
    pub fn lfsr(&self) -> LfsrR {
        LfsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select input for conditioning function and continuous tests: 0: Noise source (normal mode) 1: Test data register (test mode)"]
    #[inline(always)]
    pub fn testen(&self) -> TestenR {
        TestenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Conditioning function is used (normal mode); 1: Conditioning function is bypassed (to observe entropy source directly)"]
    #[inline(always)]
    pub fn condbypass(&self) -> CondbypassR {
        CondbypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable interrupt if any of the health test fails"]
    #[inline(always)]
    pub fn tfailinten(&self) -> TfailintenR {
        TfailintenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable interrupt if FIFO is full"]
    #[inline(always)]
    pub fn ffullinten(&self) -> FfullintenR {
        FfullintenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Datapath content 2ush and control FSM reset: 0: Normal mode; 1: Datapath components - the health tests, the conditioning function and the FIFO - are cleared. This bit is not cleared automatically. Configuration registers preserve their values. Send control FSM to Reset state."]
    #[inline(always)]
    pub fn softrst(&self) -> SoftrstR {
        SoftrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Force oscillators to run when FIFO is full"]
    #[inline(always)]
    pub fn forceros(&self) -> ForcerosR {
        ForcerosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Results of the health tests during start-up and online test do not affect the control FSM state. It also bypass control FSM StartUp phase"]
    #[inline(always)]
    pub fn ignorhtf(&self) -> IgnorhtfR {
        IgnorhtfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of 128-bit blocks used in conditioning (AES-CBC-MAC) post-processing. Zero value is not allowed"]
    #[inline(always)]
    pub fn countblock(&self) -> CountblockR {
        CountblockR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Enable write of the samples in the FIFO during start-up"]
    #[inline(always)]
    pub fn fifofillst(&self) -> FifofillstR {
        FifofillstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - All repetition tests (each share) are disabled via this single bit"]
    #[inline(always)]
    pub fn reptstdis(&self) -> ReptstdisR {
        ReptstdisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - All proportion tests (each share) are disabled via this single bit"]
    #[inline(always)]
    pub fn proptstdis(&self) -> ProptstdisR {
        ProptstdisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - Disable speci1c delay(s) check in auto-correlation test - same RO: x1: vs. the following sample of the same RO -> (0) delay check 1x: vs. the later sample of the same RO -> (+1) delay check"]
    #[inline(always)]
    pub fn autocorrdis(&self) -> AutocorrdisR {
        AutocorrdisR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 27:29 - Disable speci1c delay(s) check in correlation test - different ROs: xx1: vs. the same sample of the other RO -> (0) delay check x1x: vs. the preceding sample of the other RO -> (-1) delay check 1xx: vs. the next sample of the other RO -> (+1) delay check"]
    #[inline(always)]
    pub fn corrtstdis(&self) -> CorrtstdisR {
        CorrtstdisR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Select blending method: 00: Concatenation; 01: XOR_LEVEL_1; 10: XOR_LEVEL_2; 11: VON-NEUMANN debiasing"]
    #[inline(always)]
    pub fn blendmethod(&self) -> BlendmethodR {
        BlendmethodR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start/enable the NDRNG"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Select between the NDRNG with asynchronous free running oscillators (when 0) and the Pseudo-Random generator with synchronous oscillators"]
    #[inline(always)]
    pub fn lfsr(&mut self) -> LfsrW<'_, CrSpec> {
        LfsrW::new(self, 1)
    }
    #[doc = "Bit 2 - Select input for conditioning function and continuous tests: 0: Noise source (normal mode) 1: Test data register (test mode)"]
    #[inline(always)]
    pub fn testen(&mut self) -> TestenW<'_, CrSpec> {
        TestenW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: Conditioning function is used (normal mode); 1: Conditioning function is bypassed (to observe entropy source directly)"]
    #[inline(always)]
    pub fn condbypass(&mut self) -> CondbypassW<'_, CrSpec> {
        CondbypassW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable interrupt if any of the health test fails"]
    #[inline(always)]
    pub fn tfailinten(&mut self) -> TfailintenW<'_, CrSpec> {
        TfailintenW::new(self, 4)
    }
    #[doc = "Bit 7 - Enable interrupt if FIFO is full"]
    #[inline(always)]
    pub fn ffullinten(&mut self) -> FfullintenW<'_, CrSpec> {
        FfullintenW::new(self, 7)
    }
    #[doc = "Bit 8 - Datapath content 2ush and control FSM reset: 0: Normal mode; 1: Datapath components - the health tests, the conditioning function and the FIFO - are cleared. This bit is not cleared automatically. Configuration registers preserve their values. Send control FSM to Reset state."]
    #[inline(always)]
    pub fn softrst(&mut self) -> SoftrstW<'_, CrSpec> {
        SoftrstW::new(self, 8)
    }
    #[doc = "Bit 11 - Force oscillators to run when FIFO is full"]
    #[inline(always)]
    pub fn forceros(&mut self) -> ForcerosW<'_, CrSpec> {
        ForcerosW::new(self, 11)
    }
    #[doc = "Bit 12 - Results of the health tests during start-up and online test do not affect the control FSM state. It also bypass control FSM StartUp phase"]
    #[inline(always)]
    pub fn ignorhtf(&mut self) -> IgnorhtfW<'_, CrSpec> {
        IgnorhtfW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Number of 128-bit blocks used in conditioning (AES-CBC-MAC) post-processing. Zero value is not allowed"]
    #[inline(always)]
    pub fn countblock(&mut self) -> CountblockW<'_, CrSpec> {
        CountblockW::new(self, 16)
    }
    #[doc = "Bit 20 - Enable write of the samples in the FIFO during start-up"]
    #[inline(always)]
    pub fn fifofillst(&mut self) -> FifofillstW<'_, CrSpec> {
        FifofillstW::new(self, 20)
    }
    #[doc = "Bit 21 - All repetition tests (each share) are disabled via this single bit"]
    #[inline(always)]
    pub fn reptstdis(&mut self) -> ReptstdisW<'_, CrSpec> {
        ReptstdisW::new(self, 21)
    }
    #[doc = "Bit 22 - All proportion tests (each share) are disabled via this single bit"]
    #[inline(always)]
    pub fn proptstdis(&mut self) -> ProptstdisW<'_, CrSpec> {
        ProptstdisW::new(self, 22)
    }
    #[doc = "Bits 23:24 - Disable speci1c delay(s) check in auto-correlation test - same RO: x1: vs. the following sample of the same RO -> (0) delay check 1x: vs. the later sample of the same RO -> (+1) delay check"]
    #[inline(always)]
    pub fn autocorrdis(&mut self) -> AutocorrdisW<'_, CrSpec> {
        AutocorrdisW::new(self, 23)
    }
    #[doc = "Bits 27:29 - Disable speci1c delay(s) check in correlation test - different ROs: xx1: vs. the same sample of the other RO -> (0) delay check x1x: vs. the preceding sample of the other RO -> (-1) delay check 1xx: vs. the next sample of the other RO -> (+1) delay check"]
    #[inline(always)]
    pub fn corrtstdis(&mut self) -> CorrtstdisW<'_, CrSpec> {
        CorrtstdisW::new(self, 27)
    }
    #[doc = "Bits 30:31 - Select blending method: 00: Concatenation; 01: XOR_LEVEL_1; 10: XOR_LEVEL_2; 11: VON-NEUMANN debiasing"]
    #[inline(always)]
    pub fn blendmethod(&mut self) -> BlendmethodW<'_, CrSpec> {
        BlendmethodW::new(self, 30)
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
#[doc = "`reset()` method sets CR to value 0x0004_0000"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x0004_0000;
}
