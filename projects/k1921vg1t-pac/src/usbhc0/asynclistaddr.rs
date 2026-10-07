#[doc = "Register `ASYNCLISTADDR` reader"]
pub type R = crate::R<AsynclistaddrSpec>;
#[doc = "Register `ASYNCLISTADDR` writer"]
pub type W = crate::W<AsynclistaddrSpec>;
#[doc = "Field `LPL` reader - "]
pub type LplR = crate::FieldReader<u32>;
#[doc = "Field `LPL` writer - "]
pub type LplW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn lpl(&self) -> LplR {
        LplR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn lpl(&mut self) -> LplW<'_, AsynclistaddrSpec> {
        LplW::new(self, 5)
    }
}
#[doc = "USBHC EHC Current Asynchronous List Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`asynclistaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asynclistaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsynclistaddrSpec;
impl crate::RegisterSpec for AsynclistaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asynclistaddr::R`](R) reader structure"]
impl crate::Readable for AsynclistaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`asynclistaddr::W`](W) writer structure"]
impl crate::Writable for AsynclistaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASYNCLISTADDR to value 0"]
impl crate::Resettable for AsynclistaddrSpec {}
