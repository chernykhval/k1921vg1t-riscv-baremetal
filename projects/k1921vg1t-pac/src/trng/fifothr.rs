#[doc = "Register `FIFOTHR` reader"]
pub type R = crate::R<FifothrSpec>;
#[doc = "Register `FIFOTHR` writer"]
pub type W = crate::W<FifothrSpec>;
#[doc = "Field `VAL` reader - FIFO level below which the module starts to refill the FIFO, expressed in number of 128bit blocks."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - FIFO level below which the module starts to refill the FIFO, expressed in number of 128bit blocks."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - FIFO level below which the module starts to refill the FIFO, expressed in number of 128bit blocks."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO level below which the module starts to refill the FIFO, expressed in number of 128bit blocks."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FifothrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "FIFO Threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifothrSpec;
impl crate::RegisterSpec for FifothrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifothr::R`](R) reader structure"]
impl crate::Readable for FifothrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifothr::W`](W) writer structure"]
impl crate::Writable for FifothrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOTHR to value 0x07"]
impl crate::Resettable for FifothrSpec {
    const RESET_VALUE: u32 = 0x07;
}
