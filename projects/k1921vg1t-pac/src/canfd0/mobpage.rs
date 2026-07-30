#[doc = "Register `MOBPAGE` reader"]
pub type R = crate::R<MobpageSpec>;
#[doc = "Register `MOBPAGE` writer"]
pub type W = crate::W<MobpageSpec>;
#[doc = "Field `MOBNB` reader - Message Object Number"]
pub type MobnbR = crate::FieldReader;
#[doc = "Field `MOBNB` writer - Message Object Number"]
pub type MobnbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Message Object Number"]
    #[inline(always)]
    pub fn mobnb(&self) -> MobnbR {
        MobnbR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Message Object Number"]
    #[inline(always)]
    pub fn mobnb(&mut self) -> MobnbW<'_, MobpageSpec> {
        MobnbW::new(self, 0)
    }
}
#[doc = "CAN Page MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobpage::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobpage::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MobpageSpec;
impl crate::RegisterSpec for MobpageSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mobpage::R`](R) reader structure"]
impl crate::Readable for MobpageSpec {}
#[doc = "`write(|w| ..)` method takes [`mobpage::W`](W) writer structure"]
impl crate::Writable for MobpageSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOBPAGE to value 0"]
impl crate::Resettable for MobpageSpec {}
