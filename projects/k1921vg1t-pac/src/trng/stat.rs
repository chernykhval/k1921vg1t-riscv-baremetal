#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `DATABUSYTEST` reader - High when data written to TestData register is being processed"]
pub type DatabusytestR = crate::BitReader;
#[doc = "State of the control FSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: State Reset"]
    Reset = 0,
    #[doc = "1: State Startup"]
    StartUp = 1,
    #[doc = "2: State Idle / FIFO full"]
    IdleFifofull = 2,
    #[doc = "4: State Fill FIFO"]
    Fifofill = 4,
    #[doc = "5: State Error"]
    Error = 5,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - State of the control FSM"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            0 => Some(State::Reset),
            1 => Some(State::StartUp),
            2 => Some(State::IdleFifofull),
            4 => Some(State::Fifofill),
            5 => Some(State::Error),
            _ => None,
        }
    }
    #[doc = "State Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == State::Reset
    }
    #[doc = "State Startup"]
    #[inline(always)]
    pub fn is_start_up(&self) -> bool {
        *self == State::StartUp
    }
    #[doc = "State Idle / FIFO full"]
    #[inline(always)]
    pub fn is_idle_fifofull(&self) -> bool {
        *self == State::IdleFifofull
    }
    #[doc = "State Fill FIFO"]
    #[inline(always)]
    pub fn is_fifofill(&self) -> bool {
        *self == State::Fifofill
    }
    #[doc = "State Error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == State::Error
    }
}
#[doc = "Field `REPTESTFAIL` reader - NIST Repetition test(s) failure"]
pub type ReptestfailR = crate::BitReader;
#[doc = "Field `REPTESTFAIL` writer - NIST Repetition test(s) failure"]
pub type ReptestfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROPTESTFAIL` reader - NIST Proportion test(s) failure"]
pub type ProptestfailR = crate::BitReader;
#[doc = "Field `PROPTESTFAIL` writer - NIST Proportion test(s) failure"]
pub type ProptestfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANYTESTFAIL` reader - Any of the enabled health tests is failing"]
pub type AnytestfailR = crate::BitReader;
#[doc = "Field `ANYTESTFAIL` writer - Any of the enabled health tests is failing"]
pub type AnytestfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULL` reader - FIFO full status"]
pub type FifofullR = crate::BitReader;
#[doc = "Field `FIFOFULL` writer - FIFO full status"]
pub type FifofullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTUPFAIL` reader - Start-up test(s) failure"]
pub type StartupfailR = crate::BitReader;
#[doc = "Field `STARTUPFAIL` writer - Start-up test(s) failure"]
pub type StartupfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPTSTFAILSH` reader - NIST Repetition test failure per share"]
pub type ReptstfailshR = crate::FieldReader;
#[doc = "Field `PRTSTFAILSH` reader - NIST Proportion test failure per share"]
pub type PrtstfailshR = crate::FieldReader;
#[doc = "Field `CONDSLOW` reader - Conditioning consumes data slower than they are provided to it"]
pub type CondslowR = crate::BitReader;
#[doc = "Field `CONDSLOW` writer - Conditioning consumes data slower than they are provided to it"]
pub type CondslowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - High when data written to TestData register is being processed"]
    #[inline(always)]
    pub fn databusytest(&self) -> DatabusytestR {
        DatabusytestR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - State of the control FSM"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - NIST Repetition test(s) failure"]
    #[inline(always)]
    pub fn reptestfail(&self) -> ReptestfailR {
        ReptestfailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NIST Proportion test(s) failure"]
    #[inline(always)]
    pub fn proptestfail(&self) -> ProptestfailR {
        ProptestfailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Any of the enabled health tests is failing"]
    #[inline(always)]
    pub fn anytestfail(&self) -> AnytestfailR {
        AnytestfailR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO full status"]
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Start-up test(s) failure"]
    #[inline(always)]
    pub fn startupfail(&self) -> StartupfailR {
        StartupfailR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15 - NIST Repetition test failure per share"]
    #[inline(always)]
    pub fn reptstfailsh(&self) -> ReptstfailshR {
        ReptstfailshR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - NIST Proportion test failure per share"]
    #[inline(always)]
    pub fn prtstfailsh(&self) -> PrtstfailshR {
        PrtstfailshR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Conditioning consumes data slower than they are provided to it"]
    #[inline(always)]
    pub fn condslow(&self) -> CondslowR {
        CondslowR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - NIST Repetition test(s) failure"]
    #[inline(always)]
    pub fn reptestfail(&mut self) -> ReptestfailW<'_, StatSpec> {
        ReptestfailW::new(self, 4)
    }
    #[doc = "Bit 5 - NIST Proportion test(s) failure"]
    #[inline(always)]
    pub fn proptestfail(&mut self) -> ProptestfailW<'_, StatSpec> {
        ProptestfailW::new(self, 5)
    }
    #[doc = "Bit 6 - Any of the enabled health tests is failing"]
    #[inline(always)]
    pub fn anytestfail(&mut self) -> AnytestfailW<'_, StatSpec> {
        AnytestfailW::new(self, 6)
    }
    #[doc = "Bit 7 - FIFO full status"]
    #[inline(always)]
    pub fn fifofull(&mut self) -> FifofullW<'_, StatSpec> {
        FifofullW::new(self, 7)
    }
    #[doc = "Bit 10 - Start-up test(s) failure"]
    #[inline(always)]
    pub fn startupfail(&mut self) -> StartupfailW<'_, StatSpec> {
        StartupfailW::new(self, 10)
    }
    #[doc = "Bit 20 - Conditioning consumes data slower than they are provided to it"]
    #[inline(always)]
    pub fn condslow(&mut self) -> CondslowW<'_, StatSpec> {
        CondslowW::new(self, 20)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
