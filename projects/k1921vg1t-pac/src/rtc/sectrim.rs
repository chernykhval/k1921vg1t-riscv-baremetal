#[doc = "Register `SECTRIM` reader"]
pub type R = crate::R<SectrimSpec>;
#[doc = "Register `SECTRIM` writer"]
pub type W = crate::W<SectrimSpec>;
#[doc = "Field `TRIMVAL` reader - Trim value"]
pub type TrimvalR = crate::FieldReader<u16>;
#[doc = "Field `TRIMVAL` writer - Trim value"]
pub type TrimvalW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SIGN` reader - Sign of trim value"]
pub type SignR = crate::BitReader;
#[doc = "Field `SIGN` writer - Sign of trim value"]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Trim value"]
    #[inline(always)]
    pub fn trimval(&self) -> TrimvalR {
        TrimvalR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Sign of trim value"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Trim value"]
    #[inline(always)]
    pub fn trimval(&mut self) -> TrimvalW<'_, SectrimSpec> {
        TrimvalW::new(self, 0)
    }
    #[doc = "Bit 10 - Sign of trim value"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, SectrimSpec> {
        SignW::new(self, 10)
    }
}
#[doc = "One second clock trim register\n\nYou can [`read`](crate::Reg::read) this register and get [`sectrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sectrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SectrimSpec;
impl crate::RegisterSpec for SectrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sectrim::R`](R) reader structure"]
impl crate::Readable for SectrimSpec {}
#[doc = "`write(|w| ..)` method takes [`sectrim::W`](W) writer structure"]
impl crate::Writable for SectrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECTRIM to value 0"]
impl crate::Resettable for SectrimSpec {}
