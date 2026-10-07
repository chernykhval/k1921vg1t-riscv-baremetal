#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `BURST_LENGTH` reader - Burst length"]
pub type BurstLengthR = crate::FieldReader;
#[doc = "Field `BURST_LENGTH` writer - Burst length"]
pub type BurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Burst type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BurstType {
    #[doc = "0: Sequential burst type"]
    Sequential = 0,
    #[doc = "1: Interleaved burst type"]
    Interleaved = 1,
}
impl From<BurstType> for bool {
    #[inline(always)]
    fn from(variant: BurstType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST_TYPE` reader - Burst type"]
pub type BurstTypeR = crate::BitReader<BurstType>;
impl BurstTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BurstType {
        match self.bits {
            false => BurstType::Sequential,
            true => BurstType::Interleaved,
        }
    }
    #[doc = "Sequential burst type"]
    #[inline(always)]
    pub fn is_sequential(&self) -> bool {
        *self == BurstType::Sequential
    }
    #[doc = "Interleaved burst type"]
    #[inline(always)]
    pub fn is_interleaved(&self) -> bool {
        *self == BurstType::Interleaved
    }
}
#[doc = "Field `BURST_TYPE` writer - Burst type"]
pub type BurstTypeW<'a, REG> = crate::BitWriter<'a, REG, BurstType>;
impl<'a, REG> BurstTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sequential burst type"]
    #[inline(always)]
    pub fn sequential(self) -> &'a mut crate::W<REG> {
        self.variant(BurstType::Sequential)
    }
    #[doc = "Interleaved burst type"]
    #[inline(always)]
    pub fn interleaved(self) -> &'a mut crate::W<REG> {
        self.variant(BurstType::Interleaved)
    }
}
#[doc = "Field `CAS_LATENCY` reader - SDRAM CAS latency, specified in clocks"]
pub type CasLatencyR = crate::FieldReader;
#[doc = "Field `CAS_LATENCY` writer - SDRAM CAS latency, specified in clocks"]
pub type CasLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opmode {
    #[doc = "0: Normal operating mode"]
    Normal = 0,
    #[doc = "1: Reserved"]
    Reserv1 = 1,
    #[doc = "2: Reserved"]
    Reserv2 = 2,
    #[doc = "3: Reserved"]
    Reserv3 = 3,
}
impl From<Opmode> for u8 {
    #[inline(always)]
    fn from(variant: Opmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opmode {
    type Ux = u8;
}
impl crate::IsEnum for Opmode {}
#[doc = "Field `OPMODE` reader - Operating mode"]
pub type OpmodeR = crate::FieldReader<Opmode>;
impl OpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opmode {
        match self.bits {
            0 => Opmode::Normal,
            1 => Opmode::Reserv1,
            2 => Opmode::Reserv2,
            3 => Opmode::Reserv3,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal operating mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Opmode::Normal
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserv1(&self) -> bool {
        *self == Opmode::Reserv1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserv2(&self) -> bool {
        *self == Opmode::Reserv2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_reserv3(&self) -> bool {
        *self == Opmode::Reserv3
    }
}
#[doc = "Field `OPMODE` writer - Operating mode"]
pub type OpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Opmode, crate::Safe>;
impl<'a, REG> OpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal operating mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::Normal)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserv1(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::Reserv1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserv2(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::Reserv2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn reserv3(self) -> &'a mut crate::W<REG> {
        self.variant(Opmode::Reserv3)
    }
}
#[doc = "Writing burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wbmode {
    #[doc = "0: Programmed burst length"]
    Pbl = 0,
    #[doc = "1: Single location address"]
    Sla = 1,
}
impl From<Wbmode> for bool {
    #[inline(always)]
    fn from(variant: Wbmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WBMODE` reader - Writing burst mode"]
pub type WbmodeR = crate::BitReader<Wbmode>;
impl WbmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wbmode {
        match self.bits {
            false => Wbmode::Pbl,
            true => Wbmode::Sla,
        }
    }
    #[doc = "Programmed burst length"]
    #[inline(always)]
    pub fn is_pbl(&self) -> bool {
        *self == Wbmode::Pbl
    }
    #[doc = "Single location address"]
    #[inline(always)]
    pub fn is_sla(&self) -> bool {
        *self == Wbmode::Sla
    }
}
#[doc = "Field `WBMODE` writer - Writing burst mode"]
pub type WbmodeW<'a, REG> = crate::BitWriter<'a, REG, Wbmode>;
impl<'a, REG> WbmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmed burst length"]
    #[inline(always)]
    pub fn pbl(self) -> &'a mut crate::W<REG> {
        self.variant(Wbmode::Pbl)
    }
    #[doc = "Single location address"]
    #[inline(always)]
    pub fn sla(self) -> &'a mut crate::W<REG> {
        self.variant(Wbmode::Sla)
    }
}
impl R {
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    pub fn burst_length(&self) -> BurstLengthR {
        BurstLengthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    pub fn burst_type(&self) -> BurstTypeR {
        BurstTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SDRAM CAS latency, specified in clocks"]
    #[inline(always)]
    pub fn cas_latency(&self) -> CasLatencyR {
        CasLatencyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Operating mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Writing burst mode"]
    #[inline(always)]
    pub fn wbmode(&self) -> WbmodeR {
        WbmodeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    pub fn burst_length(&mut self) -> BurstLengthW<'_, ModeSpec> {
        BurstLengthW::new(self, 0)
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    pub fn burst_type(&mut self) -> BurstTypeW<'_, ModeSpec> {
        BurstTypeW::new(self, 3)
    }
    #[doc = "Bits 4:6 - SDRAM CAS latency, specified in clocks"]
    #[inline(always)]
    pub fn cas_latency(&mut self) -> CasLatencyW<'_, ModeSpec> {
        CasLatencyW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Operating mode"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OpmodeW<'_, ModeSpec> {
        OpmodeW::new(self, 7)
    }
    #[doc = "Bit 9 - Writing burst mode"]
    #[inline(always)]
    pub fn wbmode(&mut self) -> WbmodeW<'_, ModeSpec> {
        WbmodeW::new(self, 9)
    }
}
#[doc = "SDRAM Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0x33"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0x33;
}
