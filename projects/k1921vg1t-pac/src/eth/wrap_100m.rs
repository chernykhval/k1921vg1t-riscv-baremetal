#[doc = "Register `WRAP_100M` reader"]
pub type R = crate::R<Wrap100mSpec>;
#[doc = "Register `WRAP_100M` writer"]
pub type W = crate::W<Wrap100mSpec>;
#[doc = "Field `WRAP` reader - Number of GTX_CLK cycles to generate 25Mhz clock pattern"]
pub type WrapR = crate::FieldReader;
#[doc = "Field `WRAP` writer - Number of GTX_CLK cycles to generate 25Mhz clock pattern"]
pub type WrapW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of GTX_CLK cycles to generate 25Mhz clock pattern"]
    #[inline(always)]
    pub fn wrap(&self) -> WrapR {
        WrapR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of GTX_CLK cycles to generate 25Mhz clock pattern"]
    #[inline(always)]
    pub fn wrap(&mut self) -> WrapW<'_, Wrap100mSpec> {
        WrapW::new(self, 0)
    }
}
#[doc = "RGMII 25MHz clock wrap register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrap_100m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrap_100m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrap100mSpec;
impl crate::RegisterSpec for Wrap100mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrap_100m::R`](R) reader structure"]
impl crate::Readable for Wrap100mSpec {}
#[doc = "`write(|w| ..)` method takes [`wrap_100m::W`](W) writer structure"]
impl crate::Writable for Wrap100mSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRAP_100M to value 0"]
impl crate::Resettable for Wrap100mSpec {}
