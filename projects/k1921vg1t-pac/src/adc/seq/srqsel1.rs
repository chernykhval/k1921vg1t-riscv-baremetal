#[doc = "Register `SRQSEL1` reader"]
pub type R = crate::R<Srqsel1Spec>;
#[doc = "Register `SRQSEL1` writer"]
pub type W = crate::W<Srqsel1Spec>;
#[doc = "Field `RQ4` reader - Select ADC channel for request 4"]
pub type Rq4R = crate::FieldReader;
#[doc = "Field `RQ4` writer - Select ADC channel for request 4"]
pub type Rq4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ5` reader - Select ADC channel for request 5"]
pub type Rq5R = crate::FieldReader;
#[doc = "Field `RQ5` writer - Select ADC channel for request 5"]
pub type Rq5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ6` reader - Select ADC channel for request 6"]
pub type Rq6R = crate::FieldReader;
#[doc = "Field `RQ6` writer - Select ADC channel for request 6"]
pub type Rq6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ7` reader - Select ADC channel for request 7"]
pub type Rq7R = crate::FieldReader;
#[doc = "Field `RQ7` writer - Select ADC channel for request 7"]
pub type Rq7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Select ADC channel for request 4"]
    #[inline(always)]
    pub fn rq4(&self) -> Rq4R {
        Rq4R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 5"]
    #[inline(always)]
    pub fn rq5(&self) -> Rq5R {
        Rq5R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 6"]
    #[inline(always)]
    pub fn rq6(&self) -> Rq6R {
        Rq6R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 7"]
    #[inline(always)]
    pub fn rq7(&self) -> Rq7R {
        Rq7R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select ADC channel for request 4"]
    #[inline(always)]
    pub fn rq4(&mut self) -> Rq4W<'_, Srqsel1Spec> {
        Rq4W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 5"]
    #[inline(always)]
    pub fn rq5(&mut self) -> Rq5W<'_, Srqsel1Spec> {
        Rq5W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 6"]
    #[inline(always)]
    pub fn rq6(&mut self) -> Rq6W<'_, Srqsel1Spec> {
        Rq6W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 7"]
    #[inline(always)]
    pub fn rq7(&mut self) -> Rq7W<'_, Srqsel1Spec> {
        Rq7W::new(self, 24)
    }
}
#[doc = "Sequencer request ADC channels selection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srqsel1Spec;
impl crate::RegisterSpec for Srqsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqsel1::R`](R) reader structure"]
impl crate::Readable for Srqsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`srqsel1::W`](W) writer structure"]
impl crate::Writable for Srqsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRQSEL1 to value 0"]
impl crate::Resettable for Srqsel1Spec {}
