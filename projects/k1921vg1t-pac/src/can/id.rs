#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Register `ID` writer"]
pub type W = crate::W<IdSpec>;
#[doc = "Field `MODREV` reader - Number of modifications of the CAN"]
pub type ModrevR = crate::FieldReader;
#[doc = "Field `MODREV` writer - Number of modifications of the CAN"]
pub type ModrevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MODTYPE` reader - Digit capacity CAN"]
pub type ModtypeR = crate::FieldReader;
#[doc = "Field `MODNUM` reader - Identification number CAN"]
pub type ModnumR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Number of modifications of the CAN"]
    #[inline(always)]
    pub fn modrev(&self) -> ModrevR {
        ModrevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Digit capacity CAN"]
    #[inline(always)]
    pub fn modtype(&self) -> ModtypeR {
        ModtypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Identification number CAN"]
    #[inline(always)]
    pub fn modnum(&self) -> ModnumR {
        ModnumR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of modifications of the CAN"]
    #[inline(always)]
    pub fn modrev(&mut self) -> ModrevW<'_, IdSpec> {
        ModrevW::new(self, 0)
    }
}
#[doc = "Identity register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`write(|w| ..)` method takes [`id::W`](W) writer structure"]
impl crate::Writable for IdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ID to value 0x002b_c051"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0x002b_c051;
}
