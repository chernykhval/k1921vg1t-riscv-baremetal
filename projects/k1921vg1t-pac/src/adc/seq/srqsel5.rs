#[doc = "Register `SRQSEL5` reader"]
pub type R = crate::R<Srqsel5Spec>;
#[doc = "Register `SRQSEL5` writer"]
pub type W = crate::W<Srqsel5Spec>;
#[doc = "Field `RQ20` reader - Select ADC channel for request 20"]
pub type Rq20R = crate::FieldReader;
#[doc = "Field `RQ20` writer - Select ADC channel for request 20"]
pub type Rq20W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ21` reader - Select ADC channel for request 21"]
pub type Rq21R = crate::FieldReader;
#[doc = "Field `RQ21` writer - Select ADC channel for request 21"]
pub type Rq21W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ22` reader - Select ADC channel for request 22"]
pub type Rq22R = crate::FieldReader;
#[doc = "Field `RQ22` writer - Select ADC channel for request 22"]
pub type Rq22W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ23` reader - Select ADC channel for request 23"]
pub type Rq23R = crate::FieldReader;
#[doc = "Field `RQ23` writer - Select ADC channel for request 23"]
pub type Rq23W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Select ADC channel for request 20"]
    #[inline(always)]
    pub fn rq20(&self) -> Rq20R {
        Rq20R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 21"]
    #[inline(always)]
    pub fn rq21(&self) -> Rq21R {
        Rq21R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 22"]
    #[inline(always)]
    pub fn rq22(&self) -> Rq22R {
        Rq22R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 23"]
    #[inline(always)]
    pub fn rq23(&self) -> Rq23R {
        Rq23R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select ADC channel for request 20"]
    #[inline(always)]
    pub fn rq20(&mut self) -> Rq20W<'_, Srqsel5Spec> {
        Rq20W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 21"]
    #[inline(always)]
    pub fn rq21(&mut self) -> Rq21W<'_, Srqsel5Spec> {
        Rq21W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 22"]
    #[inline(always)]
    pub fn rq22(&mut self) -> Rq22W<'_, Srqsel5Spec> {
        Rq22W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 23"]
    #[inline(always)]
    pub fn rq23(&mut self) -> Rq23W<'_, Srqsel5Spec> {
        Rq23W::new(self, 24)
    }
}
#[doc = "Sequencer request ADC channels selection register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srqsel5Spec;
impl crate::RegisterSpec for Srqsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqsel5::R`](R) reader structure"]
impl crate::Readable for Srqsel5Spec {}
#[doc = "`write(|w| ..)` method takes [`srqsel5::W`](W) writer structure"]
impl crate::Writable for Srqsel5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRQSEL5 to value 0"]
impl crate::Resettable for Srqsel5Spec {}
