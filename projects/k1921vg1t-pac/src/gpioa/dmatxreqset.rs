#[doc = "Register `DMATXREQSET` reader"]
pub type R = crate::R<DmatxreqsetSpec>;
#[doc = "Register `DMATXREQSET` writer"]
pub type W = crate::W<DmatxreqsetSpec>;
#[doc = "Field `PIN0` reader - DMA request enable for pin 0"]
pub type Pin0R = crate::BitReader;
#[doc = "Field `PIN0` writer - DMA request enable for pin 0"]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN1` reader - DMA request enable for pin 1"]
pub type Pin1R = crate::BitReader;
#[doc = "Field `PIN1` writer - DMA request enable for pin 1"]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN2` reader - DMA request enable for pin 2"]
pub type Pin2R = crate::BitReader;
#[doc = "Field `PIN2` writer - DMA request enable for pin 2"]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN3` reader - DMA request enable for pin 3"]
pub type Pin3R = crate::BitReader;
#[doc = "Field `PIN3` writer - DMA request enable for pin 3"]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN4` reader - DMA request enable for pin 4"]
pub type Pin4R = crate::BitReader;
#[doc = "Field `PIN4` writer - DMA request enable for pin 4"]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN5` reader - DMA request enable for pin 5"]
pub type Pin5R = crate::BitReader;
#[doc = "Field `PIN5` writer - DMA request enable for pin 5"]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN6` reader - DMA request enable for pin 6"]
pub type Pin6R = crate::BitReader;
#[doc = "Field `PIN6` writer - DMA request enable for pin 6"]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN7` reader - DMA request enable for pin 7"]
pub type Pin7R = crate::BitReader;
#[doc = "Field `PIN7` writer - DMA request enable for pin 7"]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN8` reader - DMA request enable for pin 8"]
pub type Pin8R = crate::BitReader;
#[doc = "Field `PIN8` writer - DMA request enable for pin 8"]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN9` reader - DMA request enable for pin 9"]
pub type Pin9R = crate::BitReader;
#[doc = "Field `PIN9` writer - DMA request enable for pin 9"]
pub type Pin9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN10` reader - DMA request enable for pin 10"]
pub type Pin10R = crate::BitReader;
#[doc = "Field `PIN10` writer - DMA request enable for pin 10"]
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN11` reader - DMA request enable for pin 11"]
pub type Pin11R = crate::BitReader;
#[doc = "Field `PIN11` writer - DMA request enable for pin 11"]
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN12` reader - DMA request enable for pin 12"]
pub type Pin12R = crate::BitReader;
#[doc = "Field `PIN12` writer - DMA request enable for pin 12"]
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN13` reader - DMA request enable for pin 13"]
pub type Pin13R = crate::BitReader;
#[doc = "Field `PIN13` writer - DMA request enable for pin 13"]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN14` reader - DMA request enable for pin 14"]
pub type Pin14R = crate::BitReader;
#[doc = "Field `PIN14` writer - DMA request enable for pin 14"]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN15` reader - DMA request enable for pin 15"]
pub type Pin15R = crate::BitReader;
#[doc = "Field `PIN15` writer - DMA request enable for pin 15"]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA request enable for pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA request enable for pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA request enable for pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request enable for pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA request enable for pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA request enable for pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA request enable for pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA request enable for pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA request enable for pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA request enable for pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA request enable for pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA request enable for pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA request enable for pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA request enable for pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA request enable for pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA request enable for pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA request enable for pin 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, DmatxreqsetSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA request enable for pin 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, DmatxreqsetSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA request enable for pin 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<'_, DmatxreqsetSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA request enable for pin 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, DmatxreqsetSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA request enable for pin 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> Pin4W<'_, DmatxreqsetSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA request enable for pin 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> Pin5W<'_, DmatxreqsetSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA request enable for pin 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, DmatxreqsetSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA request enable for pin 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, DmatxreqsetSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMA request enable for pin 8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> Pin8W<'_, DmatxreqsetSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA request enable for pin 9"]
    #[inline(always)]
    pub fn pin9(&mut self) -> Pin9W<'_, DmatxreqsetSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA request enable for pin 10"]
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<'_, DmatxreqsetSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA request enable for pin 11"]
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<'_, DmatxreqsetSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA request enable for pin 12"]
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<'_, DmatxreqsetSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13 - DMA request enable for pin 13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<'_, DmatxreqsetSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - DMA request enable for pin 14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<'_, DmatxreqsetSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - DMA request enable for pin 15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<'_, DmatxreqsetSpec> {
        Pin15W::new(self, 15)
    }
}
#[doc = "DMA TX request generator enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxreqset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatxreqset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatxreqsetSpec;
impl crate::RegisterSpec for DmatxreqsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxreqset::R`](R) reader structure"]
impl crate::Readable for DmatxreqsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatxreqset::W`](W) writer structure"]
impl crate::Writable for DmatxreqsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATXREQSET to value 0"]
impl crate::Resettable for DmatxreqsetSpec {}
