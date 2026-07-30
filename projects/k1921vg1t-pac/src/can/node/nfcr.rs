#[doc = "Register `NFCR` reader"]
pub type R = crate::R<NfcrSpec>;
#[doc = "Register `NFCR` writer"]
pub type W = crate::W<NfcrSpec>;
#[doc = "Field `CFC` reader - Field of the message counter"]
pub type CfcR = crate::FieldReader<u16>;
#[doc = "Field `CFC` writer - Field of the message counter"]
pub type CfcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CFSEL` reader - Field parameter setting mode selected message counter"]
pub type CfselR = crate::FieldReader;
#[doc = "Field `CFSEL` writer - Field parameter setting mode selected message counter"]
pub type CfselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CFMOD` reader - Field task mode message counter"]
pub type CfmodR = crate::FieldReader;
#[doc = "Field `CFMOD` writer - Field task mode message counter"]
pub type CfmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CFCIE` reader - Interrupt enable bit of the message counter"]
pub type CfcieR = crate::BitReader;
#[doc = "Field `CFCIE` writer - Interrupt enable bit of the message counter"]
pub type CfcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFCOV` reader - Counter overflow flag messages"]
pub type CfcovR = crate::BitReader;
#[doc = "Field `CFCOV` writer - Counter overflow flag messages"]
pub type CfcovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Field of the message counter"]
    #[inline(always)]
    pub fn cfc(&self) -> CfcR {
        CfcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Field parameter setting mode selected message counter"]
    #[inline(always)]
    pub fn cfsel(&self) -> CfselR {
        CfselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - Field task mode message counter"]
    #[inline(always)]
    pub fn cfmod(&self) -> CfmodR {
        CfmodR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 22 - Interrupt enable bit of the message counter"]
    #[inline(always)]
    pub fn cfcie(&self) -> CfcieR {
        CfcieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Counter overflow flag messages"]
    #[inline(always)]
    pub fn cfcov(&self) -> CfcovR {
        CfcovR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Field of the message counter"]
    #[inline(always)]
    pub fn cfc(&mut self) -> CfcW<'_, NfcrSpec> {
        CfcW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Field parameter setting mode selected message counter"]
    #[inline(always)]
    pub fn cfsel(&mut self) -> CfselW<'_, NfcrSpec> {
        CfselW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Field task mode message counter"]
    #[inline(always)]
    pub fn cfmod(&mut self) -> CfmodW<'_, NfcrSpec> {
        CfmodW::new(self, 19)
    }
    #[doc = "Bit 22 - Interrupt enable bit of the message counter"]
    #[inline(always)]
    pub fn cfcie(&mut self) -> CfcieW<'_, NfcrSpec> {
        CfcieW::new(self, 22)
    }
    #[doc = "Bit 23 - Counter overflow flag messages"]
    #[inline(always)]
    pub fn cfcov(&mut self) -> CfcovW<'_, NfcrSpec> {
        CfcovW::new(self, 23)
    }
}
#[doc = "Register message counter node0\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NfcrSpec;
impl crate::RegisterSpec for NfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfcr::R`](R) reader structure"]
impl crate::Readable for NfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`nfcr::W`](W) writer structure"]
impl crate::Writable for NfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NFCR to value 0"]
impl crate::Resettable for NfcrSpec {}
