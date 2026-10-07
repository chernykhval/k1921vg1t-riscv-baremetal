#[doc = "Register `SRR` reader"]
pub type R = crate::R<SrrSpec>;
#[doc = "Register `SRR` writer"]
pub type W = crate::W<SrrSpec>;
#[doc = "Field `SPDR` reader - Transmission rate of the main device"]
pub type SpdrR = crate::FieldReader<u16>;
#[doc = "Field `SPDR` writer - Transmission rate of the main device"]
pub type SpdrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmission rate of the main device"]
    #[inline(always)]
    pub fn spdr(&self) -> SpdrR {
        SpdrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmission rate of the main device"]
    #[inline(always)]
    pub fn spdr(&mut self) -> SpdrW<'_, SrrSpec> {
        SpdrW::new(self, 0)
    }
}
#[doc = "Transfer rate setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`srr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrrSpec;
impl crate::RegisterSpec for SrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srr::R`](R) reader structure"]
impl crate::Readable for SrrSpec {}
#[doc = "`write(|w| ..)` method takes [`srr::W`](W) writer structure"]
impl crate::Writable for SrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRR to value 0x012c"]
impl crate::Resettable for SrrSpec {
    const RESET_VALUE: u32 = 0x012c;
}
