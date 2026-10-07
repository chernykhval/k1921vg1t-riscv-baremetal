#[doc = "Register `SRQSEL4` reader"]
pub type R = crate::R<Srqsel4Spec>;
#[doc = "Register `SRQSEL4` writer"]
pub type W = crate::W<Srqsel4Spec>;
#[doc = "Field `RQ16` reader - Select ADC channel for request 16"]
pub type Rq16R = crate::FieldReader;
#[doc = "Field `RQ16` writer - Select ADC channel for request 16"]
pub type Rq16W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ17` reader - Select ADC channel for request 17"]
pub type Rq17R = crate::FieldReader;
#[doc = "Field `RQ17` writer - Select ADC channel for request 17"]
pub type Rq17W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ18` reader - Select ADC channel for request 18"]
pub type Rq18R = crate::FieldReader;
#[doc = "Field `RQ18` writer - Select ADC channel for request 18"]
pub type Rq18W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ19` reader - Select ADC channel for request 19"]
pub type Rq19R = crate::FieldReader;
#[doc = "Field `RQ19` writer - Select ADC channel for request 19"]
pub type Rq19W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Select ADC channel for request 16"]
    #[inline(always)]
    pub fn rq16(&self) -> Rq16R {
        Rq16R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 17"]
    #[inline(always)]
    pub fn rq17(&self) -> Rq17R {
        Rq17R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 18"]
    #[inline(always)]
    pub fn rq18(&self) -> Rq18R {
        Rq18R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 19"]
    #[inline(always)]
    pub fn rq19(&self) -> Rq19R {
        Rq19R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select ADC channel for request 16"]
    #[inline(always)]
    pub fn rq16(&mut self) -> Rq16W<'_, Srqsel4Spec> {
        Rq16W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 17"]
    #[inline(always)]
    pub fn rq17(&mut self) -> Rq17W<'_, Srqsel4Spec> {
        Rq17W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 18"]
    #[inline(always)]
    pub fn rq18(&mut self) -> Rq18W<'_, Srqsel4Spec> {
        Rq18W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 19"]
    #[inline(always)]
    pub fn rq19(&mut self) -> Rq19W<'_, Srqsel4Spec> {
        Rq19W::new(self, 24)
    }
}
#[doc = "Sequencer request ADC channels selection register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srqsel4Spec;
impl crate::RegisterSpec for Srqsel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqsel4::R`](R) reader structure"]
impl crate::Readable for Srqsel4Spec {}
#[doc = "`write(|w| ..)` method takes [`srqsel4::W`](W) writer structure"]
impl crate::Writable for Srqsel4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRQSEL4 to value 0"]
impl crate::Resettable for Srqsel4Spec {}
