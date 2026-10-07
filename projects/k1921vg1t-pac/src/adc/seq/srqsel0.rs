#[doc = "Register `SRQSEL0` reader"]
pub type R = crate::R<Srqsel0Spec>;
#[doc = "Register `SRQSEL0` writer"]
pub type W = crate::W<Srqsel0Spec>;
#[doc = "Field `RQ0` reader - Select ADC channel for request 0"]
pub type Rq0R = crate::FieldReader;
#[doc = "Field `RQ0` writer - Select ADC channel for request 0"]
pub type Rq0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ1` reader - Select ADC channel for request 1"]
pub type Rq1R = crate::FieldReader;
#[doc = "Field `RQ1` writer - Select ADC channel for request 1"]
pub type Rq1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ2` reader - Select ADC channel for request 2"]
pub type Rq2R = crate::FieldReader;
#[doc = "Field `RQ2` writer - Select ADC channel for request 2"]
pub type Rq2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ3` reader - Select ADC channel for request 3"]
pub type Rq3R = crate::FieldReader;
#[doc = "Field `RQ3` writer - Select ADC channel for request 3"]
pub type Rq3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Select ADC channel for request 0"]
    #[inline(always)]
    pub fn rq0(&self) -> Rq0R {
        Rq0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 1"]
    #[inline(always)]
    pub fn rq1(&self) -> Rq1R {
        Rq1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 2"]
    #[inline(always)]
    pub fn rq2(&self) -> Rq2R {
        Rq2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 3"]
    #[inline(always)]
    pub fn rq3(&self) -> Rq3R {
        Rq3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select ADC channel for request 0"]
    #[inline(always)]
    pub fn rq0(&mut self) -> Rq0W<'_, Srqsel0Spec> {
        Rq0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 1"]
    #[inline(always)]
    pub fn rq1(&mut self) -> Rq1W<'_, Srqsel0Spec> {
        Rq1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 2"]
    #[inline(always)]
    pub fn rq2(&mut self) -> Rq2W<'_, Srqsel0Spec> {
        Rq2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 3"]
    #[inline(always)]
    pub fn rq3(&mut self) -> Rq3W<'_, Srqsel0Spec> {
        Rq3W::new(self, 24)
    }
}
#[doc = "Sequencer request ADC channels selection register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srqsel0Spec;
impl crate::RegisterSpec for Srqsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqsel0::R`](R) reader structure"]
impl crate::Readable for Srqsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`srqsel0::W`](W) writer structure"]
impl crate::Writable for Srqsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRQSEL0 to value 0"]
impl crate::Resettable for Srqsel0Spec {}
