#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `ENABLE` reader - Enable I2C"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable I2C"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLFRQ` reader - Field frequency selection signal at pin SCL in master mode (bits \\[6:0\\])"]
pub type SclfrqR = crate::FieldReader;
#[doc = "Field `SCLFRQ` writer - Field frequency selection signal at pin SCL in master mode (bits \\[6:0\\])"]
pub type SclfrqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable I2C"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Field frequency selection signal at pin SCL in master mode (bits \\[6:0\\])"]
    #[inline(always)]
    pub fn sclfrq(&self) -> SclfrqR {
        SclfrqR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable I2C"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Ctl1Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Field frequency selection signal at pin SCL in master mode (bits \\[6:0\\])"]
    #[inline(always)]
    pub fn sclfrq(&mut self) -> SclfrqW<'_, Ctl1Spec> {
        SclfrqW::new(self, 1)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
