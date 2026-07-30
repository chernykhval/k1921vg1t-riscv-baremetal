#[doc = "Register `EDGE2_100M` reader"]
pub type R = crate::R<Edge2_100mSpec>;
#[doc = "Register `EDGE2_100M` writer"]
pub type W = crate::W<Edge2_100mSpec>;
#[doc = "Field `EDGE2` reader - Number of GTX_CLK cycles after the first edge the clock change state"]
pub type Edge2R = crate::FieldReader;
#[doc = "Field `EDGE2` writer - Number of GTX_CLK cycles after the first edge the clock change state"]
pub type Edge2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Number of GTX_CLK cycles after the first edge the clock change state"]
    #[inline(always)]
    pub fn edge2(&self) -> Edge2R {
        Edge2R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of GTX_CLK cycles after the first edge the clock change state"]
    #[inline(always)]
    pub fn edge2(&mut self) -> Edge2W<'_, Edge2_100mSpec> {
        Edge2W::new(self, 0)
    }
}
#[doc = "RGMII 25 MHz second edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`edge2_100m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge2_100m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Edge2_100mSpec;
impl crate::RegisterSpec for Edge2_100mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edge2_100m::R`](R) reader structure"]
impl crate::Readable for Edge2_100mSpec {}
#[doc = "`write(|w| ..)` method takes [`edge2_100m::W`](W) writer structure"]
impl crate::Writable for Edge2_100mSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EDGE2_100M to value 0"]
impl crate::Resettable for Edge2_100mSpec {}
