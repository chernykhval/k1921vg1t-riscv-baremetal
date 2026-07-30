#[doc = "Register `HRCTL` reader"]
pub type R = crate::R<HrctlSpec>;
#[doc = "Register `HRCTL` writer"]
pub type W = crate::W<HrctlSpec>;
#[doc = "Edge selecting for delay on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edgmodea {
    #[doc = "1: posedge will be delayed"]
    PosEdge = 1,
    #[doc = "2: negedge will be delayed"]
    NegEdge = 2,
    #[doc = "3: posedge and negedge will be delayed"]
    BothEdge = 3,
}
impl From<Edgmodea> for u8 {
    #[inline(always)]
    fn from(variant: Edgmodea) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edgmodea {
    type Ux = u8;
}
impl crate::IsEnum for Edgmodea {}
#[doc = "Field `EDGMODEA` reader - Edge selecting for delay on channel A"]
pub type EdgmodeaR = crate::FieldReader<Edgmodea>;
impl EdgmodeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Edgmodea> {
        match self.bits {
            1 => Some(Edgmodea::PosEdge),
            2 => Some(Edgmodea::NegEdge),
            3 => Some(Edgmodea::BothEdge),
            _ => None,
        }
    }
    #[doc = "posedge will be delayed"]
    #[inline(always)]
    pub fn is_pos_edge(&self) -> bool {
        *self == Edgmodea::PosEdge
    }
    #[doc = "negedge will be delayed"]
    #[inline(always)]
    pub fn is_neg_edge(&self) -> bool {
        *self == Edgmodea::NegEdge
    }
    #[doc = "posedge and negedge will be delayed"]
    #[inline(always)]
    pub fn is_both_edge(&self) -> bool {
        *self == Edgmodea::BothEdge
    }
}
#[doc = "Field `EDGMODEA` writer - Edge selecting for delay on channel A"]
pub type EdgmodeaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edgmodea>;
impl<'a, REG> EdgmodeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "posedge will be delayed"]
    #[inline(always)]
    pub fn pos_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgmodea::PosEdge)
    }
    #[doc = "negedge will be delayed"]
    #[inline(always)]
    pub fn neg_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgmodea::NegEdge)
    }
    #[doc = "posedge and negedge will be delayed"]
    #[inline(always)]
    pub fn both_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgmodea::BothEdge)
    }
}
#[doc = "Register specifies the delay select on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctlmodea {
    #[doc = "0: delay source in CMPA reg"]
    Cmpahr = 0,
    #[doc = "1: delay source in TBPHS reg"]
    Tbphshr = 1,
}
impl From<Ctlmodea> for bool {
    #[inline(always)]
    fn from(variant: Ctlmodea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTLMODEA` reader - Register specifies the delay select on channel A"]
pub type CtlmodeaR = crate::BitReader<Ctlmodea>;
impl CtlmodeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctlmodea {
        match self.bits {
            false => Ctlmodea::Cmpahr,
            true => Ctlmodea::Tbphshr,
        }
    }
    #[doc = "delay source in CMPA reg"]
    #[inline(always)]
    pub fn is_cmpahr(&self) -> bool {
        *self == Ctlmodea::Cmpahr
    }
    #[doc = "delay source in TBPHS reg"]
    #[inline(always)]
    pub fn is_tbphshr(&self) -> bool {
        *self == Ctlmodea::Tbphshr
    }
}
#[doc = "Field `CTLMODEA` writer - Register specifies the delay select on channel A"]
pub type CtlmodeaW<'a, REG> = crate::BitWriter<'a, REG, Ctlmodea>;
impl<'a, REG> CtlmodeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "delay source in CMPA reg"]
    #[inline(always)]
    pub fn cmpahr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctlmodea::Cmpahr)
    }
    #[doc = "delay source in TBPHS reg"]
    #[inline(always)]
    pub fn tbphshr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctlmodea::Tbphshr)
    }
}
#[doc = "CMPAHR register shadow load mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrload {
    #[doc = "0: load when CTR = 0"]
    CtreqZero = 0,
    #[doc = "1: load when CTR = PRD"]
    CtreqPrd = 1,
}
impl From<Hrload> for bool {
    #[inline(always)]
    fn from(variant: Hrload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRLOAD` reader - CMPAHR register shadow load mode"]
pub type HrloadR = crate::BitReader<Hrload>;
impl HrloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrload {
        match self.bits {
            false => Hrload::CtreqZero,
            true => Hrload::CtreqPrd,
        }
    }
    #[doc = "load when CTR = 0"]
    #[inline(always)]
    pub fn is_ctreq_zero(&self) -> bool {
        *self == Hrload::CtreqZero
    }
    #[doc = "load when CTR = PRD"]
    #[inline(always)]
    pub fn is_ctreq_prd(&self) -> bool {
        *self == Hrload::CtreqPrd
    }
}
#[doc = "Field `HRLOAD` writer - CMPAHR register shadow load mode"]
pub type HrloadW<'a, REG> = crate::BitWriter<'a, REG, Hrload>;
impl<'a, REG> HrloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "load when CTR = 0"]
    #[inline(always)]
    pub fn ctreq_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Hrload::CtreqZero)
    }
    #[doc = "load when CTR = PRD"]
    #[inline(always)]
    pub fn ctreq_prd(self) -> &'a mut crate::W<REG> {
        self.variant(Hrload::CtreqPrd)
    }
}
#[doc = "Edge selecting for delay on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edgmodeb {
    #[doc = "1: posedge will be delayed"]
    PosEdge = 1,
    #[doc = "2: negedge will be delayed"]
    NegEdge = 2,
    #[doc = "3: posedge and negedge will be delayed"]
    BothEdge = 3,
}
impl From<Edgmodeb> for u8 {
    #[inline(always)]
    fn from(variant: Edgmodeb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edgmodeb {
    type Ux = u8;
}
impl crate::IsEnum for Edgmodeb {}
#[doc = "Field `EDGMODEB` reader - Edge selecting for delay on channel B"]
pub type EdgmodebR = crate::FieldReader<Edgmodeb>;
impl EdgmodebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Edgmodeb> {
        match self.bits {
            1 => Some(Edgmodeb::PosEdge),
            2 => Some(Edgmodeb::NegEdge),
            3 => Some(Edgmodeb::BothEdge),
            _ => None,
        }
    }
    #[doc = "posedge will be delayed"]
    #[inline(always)]
    pub fn is_pos_edge(&self) -> bool {
        *self == Edgmodeb::PosEdge
    }
    #[doc = "negedge will be delayed"]
    #[inline(always)]
    pub fn is_neg_edge(&self) -> bool {
        *self == Edgmodeb::NegEdge
    }
    #[doc = "posedge and negedge will be delayed"]
    #[inline(always)]
    pub fn is_both_edge(&self) -> bool {
        *self == Edgmodeb::BothEdge
    }
}
#[doc = "Field `EDGMODEB` writer - Edge selecting for delay on channel B"]
pub type EdgmodebW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edgmodeb>;
impl<'a, REG> EdgmodebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "posedge will be delayed"]
    #[inline(always)]
    pub fn pos_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgmodeb::PosEdge)
    }
    #[doc = "negedge will be delayed"]
    #[inline(always)]
    pub fn neg_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgmodeb::NegEdge)
    }
    #[doc = "posedge and negedge will be delayed"]
    #[inline(always)]
    pub fn both_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Edgmodeb::BothEdge)
    }
}
#[doc = "Register specifies the delay select on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctlmodeb {
    #[doc = "0: delay source in CMPA reg"]
    Cmpahr = 0,
    #[doc = "1: delay source in TBPHS reg"]
    Tbphshr = 1,
}
impl From<Ctlmodeb> for bool {
    #[inline(always)]
    fn from(variant: Ctlmodeb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTLMODEB` reader - Register specifies the delay select on channel B"]
pub type CtlmodebR = crate::BitReader<Ctlmodeb>;
impl CtlmodebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctlmodeb {
        match self.bits {
            false => Ctlmodeb::Cmpahr,
            true => Ctlmodeb::Tbphshr,
        }
    }
    #[doc = "delay source in CMPA reg"]
    #[inline(always)]
    pub fn is_cmpahr(&self) -> bool {
        *self == Ctlmodeb::Cmpahr
    }
    #[doc = "delay source in TBPHS reg"]
    #[inline(always)]
    pub fn is_tbphshr(&self) -> bool {
        *self == Ctlmodeb::Tbphshr
    }
}
#[doc = "Field `CTLMODEB` writer - Register specifies the delay select on channel B"]
pub type CtlmodebW<'a, REG> = crate::BitWriter<'a, REG, Ctlmodeb>;
impl<'a, REG> CtlmodebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "delay source in CMPA reg"]
    #[inline(always)]
    pub fn cmpahr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctlmodeb::Cmpahr)
    }
    #[doc = "delay source in TBPHS reg"]
    #[inline(always)]
    pub fn tbphshr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctlmodeb::Tbphshr)
    }
}
#[doc = "Field `DELAYCALA` reader - Delay calibration flags of channel A"]
pub type DelaycalaR = crate::FieldReader;
#[doc = "Field `DELAYCALA` writer - Delay calibration flags of channel A"]
pub type DelaycalaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELAYCALB` reader - Delay calibration flags of channel B"]
pub type DelaycalbR = crate::FieldReader;
#[doc = "Field `DELAYCALB` writer - Delay calibration flags of channel B"]
pub type DelaycalbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Edge selecting for delay on channel A"]
    #[inline(always)]
    pub fn edgmodea(&self) -> EdgmodeaR {
        EdgmodeaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Register specifies the delay select on channel A"]
    #[inline(always)]
    pub fn ctlmodea(&self) -> CtlmodeaR {
        CtlmodeaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMPAHR register shadow load mode"]
    #[inline(always)]
    pub fn hrload(&self) -> HrloadR {
        HrloadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Edge selecting for delay on channel B"]
    #[inline(always)]
    pub fn edgmodeb(&self) -> EdgmodebR {
        EdgmodebR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Register specifies the delay select on channel B"]
    #[inline(always)]
    pub fn ctlmodeb(&self) -> CtlmodebR {
        CtlmodebR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Delay calibration flags of channel A"]
    #[inline(always)]
    pub fn delaycala(&self) -> DelaycalaR {
        DelaycalaR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Delay calibration flags of channel B"]
    #[inline(always)]
    pub fn delaycalb(&self) -> DelaycalbR {
        DelaycalbR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Edge selecting for delay on channel A"]
    #[inline(always)]
    pub fn edgmodea(&mut self) -> EdgmodeaW<'_, HrctlSpec> {
        EdgmodeaW::new(self, 0)
    }
    #[doc = "Bit 2 - Register specifies the delay select on channel A"]
    #[inline(always)]
    pub fn ctlmodea(&mut self) -> CtlmodeaW<'_, HrctlSpec> {
        CtlmodeaW::new(self, 2)
    }
    #[doc = "Bit 3 - CMPAHR register shadow load mode"]
    #[inline(always)]
    pub fn hrload(&mut self) -> HrloadW<'_, HrctlSpec> {
        HrloadW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Edge selecting for delay on channel B"]
    #[inline(always)]
    pub fn edgmodeb(&mut self) -> EdgmodebW<'_, HrctlSpec> {
        EdgmodebW::new(self, 4)
    }
    #[doc = "Bit 6 - Register specifies the delay select on channel B"]
    #[inline(always)]
    pub fn ctlmodeb(&mut self) -> CtlmodebW<'_, HrctlSpec> {
        CtlmodebW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Delay calibration flags of channel A"]
    #[inline(always)]
    pub fn delaycala(&mut self) -> DelaycalaW<'_, HrctlSpec> {
        DelaycalaW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Delay calibration flags of channel B"]
    #[inline(always)]
    pub fn delaycalb(&mut self) -> DelaycalbW<'_, HrctlSpec> {
        DelaycalbW::new(self, 10)
    }
}
#[doc = "High-Resolution Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrctlSpec;
impl crate::RegisterSpec for HrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrctl::R`](R) reader structure"]
impl crate::Readable for HrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hrctl::W`](W) writer structure"]
impl crate::Writable for HrctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HRCTL to value 0"]
impl crate::Resettable for HrctlSpec {}
