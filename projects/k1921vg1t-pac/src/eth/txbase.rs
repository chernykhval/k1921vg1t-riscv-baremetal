#[doc = "Register `TXBASE` reader"]
pub type R = crate::R<TxbaseSpec>;
#[doc = "Register `TXBASE` writer"]
pub type W = crate::W<TxbaseSpec>;
#[doc = "Field `DPNT` reader - Descriptor pointer"]
pub type DpntR = crate::FieldReader;
#[doc = "Field `DPNT` writer - Descriptor pointer"]
pub type DpntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BADDR` reader - Transmitter descriptor table base address"]
pub type BaddrR = crate::FieldReader<u32>;
#[doc = "Field `BADDR` writer - Transmitter descriptor table base address"]
pub type BaddrW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 3:9 - Descriptor pointer"]
    #[inline(always)]
    pub fn dpnt(&self) -> DpntR {
        DpntR::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 10:31 - Transmitter descriptor table base address"]
    #[inline(always)]
    pub fn baddr(&self) -> BaddrR {
        BaddrR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 3:9 - Descriptor pointer"]
    #[inline(always)]
    pub fn dpnt(&mut self) -> DpntW<'_, TxbaseSpec> {
        DpntW::new(self, 3)
    }
    #[doc = "Bits 10:31 - Transmitter descriptor table base address"]
    #[inline(always)]
    pub fn baddr(&mut self) -> BaddrW<'_, TxbaseSpec> {
        BaddrW::new(self, 10)
    }
}
#[doc = "Transmitter descriptor table base address pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`txbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbaseSpec;
impl crate::RegisterSpec for TxbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbase::R`](R) reader structure"]
impl crate::Readable for TxbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`txbase::W`](W) writer structure"]
impl crate::Writable for TxbaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBASE to value 0"]
impl crate::Resettable for TxbaseSpec {}
