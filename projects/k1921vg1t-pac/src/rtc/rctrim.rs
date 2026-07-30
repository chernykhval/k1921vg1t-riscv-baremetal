#[doc = "Register `RCTRIM` reader"]
pub type R = crate::R<RctrimSpec>;
#[doc = "Register `RCTRIM` writer"]
pub type W = crate::W<RctrimSpec>;
#[doc = "Field `TRIMVAL` reader - Trim value"]
pub type TrimvalR = crate::FieldReader;
#[doc = "Field `TRIMVAL` writer - Trim value"]
pub type TrimvalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SIGN` reader - Sign of trim value"]
pub type SignR = crate::BitReader;
#[doc = "Field `SIGN` writer - Sign of trim value"]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Trim value"]
    #[inline(always)]
    pub fn trimval(&self) -> TrimvalR {
        TrimvalR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Sign of trim value"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trim value"]
    #[inline(always)]
    pub fn trimval(&mut self) -> TrimvalW<'_, RctrimSpec> {
        TrimvalW::new(self, 0)
    }
    #[doc = "Bit 7 - Sign of trim value"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, RctrimSpec> {
        SignW::new(self, 7)
    }
}
#[doc = "RC trim register\n\nYou can [`read`](crate::Reg::read) this register and get [`rctrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rctrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RctrimSpec;
impl crate::RegisterSpec for RctrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rctrim::R`](R) reader structure"]
impl crate::Readable for RctrimSpec {}
#[doc = "`write(|w| ..)` method takes [`rctrim::W`](W) writer structure"]
impl crate::Writable for RctrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCTRIM to value 0"]
impl crate::Resettable for RctrimSpec {}
