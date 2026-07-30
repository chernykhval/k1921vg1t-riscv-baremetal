#[doc = "Register `ERROR_POS` reader"]
pub type R = crate::R<ErrorPosSpec>;
#[doc = "Register `ERROR_POS` writer"]
pub type W = crate::W<ErrorPosSpec>;
#[doc = "Field `SEU_BSEL` reader - Byte position in a word in which single error has been occured"]
pub type SeuBselR = crate::FieldReader;
#[doc = "Field `SEU_BSEL` writer - Byte position in a word in which single error has been occured"]
pub type SeuBselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEU_BSEL` reader - Byte position in a word in which double error has been occured"]
pub type DeuBselR = crate::FieldReader;
#[doc = "Field `DEU_BSEL` writer - Byte position in a word in which double error has been occured"]
pub type DeuBselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Byte position in a word in which single error has been occured"]
    #[inline(always)]
    pub fn seu_bsel(&self) -> SeuBselR {
        SeuBselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Byte position in a word in which double error has been occured"]
    #[inline(always)]
    pub fn deu_bsel(&self) -> DeuBselR {
        DeuBselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Byte position in a word in which single error has been occured"]
    #[inline(always)]
    pub fn seu_bsel(&mut self) -> SeuBselW<'_, ErrorPosSpec> {
        SeuBselW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Byte position in a word in which double error has been occured"]
    #[inline(always)]
    pub fn deu_bsel(&mut self) -> DeuBselW<'_, ErrorPosSpec> {
        DeuBselW::new(self, 16)
    }
}
#[doc = "SRAM Error Position in Byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorPosSpec;
impl crate::RegisterSpec for ErrorPosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_pos::R`](R) reader structure"]
impl crate::Readable for ErrorPosSpec {}
#[doc = "`write(|w| ..)` method takes [`error_pos::W`](W) writer structure"]
impl crate::Writable for ErrorPosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERROR_POS to value 0"]
impl crate::Resettable for ErrorPosSpec {}
