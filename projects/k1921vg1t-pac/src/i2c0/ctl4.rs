#[doc = "Register `CTL4` reader"]
pub type R = crate::R<Ctl4Spec>;
#[doc = "Register `CTL4` writer"]
pub type W = crate::W<Ctl4Spec>;
#[doc = "Field `HSDIV` reader - Field frequency selection signal at pin SCL in HS master mode (bits \\[11:4\\])"]
pub type HsdivR = crate::FieldReader;
#[doc = "Field `HSDIV` writer - Field frequency selection signal at pin SCL in HS master mode (bits \\[11:4\\])"]
pub type HsdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Field frequency selection signal at pin SCL in HS master mode (bits \\[11:4\\])"]
    #[inline(always)]
    pub fn hsdiv(&self) -> HsdivR {
        HsdivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Field frequency selection signal at pin SCL in HS master mode (bits \\[11:4\\])"]
    #[inline(always)]
    pub fn hsdiv(&mut self) -> HsdivW<'_, Ctl4Spec> {
        HsdivW::new(self, 0)
    }
}
#[doc = "Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl4Spec;
impl crate::RegisterSpec for Ctl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl4::R`](R) reader structure"]
impl crate::Readable for Ctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl4::W`](W) writer structure"]
impl crate::Writable for Ctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL4 to value 0"]
impl crate::Resettable for Ctl4Spec {}
