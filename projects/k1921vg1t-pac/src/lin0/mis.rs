#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "Field `FRSI` reader - "]
pub type FrsiR = crate::BitReader;
#[doc = "Field `FRSI` writer - "]
pub type FrsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RVI` reader - "]
pub type RviR = crate::BitReader;
#[doc = "Field `RVI` writer - "]
pub type RviW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRI` reader - "]
pub type TriR = crate::BitReader;
#[doc = "Field `TRI` writer - "]
pub type TriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFSLPI` reader - "]
pub type WfslpiR = crate::BitReader;
#[doc = "Field `WFSLPI` writer - "]
pub type WfslpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTSLPI` reader - "]
pub type GtslpiR = crate::BitReader;
#[doc = "Field `GTSLPI` writer - "]
pub type GtslpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTXFRI` reader - "]
pub type EndtxfriR = crate::BitReader;
#[doc = "Field `ENDTXFRI` writer - "]
pub type EndtxfriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIDRVI` reader - "]
pub type PidrviR = crate::BitReader;
#[doc = "Field `PIDRVI` writer - "]
pub type PidrviW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATARVI` reader - "]
pub type DatarviR = crate::BitReader;
#[doc = "Field `DATARVI` writer - "]
pub type DatarviW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSUMRVI` reader - "]
pub type CsumrviR = crate::BitReader;
#[doc = "Field `CSUMRVI` writer - "]
pub type CsumrviW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLCONFI` reader - "]
pub type ErlconfiR = crate::BitReader;
#[doc = "Field `ERLCONFI` writer - "]
pub type ErlconfiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVCSUMI` reader - "]
pub type InvcsumiR = crate::BitReader;
#[doc = "Field `INVCSUMI` writer - "]
pub type InvcsumiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVPIDI` reader - "]
pub type InvpidiR = crate::BitReader;
#[doc = "Field `INVPIDI` writer - "]
pub type InvpidiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBITMI` reader - "]
pub type StbitmiR = crate::BitReader;
#[doc = "Field `STBITMI` writer - "]
pub type StbitmiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frsi(&self) -> FrsiR {
        FrsiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rvi(&self) -> RviR {
        RviR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tri(&self) -> TriR {
        TriR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wfslpi(&self) -> WfslpiR {
        WfslpiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gtslpi(&self) -> GtslpiR {
        GtslpiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn endtxfri(&self) -> EndtxfriR {
        EndtxfriR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pidrvi(&self) -> PidrviR {
        PidrviR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn datarvi(&self) -> DatarviR {
        DatarviR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn csumrvi(&self) -> CsumrviR {
        CsumrviR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn erlconfi(&self) -> ErlconfiR {
        ErlconfiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn invcsumi(&self) -> InvcsumiR {
        InvcsumiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn invpidi(&self) -> InvpidiR {
        InvpidiR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stbitmi(&self) -> StbitmiR {
        StbitmiR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frsi(&mut self) -> FrsiW<'_, MisSpec> {
        FrsiW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rvi(&mut self) -> RviW<'_, MisSpec> {
        RviW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tri(&mut self) -> TriW<'_, MisSpec> {
        TriW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wfslpi(&mut self) -> WfslpiW<'_, MisSpec> {
        WfslpiW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gtslpi(&mut self) -> GtslpiW<'_, MisSpec> {
        GtslpiW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn endtxfri(&mut self) -> EndtxfriW<'_, MisSpec> {
        EndtxfriW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pidrvi(&mut self) -> PidrviW<'_, MisSpec> {
        PidrviW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn datarvi(&mut self) -> DatarviW<'_, MisSpec> {
        DatarviW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn csumrvi(&mut self) -> CsumrviW<'_, MisSpec> {
        CsumrviW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn erlconfi(&mut self) -> ErlconfiW<'_, MisSpec> {
        ErlconfiW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn invcsumi(&mut self) -> InvcsumiW<'_, MisSpec> {
        InvcsumiW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn invpidi(&mut self) -> InvpidiW<'_, MisSpec> {
        InvpidiW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stbitmi(&mut self) -> StbitmiW<'_, MisSpec> {
        StbitmiW::new(self, 15)
    }
}
#[doc = "Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
