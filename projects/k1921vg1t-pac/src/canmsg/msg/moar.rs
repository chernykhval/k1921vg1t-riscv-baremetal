#[doc = "Register `MOAR` reader"]
pub type R = crate::R<MoarSpec>;
#[doc = "Register `MOAR` writer"]
pub type W = crate::W<MoarSpec>;
#[doc = "Field `ID` reader - Object ID message <i>"]
pub type IdR = crate::FieldReader<u32>;
#[doc = "Field `ID` writer - Object ID message <i>"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `IDE` reader - Bit extension identifier of the message object <i>"]
pub type IdeR = crate::BitReader;
#[doc = "Field `IDE` writer - Bit extension identifier of the message object <i>"]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRI` reader - Priority class"]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - Priority class"]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:28 - Object ID message <i>"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Bit extension identifier of the message object <i>"]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Priority class"]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:28 - Object ID message <i>"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, MoarSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bit 29 - Bit extension identifier of the message object <i>"]
    #[inline(always)]
    pub fn ide(&mut self) -> IdeW<'_, MoarSpec> {
        IdeW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Priority class"]
    #[inline(always)]
    pub fn pri(&mut self) -> PriW<'_, MoarSpec> {
        PriW::new(self, 30)
    }
}
#[doc = "Register arbitration message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`moar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoarSpec;
impl crate::RegisterSpec for MoarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moar::R`](R) reader structure"]
impl crate::Readable for MoarSpec {}
#[doc = "`write(|w| ..)` method takes [`moar::W`](W) writer structure"]
impl crate::Writable for MoarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOAR to value 0"]
impl crate::Resettable for MoarSpec {}
