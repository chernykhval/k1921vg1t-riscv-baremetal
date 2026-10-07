#[doc = "Register `TACC1` reader"]
pub type R = crate::R<Tacc1Spec>;
#[doc = "Register `TACC1` writer"]
pub type W = crate::W<Tacc1Spec>;
#[doc = "Field `DATA` reader - AD conversion value"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `EOC` reader - End of conversion"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOCF` reader - Detect falling edge of EOC signal"]
pub type EocfR = crate::BitReader;
#[doc = "Field `EOCF` writer - Detect falling edge of EOC signal"]
pub type EocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCR` reader - Detect rise edge of EOC signal"]
pub type EocrR = crate::BitReader;
#[doc = "Field `EOCR` writer - Detect rise edge of EOC signal"]
pub type EocrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - AD conversion value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - End of conversion"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Detect falling edge of EOC signal"]
    #[inline(always)]
    pub fn eocf(&self) -> EocfR {
        EocfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Detect rise edge of EOC signal"]
    #[inline(always)]
    pub fn eocr(&self) -> EocrR {
        EocrR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Detect falling edge of EOC signal"]
    #[inline(always)]
    pub fn eocf(&mut self) -> EocfW<'_, Tacc1Spec> {
        EocfW::new(self, 16)
    }
    #[doc = "Bit 17 - Detect rise edge of EOC signal"]
    #[inline(always)]
    pub fn eocr(&mut self) -> EocrW<'_, Tacc1Spec> {
        EocrW::new(self, 17)
    }
}
#[doc = "ADC test access register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tacc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tacc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tacc1Spec;
impl crate::RegisterSpec for Tacc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tacc1::R`](R) reader structure"]
impl crate::Readable for Tacc1Spec {}
#[doc = "`write(|w| ..)` method takes [`tacc1::W`](W) writer structure"]
impl crate::Writable for Tacc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TACC1 to value 0"]
impl crate::Resettable for Tacc1Spec {}
