#[doc = "Register `PWMSYNC` reader"]
pub type R = crate::R<PwmsyncSpec>;
#[doc = "Register `PWMSYNC` writer"]
pub type W = crate::W<PwmsyncSpec>;
#[doc = "Field `SWSYNC` reader - SYNCI pulse software emulation on PWM0"]
pub type SwsyncR = crate::BitReader;
#[doc = "Field `SWSYNC` writer - SYNCI pulse software emulation on PWM0"]
pub type SwsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL1` reader - PWM sync scheme control"]
pub type Pwmsyncsel1R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL1` writer - PWM sync scheme control"]
pub type Pwmsyncsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL2` reader - PWM sync scheme control"]
pub type Pwmsyncsel2R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL2` writer - PWM sync scheme control"]
pub type Pwmsyncsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL3` reader - PWM sync scheme control"]
pub type Pwmsyncsel3R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL3` writer - PWM sync scheme control"]
pub type Pwmsyncsel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL4` reader - PWM sync scheme control"]
pub type Pwmsyncsel4R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL4` writer - PWM sync scheme control"]
pub type Pwmsyncsel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL5` reader - PWM sync scheme control"]
pub type Pwmsyncsel5R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL5` writer - PWM sync scheme control"]
pub type Pwmsyncsel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL6` reader - PWM sync scheme control"]
pub type Pwmsyncsel6R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL6` writer - PWM sync scheme control"]
pub type Pwmsyncsel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL7` reader - PWM sync scheme control"]
pub type Pwmsyncsel7R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL7` writer - PWM sync scheme control"]
pub type Pwmsyncsel7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL8` reader - PWM sync scheme control"]
pub type Pwmsyncsel8R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL8` writer - PWM sync scheme control"]
pub type Pwmsyncsel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL9` reader - PWM sync scheme control"]
pub type Pwmsyncsel9R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL9` writer - PWM sync scheme control"]
pub type Pwmsyncsel9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL10` reader - PWM sync scheme control"]
pub type Pwmsyncsel10R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL10` writer - PWM sync scheme control"]
pub type Pwmsyncsel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL11` reader - PWM sync scheme control"]
pub type Pwmsyncsel11R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL11` writer - PWM sync scheme control"]
pub type Pwmsyncsel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL12` reader - PWM sync scheme control"]
pub type Pwmsyncsel12R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL12` writer - PWM sync scheme control"]
pub type Pwmsyncsel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL13` reader - PWM sync scheme control"]
pub type Pwmsyncsel13R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL13` writer - PWM sync scheme control"]
pub type Pwmsyncsel13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL14` reader - PWM sync scheme control"]
pub type Pwmsyncsel14R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL14` writer - PWM sync scheme control"]
pub type Pwmsyncsel14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNCSEL15` reader - PWM sync scheme control"]
pub type Pwmsyncsel15R = crate::BitReader;
#[doc = "Field `PWMSYNCSEL15` writer - PWM sync scheme control"]
pub type Pwmsyncsel15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYNCI pulse software emulation on PWM0"]
    #[inline(always)]
    pub fn swsync(&self) -> SwsyncR {
        SwsyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel1(&self) -> Pwmsyncsel1R {
        Pwmsyncsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel2(&self) -> Pwmsyncsel2R {
        Pwmsyncsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel3(&self) -> Pwmsyncsel3R {
        Pwmsyncsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel4(&self) -> Pwmsyncsel4R {
        Pwmsyncsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel5(&self) -> Pwmsyncsel5R {
        Pwmsyncsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel6(&self) -> Pwmsyncsel6R {
        Pwmsyncsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel7(&self) -> Pwmsyncsel7R {
        Pwmsyncsel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel8(&self) -> Pwmsyncsel8R {
        Pwmsyncsel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel9(&self) -> Pwmsyncsel9R {
        Pwmsyncsel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel10(&self) -> Pwmsyncsel10R {
        Pwmsyncsel10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel11(&self) -> Pwmsyncsel11R {
        Pwmsyncsel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel12(&self) -> Pwmsyncsel12R {
        Pwmsyncsel12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel13(&self) -> Pwmsyncsel13R {
        Pwmsyncsel13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel14(&self) -> Pwmsyncsel14R {
        Pwmsyncsel14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel15(&self) -> Pwmsyncsel15R {
        Pwmsyncsel15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYNCI pulse software emulation on PWM0"]
    #[inline(always)]
    pub fn swsync(&mut self) -> SwsyncW<'_, PwmsyncSpec> {
        SwsyncW::new(self, 0)
    }
    #[doc = "Bit 1 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel1(&mut self) -> Pwmsyncsel1W<'_, PwmsyncSpec> {
        Pwmsyncsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel2(&mut self) -> Pwmsyncsel2W<'_, PwmsyncSpec> {
        Pwmsyncsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel3(&mut self) -> Pwmsyncsel3W<'_, PwmsyncSpec> {
        Pwmsyncsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel4(&mut self) -> Pwmsyncsel4W<'_, PwmsyncSpec> {
        Pwmsyncsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel5(&mut self) -> Pwmsyncsel5W<'_, PwmsyncSpec> {
        Pwmsyncsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel6(&mut self) -> Pwmsyncsel6W<'_, PwmsyncSpec> {
        Pwmsyncsel6W::new(self, 6)
    }
    #[doc = "Bit 7 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel7(&mut self) -> Pwmsyncsel7W<'_, PwmsyncSpec> {
        Pwmsyncsel7W::new(self, 7)
    }
    #[doc = "Bit 8 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel8(&mut self) -> Pwmsyncsel8W<'_, PwmsyncSpec> {
        Pwmsyncsel8W::new(self, 8)
    }
    #[doc = "Bit 9 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel9(&mut self) -> Pwmsyncsel9W<'_, PwmsyncSpec> {
        Pwmsyncsel9W::new(self, 9)
    }
    #[doc = "Bit 10 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel10(&mut self) -> Pwmsyncsel10W<'_, PwmsyncSpec> {
        Pwmsyncsel10W::new(self, 10)
    }
    #[doc = "Bit 11 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel11(&mut self) -> Pwmsyncsel11W<'_, PwmsyncSpec> {
        Pwmsyncsel11W::new(self, 11)
    }
    #[doc = "Bit 12 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel12(&mut self) -> Pwmsyncsel12W<'_, PwmsyncSpec> {
        Pwmsyncsel12W::new(self, 12)
    }
    #[doc = "Bit 13 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel13(&mut self) -> Pwmsyncsel13W<'_, PwmsyncSpec> {
        Pwmsyncsel13W::new(self, 13)
    }
    #[doc = "Bit 14 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel14(&mut self) -> Pwmsyncsel14W<'_, PwmsyncSpec> {
        Pwmsyncsel14W::new(self, 14)
    }
    #[doc = "Bit 15 - PWM sync scheme control"]
    #[inline(always)]
    pub fn pwmsyncsel15(&mut self) -> Pwmsyncsel15W<'_, PwmsyncSpec> {
        Pwmsyncsel15W::new(self, 15)
    }
}
#[doc = "Sync PWM register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmsync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmsync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmsyncSpec;
impl crate::RegisterSpec for PwmsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmsync::R`](R) reader structure"]
impl crate::Readable for PwmsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmsync::W`](W) writer structure"]
impl crate::Writable for PwmsyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMSYNC to value 0"]
impl crate::Resettable for PwmsyncSpec {}
