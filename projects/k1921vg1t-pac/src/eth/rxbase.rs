#[doc = "Register `RXBASE` reader"]
pub type R = crate::R<RxbaseSpec>;
#[doc = "Register `RXBASE` writer"]
pub type W = crate::W<RxbaseSpec>;
#[doc = "Field `DPNT` reader - Descriptor pointer"]
pub type DpntR = crate::FieldReader;
#[doc = "Field `DPNT` writer - Descriptor pointer"]
pub type DpntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BADDR` reader - Receiver descriptor table base address"]
pub type BaddrR = crate::FieldReader<u32>;
#[doc = "Field `BADDR` writer - Receiver descriptor table base address"]
pub type BaddrW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 3:9 - Descriptor pointer"]
    #[inline(always)]
    pub fn dpnt(&self) -> DpntR {
        DpntR::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 10:31 - Receiver descriptor table base address"]
    #[inline(always)]
    pub fn baddr(&self) -> BaddrR {
        BaddrR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 3:9 - Descriptor pointer"]
    #[inline(always)]
    pub fn dpnt(&mut self) -> DpntW<'_, RxbaseSpec> {
        DpntW::new(self, 3)
    }
    #[doc = "Bits 10:31 - Receiver descriptor table base address"]
    #[inline(always)]
    pub fn baddr(&mut self) -> BaddrW<'_, RxbaseSpec> {
        BaddrW::new(self, 10)
    }
}
#[doc = "Receiver descriptor table base address pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbaseSpec;
impl crate::RegisterSpec for RxbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbase::R`](R) reader structure"]
impl crate::Readable for RxbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`rxbase::W`](W) writer structure"]
impl crate::Writable for RxbaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXBASE to value 0"]
impl crate::Resettable for RxbaseSpec {}
