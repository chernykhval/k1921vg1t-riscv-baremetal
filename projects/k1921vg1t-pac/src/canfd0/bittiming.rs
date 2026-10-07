#[doc = "Register `BITTIMING` reader"]
pub type R = crate::R<BittimingSpec>;
#[doc = "Register `BITTIMING` writer"]
pub type W = crate::W<BittimingSpec>;
#[doc = "Field `BRP` reader - Baud Rate"]
pub type BrpR = crate::FieldReader;
#[doc = "Field `BRP` writer - Baud Rate"]
pub type BrpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PTS` reader - Propagation Time Segment"]
pub type PtsR = crate::FieldReader;
#[doc = "Field `PTS` writer - Propagation Time Segment"]
pub type PtsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SJW` reader - Synchronization Jump Width"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - Synchronization Jump Width"]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMP` reader - Sample Point"]
pub type SmpR = crate::BitReader;
#[doc = "Field `SMP` writer - Sample Point"]
pub type SmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHS1` reader - Phase Segment 1"]
pub type Phs1R = crate::FieldReader;
#[doc = "Field `PHS1` writer - Phase Segment 1"]
pub type Phs1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHS2` reader - Phase Segment 2"]
pub type Phs2R = crate::FieldReader;
#[doc = "Field `PHS2` writer - Phase Segment 2"]
pub type Phs2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Baud Rate"]
    #[inline(always)]
    pub fn brp(&self) -> BrpR {
        BrpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:11 - Propagation Time Segment"]
    #[inline(always)]
    pub fn pts(&self) -> PtsR {
        PtsR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - Sample Point"]
    #[inline(always)]
    pub fn smp(&self) -> SmpR {
        SmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Phase Segment 1"]
    #[inline(always)]
    pub fn phs1(&self) -> Phs1R {
        Phs1R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Phase Segment 2"]
    #[inline(always)]
    pub fn phs2(&self) -> Phs2R {
        Phs2R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate"]
    #[inline(always)]
    pub fn brp(&mut self) -> BrpW<'_, BittimingSpec> {
        BrpW::new(self, 0)
    }
    #[doc = "Bits 9:11 - Propagation Time Segment"]
    #[inline(always)]
    pub fn pts(&mut self) -> PtsW<'_, BittimingSpec> {
        PtsW::new(self, 9)
    }
    #[doc = "Bits 13:14 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SjwW<'_, BittimingSpec> {
        SjwW::new(self, 13)
    }
    #[doc = "Bit 16 - Sample Point"]
    #[inline(always)]
    pub fn smp(&mut self) -> SmpW<'_, BittimingSpec> {
        SmpW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Phase Segment 1"]
    #[inline(always)]
    pub fn phs1(&mut self) -> Phs1W<'_, BittimingSpec> {
        Phs1W::new(self, 17)
    }
    #[doc = "Bits 20:22 - Phase Segment 2"]
    #[inline(always)]
    pub fn phs2(&mut self) -> Phs2W<'_, BittimingSpec> {
        Phs2W::new(self, 20)
    }
}
#[doc = "CAN Bit Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bittiming::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bittiming::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BittimingSpec;
impl crate::RegisterSpec for BittimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bittiming::R`](R) reader structure"]
impl crate::Readable for BittimingSpec {}
#[doc = "`write(|w| ..)` method takes [`bittiming::W`](W) writer structure"]
impl crate::Writable for BittimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BITTIMING to value 0"]
impl crate::Resettable for BittimingSpec {}
