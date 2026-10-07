#[doc = "Register `MOIPR` reader"]
pub type R = crate::R<MoiprSpec>;
#[doc = "Register `MOIPR` writer"]
pub type W = crate::W<MoiprSpec>;
#[doc = "Field `RXINP` reader - Pointer interrupt line to interrupt after receiving"]
pub type RxinpR = crate::FieldReader;
#[doc = "Field `RXINP` writer - Pointer interrupt line to interrupt after receiving"]
pub type RxinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXINP` reader - Pointer interrupt line to interrupt after transfer"]
pub type TxinpR = crate::FieldReader;
#[doc = "Field `TXINP` writer - Pointer interrupt line to interrupt after transfer"]
pub type TxinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MPN` reader - Number message waiting bit"]
pub type MpnR = crate::FieldReader;
#[doc = "Field `MPN` writer - Number message waiting bit"]
pub type MpnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFCVAL` reader - Number of frames"]
pub type CfcvalR = crate::FieldReader<u16>;
#[doc = "Field `CFCVAL` writer - Number of frames"]
pub type CfcvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Pointer interrupt line to interrupt after receiving"]
    #[inline(always)]
    pub fn rxinp(&self) -> RxinpR {
        RxinpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pointer interrupt line to interrupt after transfer"]
    #[inline(always)]
    pub fn txinp(&self) -> TxinpR {
        TxinpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Number message waiting bit"]
    #[inline(always)]
    pub fn mpn(&self) -> MpnR {
        MpnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Number of frames"]
    #[inline(always)]
    pub fn cfcval(&self) -> CfcvalR {
        CfcvalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pointer interrupt line to interrupt after receiving"]
    #[inline(always)]
    pub fn rxinp(&mut self) -> RxinpW<'_, MoiprSpec> {
        RxinpW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pointer interrupt line to interrupt after transfer"]
    #[inline(always)]
    pub fn txinp(&mut self) -> TxinpW<'_, MoiprSpec> {
        TxinpW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Number message waiting bit"]
    #[inline(always)]
    pub fn mpn(&mut self) -> MpnW<'_, MoiprSpec> {
        MpnW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Number of frames"]
    #[inline(always)]
    pub fn cfcval(&mut self) -> CfcvalW<'_, MoiprSpec> {
        CfcvalW::new(self, 16)
    }
}
#[doc = "Pointer register interrupt message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`moipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoiprSpec;
impl crate::RegisterSpec for MoiprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moipr::R`](R) reader structure"]
impl crate::Readable for MoiprSpec {}
#[doc = "`write(|w| ..)` method takes [`moipr::W`](W) writer structure"]
impl crate::Writable for MoiprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOIPR to value 0"]
impl crate::Resettable for MoiprSpec {}
