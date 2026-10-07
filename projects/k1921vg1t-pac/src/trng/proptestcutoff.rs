#[doc = "Register `PROPTESTCUTOFF` reader"]
pub type R = crate::R<ProptestcutoffSpec>;
#[doc = "Register `PROPTESTCUTOFF` writer"]
pub type W = crate::W<ProptestcutoffSpec>;
#[doc = "Field `VAL` reader - Proportion test cut-off value"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Proportion test cut-off value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Proportion test cut-off value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Proportion test cut-off value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, ProptestcutoffSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Proportion test cut-off value register\n\nYou can [`read`](crate::Reg::read) this register and get [`proptestcutoff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proptestcutoff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProptestcutoffSpec;
impl crate::RegisterSpec for ProptestcutoffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proptestcutoff::R`](R) reader structure"]
impl crate::Readable for ProptestcutoffSpec {}
#[doc = "`write(|w| ..)` method takes [`proptestcutoff::W`](W) writer structure"]
impl crate::Writable for ProptestcutoffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PROPTESTCUTOFF to value 0x3e"]
impl crate::Resettable for ProptestcutoffSpec {
    const RESET_VALUE: u32 = 0x3e;
}
