#[doc = "Register `SRQSEL2` reader"]
pub type R = crate::R<Srqsel2Spec>;
#[doc = "Register `SRQSEL2` writer"]
pub type W = crate::W<Srqsel2Spec>;
#[doc = "Field `RQ8` reader - Select ADC channel for request 8"]
pub type Rq8R = crate::FieldReader;
#[doc = "Field `RQ8` writer - Select ADC channel for request 8"]
pub type Rq8W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ9` reader - Select ADC channel for request 9"]
pub type Rq9R = crate::FieldReader;
#[doc = "Field `RQ9` writer - Select ADC channel for request 9"]
pub type Rq9W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ10` reader - Select ADC channel for request 10"]
pub type Rq10R = crate::FieldReader;
#[doc = "Field `RQ10` writer - Select ADC channel for request 10"]
pub type Rq10W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ11` reader - Select ADC channel for request 11"]
pub type Rq11R = crate::FieldReader;
#[doc = "Field `RQ11` writer - Select ADC channel for request 11"]
pub type Rq11W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Select ADC channel for request 8"]
    #[inline(always)]
    pub fn rq8(&self) -> Rq8R {
        Rq8R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 9"]
    #[inline(always)]
    pub fn rq9(&self) -> Rq9R {
        Rq9R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 10"]
    #[inline(always)]
    pub fn rq10(&self) -> Rq10R {
        Rq10R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 11"]
    #[inline(always)]
    pub fn rq11(&self) -> Rq11R {
        Rq11R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select ADC channel for request 8"]
    #[inline(always)]
    pub fn rq8(&mut self) -> Rq8W<'_, Srqsel2Spec> {
        Rq8W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 9"]
    #[inline(always)]
    pub fn rq9(&mut self) -> Rq9W<'_, Srqsel2Spec> {
        Rq9W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 10"]
    #[inline(always)]
    pub fn rq10(&mut self) -> Rq10W<'_, Srqsel2Spec> {
        Rq10W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 11"]
    #[inline(always)]
    pub fn rq11(&mut self) -> Rq11W<'_, Srqsel2Spec> {
        Rq11W::new(self, 24)
    }
}
#[doc = "Sequencer request ADC channels selection register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srqsel2Spec;
impl crate::RegisterSpec for Srqsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqsel2::R`](R) reader structure"]
impl crate::Readable for Srqsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`srqsel2::W`](W) writer structure"]
impl crate::Writable for Srqsel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRQSEL2 to value 0"]
impl crate::Resettable for Srqsel2Spec {}
