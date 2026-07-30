#[doc = "Register `UI_CLR` writer"]
pub type W = crate::W<UiClrSpec>;
#[doc = "Field `I0` writer - Clear valid bit in the Upstream Item 0"]
pub type I0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I1` writer - Clear valid bit in the Upstream Item 1"]
pub type I1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2` writer - Clear valid bit in the Upstream Item 2"]
pub type I2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3` writer - Clear valid bit in the Upstream Item 3"]
pub type I3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear valid bit in the Upstream Item 0"]
    #[inline(always)]
    pub fn i0(&mut self) -> I0W<'_, UiClrSpec> {
        I0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear valid bit in the Upstream Item 1"]
    #[inline(always)]
    pub fn i1(&mut self) -> I1W<'_, UiClrSpec> {
        I1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear valid bit in the Upstream Item 2"]
    #[inline(always)]
    pub fn i2(&mut self) -> I2W<'_, UiClrSpec> {
        I2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear valid bit in the Upstream Item 3"]
    #[inline(always)]
    pub fn i3(&mut self) -> I3W<'_, UiClrSpec> {
        I3W::new(self, 3)
    }
}
#[doc = "Upstream Items valid clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ui_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UiClrSpec;
impl crate::RegisterSpec for UiClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ui_clr::W`](W) writer structure"]
impl crate::Writable for UiClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UI_CLR to value 0"]
impl crate::Resettable for UiClrSpec {}
