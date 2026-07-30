#[doc = "Register `FIFOLEV` reader"]
pub type R = crate::R<FifolevSpec>;
#[doc = "Register `FIFOLEV` writer"]
pub type W = crate::W<FifolevSpec>;
#[doc = "Field `VAL` reader - Number of 32 bits words of random available in the FIFO. Any write to this register clears the FifoFull 2ag in the Status register."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Number of 32 bits words of random available in the FIFO. Any write to this register clears the FifoFull 2ag in the Status register."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of 32 bits words of random available in the FIFO. Any write to this register clears the FifoFull 2ag in the Status register."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of 32 bits words of random available in the FIFO. Any write to this register clears the FifoFull 2ag in the Status register."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FifolevSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "FIFO Level register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifolev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifolevSpec;
impl crate::RegisterSpec for FifolevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifolev::R`](R) reader structure"]
impl crate::Readable for FifolevSpec {}
#[doc = "`write(|w| ..)` method takes [`fifolev::W`](W) writer structure"]
impl crate::Writable for FifolevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOLEV to value 0"]
impl crate::Resettable for FifolevSpec {}
