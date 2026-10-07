#[doc = "Register `SRQSEL3` reader"]
pub type R = crate::R<Srqsel3Spec>;
#[doc = "Register `SRQSEL3` writer"]
pub type W = crate::W<Srqsel3Spec>;
#[doc = "Field `RQ12` reader - Select ADC channel for request 12"]
pub type Rq12R = crate::FieldReader;
#[doc = "Field `RQ12` writer - Select ADC channel for request 12"]
pub type Rq12W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ13` reader - Select ADC channel for request 13"]
pub type Rq13R = crate::FieldReader;
#[doc = "Field `RQ13` writer - Select ADC channel for request 13"]
pub type Rq13W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ14` reader - Select ADC channel for request 14"]
pub type Rq14R = crate::FieldReader;
#[doc = "Field `RQ14` writer - Select ADC channel for request 14"]
pub type Rq14W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RQ15` reader - Select ADC channel for request 15"]
pub type Rq15R = crate::FieldReader;
#[doc = "Field `RQ15` writer - Select ADC channel for request 15"]
pub type Rq15W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Select ADC channel for request 12"]
    #[inline(always)]
    pub fn rq12(&self) -> Rq12R {
        Rq12R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 13"]
    #[inline(always)]
    pub fn rq13(&self) -> Rq13R {
        Rq13R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 14"]
    #[inline(always)]
    pub fn rq14(&self) -> Rq14R {
        Rq14R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 15"]
    #[inline(always)]
    pub fn rq15(&self) -> Rq15R {
        Rq15R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select ADC channel for request 12"]
    #[inline(always)]
    pub fn rq12(&mut self) -> Rq12W<'_, Srqsel3Spec> {
        Rq12W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Select ADC channel for request 13"]
    #[inline(always)]
    pub fn rq13(&mut self) -> Rq13W<'_, Srqsel3Spec> {
        Rq13W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Select ADC channel for request 14"]
    #[inline(always)]
    pub fn rq14(&mut self) -> Rq14W<'_, Srqsel3Spec> {
        Rq14W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Select ADC channel for request 15"]
    #[inline(always)]
    pub fn rq15(&mut self) -> Rq15W<'_, Srqsel3Spec> {
        Rq15W::new(self, 24)
    }
}
#[doc = "Sequencer request ADC channels selection register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srqsel3Spec;
impl crate::RegisterSpec for Srqsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srqsel3::R`](R) reader structure"]
impl crate::Readable for Srqsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`srqsel3::W`](W) writer structure"]
impl crate::Writable for Srqsel3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRQSEL3 to value 0"]
impl crate::Resettable for Srqsel3Spec {}
