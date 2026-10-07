#[doc = "Register `DACCTL` reader"]
pub type R = crate::R<DacctlSpec>;
#[doc = "Register `DACCTL` writer"]
pub type W = crate::W<DacctlSpec>;
#[doc = "Field `REF0` reader - Select voltage for DAC output channel 0"]
pub type Ref0R = crate::FieldReader;
#[doc = "Field `REF0` writer - Select voltage for DAC output channel 0"]
pub type Ref0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REF1` reader - Select Voltage for DAC output channel 1"]
pub type Ref1R = crate::FieldReader;
#[doc = "Field `REF1` writer - Select Voltage for DAC output channel 1"]
pub type Ref1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EN` reader - Enable DAC for comparator"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable DAC for comparator"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select voltage for DAC output channel 0"]
    #[inline(always)]
    pub fn ref0(&self) -> Ref0R {
        Ref0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select Voltage for DAC output channel 1"]
    #[inline(always)]
    pub fn ref1(&self) -> Ref1R {
        Ref1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enable DAC for comparator"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select voltage for DAC output channel 0"]
    #[inline(always)]
    pub fn ref0(&mut self) -> Ref0W<'_, DacctlSpec> {
        Ref0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Select Voltage for DAC output channel 1"]
    #[inline(always)]
    pub fn ref1(&mut self) -> Ref1W<'_, DacctlSpec> {
        Ref1W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable DAC for comparator"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, DacctlSpec> {
        EnW::new(self, 16)
    }
}
#[doc = "DAC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacctlSpec;
impl crate::RegisterSpec for DacctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacctl::R`](R) reader structure"]
impl crate::Readable for DacctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dacctl::W`](W) writer structure"]
impl crate::Writable for DacctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DACCTL to value 0"]
impl crate::Resettable for DacctlSpec {}
