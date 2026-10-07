#[doc = "Register `DATA_DUAL` reader"]
pub type R = crate::R<DataDualSpec>;
#[doc = "Register `DATA_DUAL` writer"]
pub type W = crate::W<DataDualSpec>;
#[doc = "Field `DAC0` reader - Value for DAC0 in Dual mode"]
pub type Dac0R = crate::FieldReader<u16>;
#[doc = "Field `DAC0` writer - Value for DAC0 in Dual mode"]
pub type Dac0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DAC1` reader - Value for DAC1 in Dual mode"]
pub type Dac1R = crate::FieldReader<u16>;
#[doc = "Field `DAC1` writer - Value for DAC1 in Dual mode"]
pub type Dac1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Value for DAC0 in Dual mode"]
    #[inline(always)]
    pub fn dac0(&self) -> Dac0R {
        Dac0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Value for DAC1 in Dual mode"]
    #[inline(always)]
    pub fn dac1(&self) -> Dac1R {
        Dac1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Value for DAC0 in Dual mode"]
    #[inline(always)]
    pub fn dac0(&mut self) -> Dac0W<'_, DataDualSpec> {
        Dac0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Value for DAC1 in Dual mode"]
    #[inline(always)]
    pub fn dac1(&mut self) -> Dac1W<'_, DataDualSpec> {
        Dac1W::new(self, 16)
    }
}
#[doc = "Output data register (Dual mode and DAC0 only)\n\nYou can [`read`](crate::Reg::read) this register and get [`data_dual::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_dual::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataDualSpec;
impl crate::RegisterSpec for DataDualSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_dual::R`](R) reader structure"]
impl crate::Readable for DataDualSpec {}
#[doc = "`write(|w| ..)` method takes [`data_dual::W`](W) writer structure"]
impl crate::Writable for DataDualSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_DUAL to value 0"]
impl crate::Resettable for DataDualSpec {}
