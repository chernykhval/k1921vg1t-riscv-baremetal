#[doc = "Register `D_DATA` reader"]
pub type R = crate::R<DDataSpec>;
#[doc = "Register `D_DATA` writer"]
pub type W = crate::W<DDataSpec>;
#[doc = "Field `LOW` reader - "]
pub type LowR = crate::FieldReader<u16>;
#[doc = "Field `LOW` writer - "]
pub type LowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HIGH` reader - "]
pub type HighR = crate::FieldReader<u16>;
#[doc = "Field `HIGH` writer - "]
pub type HighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn low(&self) -> LowR {
        LowR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn high(&self) -> HighR {
        HighR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn low(&mut self) -> LowW<'_, DDataSpec> {
        LowW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn high(&mut self) -> HighW<'_, DDataSpec> {
        HighW::new(self, 16)
    }
}
#[doc = "Downstream Data bits\n\nYou can [`read`](crate::Reg::read) this register and get [`d_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDataSpec;
impl crate::RegisterSpec for DDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_data::R`](R) reader structure"]
impl crate::Readable for DDataSpec {}
#[doc = "`write(|w| ..)` method takes [`d_data::W`](W) writer structure"]
impl crate::Writable for DDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D_DATA to value 0"]
impl crate::Resettable for DDataSpec {}
