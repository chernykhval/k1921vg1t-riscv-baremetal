#[doc = "Register `NECNT` reader"]
pub type R = crate::R<NecntSpec>;
#[doc = "Register `NECNT` writer"]
pub type W = crate::W<NecntSpec>;
#[doc = "Field `REC` reader - Receive Error Counter field messages"]
pub type RecR = crate::FieldReader;
#[doc = "Field `REC` writer - Receive Error Counter field messages"]
pub type RecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TEC` reader - Counter field messaging error"]
pub type TecR = crate::FieldReader;
#[doc = "Field `TEC` writer - Counter field messaging error"]
pub type TecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EWRNLVL` reader - Error limit at which a flag is set in the register EWRN NSR"]
pub type EwrnlvlR = crate::FieldReader;
#[doc = "Field `EWRNLVL` writer - Error limit at which a flag is set in the register EWRN NSR"]
pub type EwrnlvlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LETD` reader - Flag last transmission errors"]
pub type LetdR = crate::BitReader;
#[doc = "Field `LEINC` reader - Indicator increment at the last error"]
pub type LeincR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Receive Error Counter field messages"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Counter field messaging error"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Error limit at which a flag is set in the register EWRN NSR"]
    #[inline(always)]
    pub fn ewrnlvl(&self) -> EwrnlvlR {
        EwrnlvlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Flag last transmission errors"]
    #[inline(always)]
    pub fn letd(&self) -> LetdR {
        LetdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicator increment at the last error"]
    #[inline(always)]
    pub fn leinc(&self) -> LeincR {
        LeincR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Error Counter field messages"]
    #[inline(always)]
    pub fn rec(&mut self) -> RecW<'_, NecntSpec> {
        RecW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Counter field messaging error"]
    #[inline(always)]
    pub fn tec(&mut self) -> TecW<'_, NecntSpec> {
        TecW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Error limit at which a flag is set in the register EWRN NSR"]
    #[inline(always)]
    pub fn ewrnlvl(&mut self) -> EwrnlvlW<'_, NecntSpec> {
        EwrnlvlW::new(self, 16)
    }
}
#[doc = "Counter error register node0\n\nYou can [`read`](crate::Reg::read) this register and get [`necnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`necnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NecntSpec;
impl crate::RegisterSpec for NecntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`necnt::R`](R) reader structure"]
impl crate::Readable for NecntSpec {}
#[doc = "`write(|w| ..)` method takes [`necnt::W`](W) writer structure"]
impl crate::Writable for NecntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NECNT to value 0x0060_0000"]
impl crate::Resettable for NecntSpec {
    const RESET_VALUE: u32 = 0x0060_0000;
}
