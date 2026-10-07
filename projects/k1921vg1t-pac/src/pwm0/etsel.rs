#[doc = "Register `ETSEL` reader"]
pub type R = crate::R<EtselSpec>;
#[doc = "Register `ETSEL` writer"]
pub type W = crate::W<EtselSpec>;
#[doc = "PWM_INT interrupt selection options\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intsel {
    #[doc = "0: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    CtreqCmpa = 0,
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "3: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    CtreqCmpb = 3,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Intsel> for u8 {
    #[inline(always)]
    fn from(variant: Intsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intsel {
    type Ux = u8;
}
impl crate::IsEnum for Intsel {}
#[doc = "Field `INTSEL` reader - PWM_INT interrupt selection options"]
pub type IntselR = crate::FieldReader<Intsel>;
impl IntselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intsel {
        match self.bits {
            0 => Intsel::CtreqCmpa,
            1 => Intsel::CtreqZero,
            2 => Intsel::CtreqPrd,
            3 => Intsel::CtreqCmpb,
            4 => Intsel::CtreqCmpaOnUp,
            5 => Intsel::CtreqCmpaOnDown,
            6 => Intsel::CtreqCmpbOnUp,
            7 => Intsel::CtreqCmpbOnDown,
            _ => unreachable!(),
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn is_ctreq_cmpa(&self) -> bool {
        *self == Intsel::CtreqCmpa
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Intsel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Intsel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn is_ctreq_cmpb(&self) -> bool {
        *self == Intsel::CtreqCmpb
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Intsel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Intsel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Intsel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Intsel::CtreqCmpbOnDown
    }
}
#[doc = "Field `INTSEL` writer - PWM_INT interrupt selection options"]
pub type IntselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Intsel, crate::Safe>;
impl<'a, REG> IntselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn ctreq_cmpa(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpa)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn ctreq_cmpb(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpb)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Intsel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `INTEN` reader - Enable PWM_INT interrupt generation"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Enable PWM_INT interrupt generation"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQASTOP` reader - Automatic clear DMA TX request (PWM A) generation when channel ends all transactions"]
pub type DrqastopR = crate::BitReader;
#[doc = "Field `DRQASTOP` writer - Automatic clear DMA TX request (PWM A) generation when channel ends all transactions"]
pub type DrqastopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQBSTOP` reader - Automatic clear DMA RX request (PWM B) generation when channel ends all transactions"]
pub type DrqbstopR = crate::BitReader;
#[doc = "Field `DRQBSTOP` writer - Automatic clear DMA RX request (PWM B) generation when channel ends all transactions"]
pub type DrqbstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PWM_SOCA selection Options\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Socasel {
    #[doc = "0: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    CtreqCmpa = 0,
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "3: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    CtreqCmpb = 3,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Socasel> for u8 {
    #[inline(always)]
    fn from(variant: Socasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Socasel {
    type Ux = u8;
}
impl crate::IsEnum for Socasel {}
#[doc = "Field `SOCASEL` reader - PWM_SOCA selection Options"]
pub type SocaselR = crate::FieldReader<Socasel>;
impl SocaselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Socasel {
        match self.bits {
            0 => Socasel::CtreqCmpa,
            1 => Socasel::CtreqZero,
            2 => Socasel::CtreqPrd,
            3 => Socasel::CtreqCmpb,
            4 => Socasel::CtreqCmpaOnUp,
            5 => Socasel::CtreqCmpaOnDown,
            6 => Socasel::CtreqCmpbOnUp,
            7 => Socasel::CtreqCmpbOnDown,
            _ => unreachable!(),
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn is_ctreq_cmpa(&self) -> bool {
        *self == Socasel::CtreqCmpa
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Socasel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Socasel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn is_ctreq_cmpb(&self) -> bool {
        *self == Socasel::CtreqCmpb
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Socasel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Socasel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Socasel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Socasel::CtreqCmpbOnDown
    }
}
#[doc = "Field `SOCASEL` writer - PWM_SOCA selection Options"]
pub type SocaselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Socasel, crate::Safe>;
impl<'a, REG> SocaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn ctreq_cmpa(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpa)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn ctreq_cmpb(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpb)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Socasel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `SOCAEN` reader - Enable the ADC start of conversion A PWM_SOCA pulse"]
pub type SocaenR = crate::BitReader;
#[doc = "Field `SOCAEN` writer - Enable the ADC start of conversion A PWM_SOCA pulse"]
pub type SocaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PWM_SOCB selection Options\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Socbsel {
    #[doc = "0: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    CtreqCmpa = 0,
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "3: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    CtreqCmpb = 3,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Socbsel> for u8 {
    #[inline(always)]
    fn from(variant: Socbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Socbsel {
    type Ux = u8;
}
impl crate::IsEnum for Socbsel {}
#[doc = "Field `SOCBSEL` reader - PWM_SOCB selection Options"]
pub type SocbselR = crate::FieldReader<Socbsel>;
impl SocbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Socbsel {
        match self.bits {
            0 => Socbsel::CtreqCmpa,
            1 => Socbsel::CtreqZero,
            2 => Socbsel::CtreqPrd,
            3 => Socbsel::CtreqCmpb,
            4 => Socbsel::CtreqCmpaOnUp,
            5 => Socbsel::CtreqCmpaOnDown,
            6 => Socbsel::CtreqCmpbOnUp,
            7 => Socbsel::CtreqCmpbOnDown,
            _ => unreachable!(),
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn is_ctreq_cmpa(&self) -> bool {
        *self == Socbsel::CtreqCmpa
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Socbsel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Socbsel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn is_ctreq_cmpb(&self) -> bool {
        *self == Socbsel::CtreqCmpb
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Socbsel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Socbsel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Socbsel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Socbsel::CtreqCmpbOnDown
    }
}
#[doc = "Field `SOCBSEL` writer - PWM_SOCB selection Options"]
pub type SocbselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Socbsel, crate::Safe>;
impl<'a, REG> SocbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn ctreq_cmpa(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpa)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn ctreq_cmpb(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpb)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Socbsel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `SOCBEN` reader - Enable the ADC start of conversion B PWM_SOCB pulse"]
pub type SocbenR = crate::BitReader;
#[doc = "Field `SOCBEN` writer - Enable the ADC start of conversion B PWM_SOCB pulse"]
pub type SocbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PWM A DMA request event selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drqasel {
    #[doc = "0: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    CtreqCmpa = 0,
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "3: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    CtreqCmpb = 3,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Drqasel> for u8 {
    #[inline(always)]
    fn from(variant: Drqasel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drqasel {
    type Ux = u8;
}
impl crate::IsEnum for Drqasel {}
#[doc = "Field `DRQASEL` reader - PWM A DMA request event selection"]
pub type DrqaselR = crate::FieldReader<Drqasel>;
impl DrqaselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drqasel {
        match self.bits {
            0 => Drqasel::CtreqCmpa,
            1 => Drqasel::CtreqZero,
            2 => Drqasel::CtreqPrd,
            3 => Drqasel::CtreqCmpb,
            4 => Drqasel::CtreqCmpaOnUp,
            5 => Drqasel::CtreqCmpaOnDown,
            6 => Drqasel::CtreqCmpbOnUp,
            7 => Drqasel::CtreqCmpbOnDown,
            _ => unreachable!(),
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn is_ctreq_cmpa(&self) -> bool {
        *self == Drqasel::CtreqCmpa
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Drqasel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Drqasel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn is_ctreq_cmpb(&self) -> bool {
        *self == Drqasel::CtreqCmpb
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Drqasel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Drqasel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Drqasel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Drqasel::CtreqCmpbOnDown
    }
}
#[doc = "Field `DRQASEL` writer - PWM A DMA request event selection"]
pub type DrqaselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Drqasel, crate::Safe>;
impl<'a, REG> DrqaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn ctreq_cmpa(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpa)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn ctreq_cmpb(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpb)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Drqasel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `DRQAEN` reader - Enable the DMA request from PWM A (TX)"]
pub type DrqaenR = crate::BitReader;
#[doc = "Field `DRQAEN` writer - Enable the DMA request from PWM A (TX)"]
pub type DrqaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PWM B DMA request event selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drqbsel {
    #[doc = "0: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    CtreqCmpa = 0,
    #[doc = "1: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    CtreqZero = 1,
    #[doc = "2: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    CtreqPrd = 2,
    #[doc = "3: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    CtreqCmpb = 3,
    #[doc = "4: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    CtreqCmpaOnUp = 4,
    #[doc = "5: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    CtreqCmpaOnDown = 5,
    #[doc = "6: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    CtreqCmpbOnUp = 6,
    #[doc = "7: generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    CtreqCmpbOnDown = 7,
}
impl From<Drqbsel> for u8 {
    #[inline(always)]
    fn from(variant: Drqbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drqbsel {
    type Ux = u8;
}
impl crate::IsEnum for Drqbsel {}
#[doc = "Field `DRQBSEL` reader - PWM B DMA request event selection"]
pub type DrqbselR = crate::FieldReader<Drqbsel>;
impl DrqbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drqbsel {
        match self.bits {
            0 => Drqbsel::CtreqCmpa,
            1 => Drqbsel::CtreqZero,
            2 => Drqbsel::CtreqPrd,
            3 => Drqbsel::CtreqCmpb,
            4 => Drqbsel::CtreqCmpaOnUp,
            5 => Drqbsel::CtreqCmpaOnDown,
            6 => Drqbsel::CtreqCmpbOnUp,
            7 => Drqbsel::CtreqCmpbOnDown,
            _ => unreachable!(),
        }
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn is_ctreq_cmpa(&self) -> bool {
        *self == Drqbsel::CtreqCmpa
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Drqbsel::CtreqZero
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Drqbsel::CtreqPrd
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn is_ctreq_cmpb(&self) -> bool {
        *self == Drqbsel::CtreqCmpb
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_up(&self) -> bool {
        *self == Drqbsel::CtreqCmpaOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpa_on_down(&self) -> bool {
        *self == Drqbsel::CtreqCmpaOnDown
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_up(&self) -> bool {
        *self == Drqbsel::CtreqCmpbOnUp
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn is_ctreq_cmpb_on_down(&self) -> bool {
        *self == Drqbsel::CtreqCmpbOnDown
    }
}
#[doc = "Field `DRQBSEL` writer - PWM B DMA request event selection"]
pub type DrqbselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Drqbsel, crate::Safe>;
impl<'a, REG> DrqbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA"]
    #[inline(always)]
    pub fn ctreq_cmpa(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpa)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqZero)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqPrd)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB"]
    #[inline(always)]
    pub fn ctreq_cmpb(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpb)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count up"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpaOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPA when count down"]
    #[inline(always)]
    pub fn ctreq_cmpa_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpaOnDown)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count up"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_up(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpbOnUp)
    }
    #[doc = "generate PWM_SOCA/PWM_SOCB/PWN_INT impulse on CTR = CMPB when count down"]
    #[inline(always)]
    pub fn ctreq_cmpb_on_down(self) -> &'a mut crate::W<REG> {
        self.variant(Drqbsel::CtreqCmpbOnDown)
    }
}
#[doc = "Field `DRQBEN` reader - Enable the DMA request from PWM B (RX)"]
pub type DrqbenR = crate::BitReader;
#[doc = "Field `DRQBEN` writer - Enable the DMA request from PWM B (RX)"]
pub type DrqbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - PWM_INT interrupt selection options"]
    #[inline(always)]
    pub fn intsel(&self) -> IntselR {
        IntselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable PWM_INT interrupt generation"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic clear DMA TX request (PWM A) generation when channel ends all transactions"]
    #[inline(always)]
    pub fn drqastop(&self) -> DrqastopR {
        DrqastopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic clear DMA RX request (PWM B) generation when channel ends all transactions"]
    #[inline(always)]
    pub fn drqbstop(&self) -> DrqbstopR {
        DrqbstopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10 - PWM_SOCA selection Options"]
    #[inline(always)]
    pub fn socasel(&self) -> SocaselR {
        SocaselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable the ADC start of conversion A PWM_SOCA pulse"]
    #[inline(always)]
    pub fn socaen(&self) -> SocaenR {
        SocaenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - PWM_SOCB selection Options"]
    #[inline(always)]
    pub fn socbsel(&self) -> SocbselR {
        SocbselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable the ADC start of conversion B PWM_SOCB pulse"]
    #[inline(always)]
    pub fn socben(&self) -> SocbenR {
        SocbenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - PWM A DMA request event selection"]
    #[inline(always)]
    pub fn drqasel(&self) -> DrqaselR {
        DrqaselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Enable the DMA request from PWM A (TX)"]
    #[inline(always)]
    pub fn drqaen(&self) -> DrqaenR {
        DrqaenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - PWM B DMA request event selection"]
    #[inline(always)]
    pub fn drqbsel(&self) -> DrqbselR {
        DrqbselR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Enable the DMA request from PWM B (RX)"]
    #[inline(always)]
    pub fn drqben(&self) -> DrqbenR {
        DrqbenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PWM_INT interrupt selection options"]
    #[inline(always)]
    pub fn intsel(&mut self) -> IntselW<'_, EtselSpec> {
        IntselW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable PWM_INT interrupt generation"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, EtselSpec> {
        IntenW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic clear DMA TX request (PWM A) generation when channel ends all transactions"]
    #[inline(always)]
    pub fn drqastop(&mut self) -> DrqastopW<'_, EtselSpec> {
        DrqastopW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic clear DMA RX request (PWM B) generation when channel ends all transactions"]
    #[inline(always)]
    pub fn drqbstop(&mut self) -> DrqbstopW<'_, EtselSpec> {
        DrqbstopW::new(self, 5)
    }
    #[doc = "Bits 8:10 - PWM_SOCA selection Options"]
    #[inline(always)]
    pub fn socasel(&mut self) -> SocaselW<'_, EtselSpec> {
        SocaselW::new(self, 8)
    }
    #[doc = "Bit 11 - Enable the ADC start of conversion A PWM_SOCA pulse"]
    #[inline(always)]
    pub fn socaen(&mut self) -> SocaenW<'_, EtselSpec> {
        SocaenW::new(self, 11)
    }
    #[doc = "Bits 12:14 - PWM_SOCB selection Options"]
    #[inline(always)]
    pub fn socbsel(&mut self) -> SocbselW<'_, EtselSpec> {
        SocbselW::new(self, 12)
    }
    #[doc = "Bit 15 - Enable the ADC start of conversion B PWM_SOCB pulse"]
    #[inline(always)]
    pub fn socben(&mut self) -> SocbenW<'_, EtselSpec> {
        SocbenW::new(self, 15)
    }
    #[doc = "Bits 16:18 - PWM A DMA request event selection"]
    #[inline(always)]
    pub fn drqasel(&mut self) -> DrqaselW<'_, EtselSpec> {
        DrqaselW::new(self, 16)
    }
    #[doc = "Bit 19 - Enable the DMA request from PWM A (TX)"]
    #[inline(always)]
    pub fn drqaen(&mut self) -> DrqaenW<'_, EtselSpec> {
        DrqaenW::new(self, 19)
    }
    #[doc = "Bits 20:22 - PWM B DMA request event selection"]
    #[inline(always)]
    pub fn drqbsel(&mut self) -> DrqbselW<'_, EtselSpec> {
        DrqbselW::new(self, 20)
    }
    #[doc = "Bit 23 - Enable the DMA request from PWM B (RX)"]
    #[inline(always)]
    pub fn drqben(&mut self) -> DrqbenW<'_, EtselSpec> {
        DrqbenW::new(self, 23)
    }
}
#[doc = "Event-Trigger Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtselSpec;
impl crate::RegisterSpec for EtselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etsel::R`](R) reader structure"]
impl crate::Readable for EtselSpec {}
#[doc = "`write(|w| ..)` method takes [`etsel::W`](W) writer structure"]
impl crate::Writable for EtselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETSEL to value 0"]
impl crate::Resettable for EtselSpec {}
