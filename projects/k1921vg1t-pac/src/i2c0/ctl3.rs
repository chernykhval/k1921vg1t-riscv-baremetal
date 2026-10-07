#[doc = "Register `CTL3` reader"]
pub type R = crate::R<Ctl3Spec>;
#[doc = "Register `CTL3` writer"]
pub type W = crate::W<Ctl3Spec>;
#[doc = "Field `SCLFRQ` reader - Field frequency selection signal at pin SCL in master mode (bits \\[14:7\\])"]
pub type SclfrqR = crate::FieldReader;
#[doc = "Field `SCLFRQ` writer - Field frequency selection signal at pin SCL in master mode (bits \\[14:7\\])"]
pub type SclfrqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Field frequency selection signal at pin SCL in master mode (bits \\[14:7\\])"]
    #[inline(always)]
    pub fn sclfrq(&self) -> SclfrqR {
        SclfrqR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Field frequency selection signal at pin SCL in master mode (bits \\[14:7\\])"]
    #[inline(always)]
    pub fn sclfrq(&mut self) -> SclfrqW<'_, Ctl3Spec> {
        SclfrqW::new(self, 0)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl3Spec;
impl crate::RegisterSpec for Ctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl3::R`](R) reader structure"]
impl crate::Readable for Ctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl3::W`](W) writer structure"]
impl crate::Writable for Ctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL3 to value 0"]
impl crate::Resettable for Ctl3Spec {}
