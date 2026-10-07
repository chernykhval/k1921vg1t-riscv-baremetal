#[doc = "Register `EDGE1_100M` reader"]
pub type R = crate::R<Edge1_100mSpec>;
#[doc = "Register `EDGE1_100M` writer"]
pub type W = crate::W<Edge1_100mSpec>;
#[doc = "Field `EDGE1` reader - Number of GTX_CLK cycles to before clock change state"]
pub type Edge1R = crate::FieldReader;
#[doc = "Field `EDGE1` writer - Number of GTX_CLK cycles to before clock change state"]
pub type Edge1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of GTX_CLK cycles to before clock change state"]
    #[inline(always)]
    pub fn edge1(&self) -> Edge1R {
        Edge1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of GTX_CLK cycles to before clock change state"]
    #[inline(always)]
    pub fn edge1(&mut self) -> Edge1W<'_, Edge1_100mSpec> {
        Edge1W::new(self, 0)
    }
}
#[doc = "RGMII 25 MHz first edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`edge1_100m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge1_100m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Edge1_100mSpec;
impl crate::RegisterSpec for Edge1_100mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edge1_100m::R`](R) reader structure"]
impl crate::Readable for Edge1_100mSpec {}
#[doc = "`write(|w| ..)` method takes [`edge1_100m::W`](W) writer structure"]
impl crate::Writable for Edge1_100mSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EDGE1_100M to value 0"]
impl crate::Resettable for Edge1_100mSpec {}
