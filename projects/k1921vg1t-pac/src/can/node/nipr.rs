#[doc = "Register `NIPR` reader"]
pub type R = crate::R<NiprSpec>;
#[doc = "Register `NIPR` writer"]
pub type W = crate::W<NiprSpec>;
#[doc = "Field `ALINP` reader - Alert interrupt node pointer - select the interrupt line for alert"]
pub type AlinpR = crate::FieldReader;
#[doc = "Field `ALINP` writer - Alert interrupt node pointer - select the interrupt line for alert"]
pub type AlinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LECINP` reader - Last error code interrupt node pointer - selects the interrupt output line for LEC"]
pub type LecinpR = crate::FieldReader;
#[doc = "Field `LECINP` writer - Last error code interrupt node pointer - selects the interrupt output line for LEC"]
pub type LecinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRINP` reader - Transfer OK interrupt node pointer - selects the interrupt line for transfer OK"]
pub type TrinpR = crate::FieldReader;
#[doc = "Field `TRINP` writer - Transfer OK interrupt node pointer - selects the interrupt line for transfer OK"]
pub type TrinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFCINP` reader - Frame counter interrupt node pointer - selects the interrupt line for a frame counter overflow"]
pub type CfcinpR = crate::FieldReader;
#[doc = "Field `CFCINP` writer - Frame counter interrupt node pointer - selects the interrupt line for a frame counter overflow"]
pub type CfcinpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alert interrupt node pointer - select the interrupt line for alert"]
    #[inline(always)]
    pub fn alinp(&self) -> AlinpR {
        AlinpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Last error code interrupt node pointer - selects the interrupt output line for LEC"]
    #[inline(always)]
    pub fn lecinp(&self) -> LecinpR {
        LecinpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transfer OK interrupt node pointer - selects the interrupt line for transfer OK"]
    #[inline(always)]
    pub fn trinp(&self) -> TrinpR {
        TrinpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Frame counter interrupt node pointer - selects the interrupt line for a frame counter overflow"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CfcinpR {
        CfcinpR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alert interrupt node pointer - select the interrupt line for alert"]
    #[inline(always)]
    pub fn alinp(&mut self) -> AlinpW<'_, NiprSpec> {
        AlinpW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Last error code interrupt node pointer - selects the interrupt output line for LEC"]
    #[inline(always)]
    pub fn lecinp(&mut self) -> LecinpW<'_, NiprSpec> {
        LecinpW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Transfer OK interrupt node pointer - selects the interrupt line for transfer OK"]
    #[inline(always)]
    pub fn trinp(&mut self) -> TrinpW<'_, NiprSpec> {
        TrinpW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Frame counter interrupt node pointer - selects the interrupt line for a frame counter overflow"]
    #[inline(always)]
    pub fn cfcinp(&mut self) -> CfcinpW<'_, NiprSpec> {
        CfcinpW::new(self, 12)
    }
}
#[doc = "Interrupt pointer register node0\n\nYou can [`read`](crate::Reg::read) this register and get [`nipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NiprSpec;
impl crate::RegisterSpec for NiprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nipr::R`](R) reader structure"]
impl crate::Readable for NiprSpec {}
#[doc = "`write(|w| ..)` method takes [`nipr::W`](W) writer structure"]
impl crate::Writable for NiprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NIPR to value 0"]
impl crate::Resettable for NiprSpec {}
