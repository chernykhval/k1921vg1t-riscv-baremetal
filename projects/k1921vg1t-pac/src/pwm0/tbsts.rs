#[doc = "Register `TBSTS` reader"]
pub type R = crate::R<TbstsSpec>;
#[doc = "Register `TBSTS` writer"]
pub type W = crate::W<TbstsSpec>;
#[doc = "Field `CTRDIR` reader - Time-Base counter direction status bit"]
pub type CtrdirR = crate::BitReader;
#[doc = "Field `SYNCI` reader - Input synchronization latched status bit"]
pub type SynciR = crate::BitReader;
#[doc = "Field `SYNCI` writer - Input synchronization latched status bit"]
pub type SynciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRMAX` reader - Time-Base counter max latched status bit"]
pub type CtrmaxR = crate::BitReader;
#[doc = "Field `CTRMAX` writer - Time-Base counter max latched status bit"]
pub type CtrmaxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Time-Base counter direction status bit"]
    #[inline(always)]
    pub fn ctrdir(&self) -> CtrdirR {
        CtrdirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input synchronization latched status bit"]
    #[inline(always)]
    pub fn synci(&self) -> SynciR {
        SynciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time-Base counter max latched status bit"]
    #[inline(always)]
    pub fn ctrmax(&self) -> CtrmaxR {
        CtrmaxR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Input synchronization latched status bit"]
    #[inline(always)]
    pub fn synci(&mut self) -> SynciW<'_, TbstsSpec> {
        SynciW::new(self, 1)
    }
    #[doc = "Bit 2 - Time-Base counter max latched status bit"]
    #[inline(always)]
    pub fn ctrmax(&mut self) -> CtrmaxW<'_, TbstsSpec> {
        CtrmaxW::new(self, 2)
    }
}
#[doc = "Time-Base Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbstsSpec;
impl crate::RegisterSpec for TbstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbsts::R`](R) reader structure"]
impl crate::Readable for TbstsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbsts::W`](W) writer structure"]
impl crate::Writable for TbstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBSTS to value 0"]
impl crate::Resettable for TbstsSpec {}
