#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `RXD` reader - Received data"]
pub type RxdR = crate::FieldReader;
#[doc = "Field `RXD` writer - Received data"]
pub type RxdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Received data"]
    #[inline(always)]
    pub fn rxd(&self) -> RxdR {
        RxdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Received data"]
    #[inline(always)]
    pub fn rxd(&mut self) -> RxdW<'_, ValueSpec> {
        RxdW::new(self, 0)
    }
}
#[doc = "Received data register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
