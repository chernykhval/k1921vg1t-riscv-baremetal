#[doc = "Register `MOBFDEN` reader"]
pub type R = crate::R<MobfdenSpec>;
#[doc = "Register `MOBFDEN` writer"]
pub type W = crate::W<MobfdenSpec>;
#[doc = "Field `FDMOBEN0` reader - Message Object 0 FD Mode Enable"]
pub type Fdmoben0R = crate::BitReader;
#[doc = "Field `FDMOBEN0` writer - Message Object 0 FD Mode Enable"]
pub type Fdmoben0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN1` reader - Message Object 1 FD Mode Enable"]
pub type Fdmoben1R = crate::BitReader;
#[doc = "Field `FDMOBEN1` writer - Message Object 1 FD Mode Enable"]
pub type Fdmoben1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN2` reader - Message Object 2 FD Mode Enable"]
pub type Fdmoben2R = crate::BitReader;
#[doc = "Field `FDMOBEN2` writer - Message Object 2 FD Mode Enable"]
pub type Fdmoben2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN3` reader - Message Object 3 FD Mode Enable"]
pub type Fdmoben3R = crate::BitReader;
#[doc = "Field `FDMOBEN3` writer - Message Object 3 FD Mode Enable"]
pub type Fdmoben3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN4` reader - Message Object 4 FD Mode Enable"]
pub type Fdmoben4R = crate::BitReader;
#[doc = "Field `FDMOBEN4` writer - Message Object 4 FD Mode Enable"]
pub type Fdmoben4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN5` reader - Message Object 5 FD Mode Enable"]
pub type Fdmoben5R = crate::BitReader;
#[doc = "Field `FDMOBEN5` writer - Message Object 5 FD Mode Enable"]
pub type Fdmoben5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN6` reader - Message Object 6 FD Mode Enable"]
pub type Fdmoben6R = crate::BitReader;
#[doc = "Field `FDMOBEN6` writer - Message Object 6 FD Mode Enable"]
pub type Fdmoben6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN7` reader - Message Object 7 FD Mode Enable"]
pub type Fdmoben7R = crate::BitReader;
#[doc = "Field `FDMOBEN7` writer - Message Object 7 FD Mode Enable"]
pub type Fdmoben7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN8` reader - Message Object 8 FD Mode Enable"]
pub type Fdmoben8R = crate::BitReader;
#[doc = "Field `FDMOBEN8` writer - Message Object 8 FD Mode Enable"]
pub type Fdmoben8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN9` reader - Message Object 9 FD Mode Enable"]
pub type Fdmoben9R = crate::BitReader;
#[doc = "Field `FDMOBEN9` writer - Message Object 9 FD Mode Enable"]
pub type Fdmoben9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN10` reader - Message Object 10 FD Mode Enable"]
pub type Fdmoben10R = crate::BitReader;
#[doc = "Field `FDMOBEN10` writer - Message Object 10 FD Mode Enable"]
pub type Fdmoben10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN11` reader - Message Object 11 FD Mode Enable"]
pub type Fdmoben11R = crate::BitReader;
#[doc = "Field `FDMOBEN11` writer - Message Object 11 FD Mode Enable"]
pub type Fdmoben11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN12` reader - Message Object 12 FD Mode Enable"]
pub type Fdmoben12R = crate::BitReader;
#[doc = "Field `FDMOBEN12` writer - Message Object 12 FD Mode Enable"]
pub type Fdmoben12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN13` reader - Message Object 13 FD Mode Enable"]
pub type Fdmoben13R = crate::BitReader;
#[doc = "Field `FDMOBEN13` writer - Message Object 13 FD Mode Enable"]
pub type Fdmoben13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN14` reader - Message Object 14 FD Mode Enable"]
pub type Fdmoben14R = crate::BitReader;
#[doc = "Field `FDMOBEN14` writer - Message Object 14 FD Mode Enable"]
pub type Fdmoben14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOBEN15` reader - Message Object 15 FD Mode Enable"]
pub type Fdmoben15R = crate::BitReader;
#[doc = "Field `FDMOBEN15` writer - Message Object 15 FD Mode Enable"]
pub type Fdmoben15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message Object 0 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben0(&self) -> Fdmoben0R {
        Fdmoben0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Message Object 1 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben1(&self) -> Fdmoben1R {
        Fdmoben1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message Object 2 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben2(&self) -> Fdmoben2R {
        Fdmoben2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message Object 3 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben3(&self) -> Fdmoben3R {
        Fdmoben3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Object 4 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben4(&self) -> Fdmoben4R {
        Fdmoben4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Message Object 5 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben5(&self) -> Fdmoben5R {
        Fdmoben5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Message Object 6 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben6(&self) -> Fdmoben6R {
        Fdmoben6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Message Object 7 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben7(&self) -> Fdmoben7R {
        Fdmoben7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Message Object 8 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben8(&self) -> Fdmoben8R {
        Fdmoben8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Message Object 9 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben9(&self) -> Fdmoben9R {
        Fdmoben9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Message Object 10 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben10(&self) -> Fdmoben10R {
        Fdmoben10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Object 11 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben11(&self) -> Fdmoben11R {
        Fdmoben11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Message Object 12 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben12(&self) -> Fdmoben12R {
        Fdmoben12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Message Object 13 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben13(&self) -> Fdmoben13R {
        Fdmoben13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Object 14 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben14(&self) -> Fdmoben14R {
        Fdmoben14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Message Object 15 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben15(&self) -> Fdmoben15R {
        Fdmoben15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Message Object 0 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben0(&mut self) -> Fdmoben0W<'_, MobfdenSpec> {
        Fdmoben0W::new(self, 0)
    }
    #[doc = "Bit 1 - Message Object 1 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben1(&mut self) -> Fdmoben1W<'_, MobfdenSpec> {
        Fdmoben1W::new(self, 1)
    }
    #[doc = "Bit 2 - Message Object 2 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben2(&mut self) -> Fdmoben2W<'_, MobfdenSpec> {
        Fdmoben2W::new(self, 2)
    }
    #[doc = "Bit 3 - Message Object 3 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben3(&mut self) -> Fdmoben3W<'_, MobfdenSpec> {
        Fdmoben3W::new(self, 3)
    }
    #[doc = "Bit 4 - Message Object 4 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben4(&mut self) -> Fdmoben4W<'_, MobfdenSpec> {
        Fdmoben4W::new(self, 4)
    }
    #[doc = "Bit 5 - Message Object 5 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben5(&mut self) -> Fdmoben5W<'_, MobfdenSpec> {
        Fdmoben5W::new(self, 5)
    }
    #[doc = "Bit 6 - Message Object 6 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben6(&mut self) -> Fdmoben6W<'_, MobfdenSpec> {
        Fdmoben6W::new(self, 6)
    }
    #[doc = "Bit 7 - Message Object 7 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben7(&mut self) -> Fdmoben7W<'_, MobfdenSpec> {
        Fdmoben7W::new(self, 7)
    }
    #[doc = "Bit 8 - Message Object 8 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben8(&mut self) -> Fdmoben8W<'_, MobfdenSpec> {
        Fdmoben8W::new(self, 8)
    }
    #[doc = "Bit 9 - Message Object 9 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben9(&mut self) -> Fdmoben9W<'_, MobfdenSpec> {
        Fdmoben9W::new(self, 9)
    }
    #[doc = "Bit 10 - Message Object 10 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben10(&mut self) -> Fdmoben10W<'_, MobfdenSpec> {
        Fdmoben10W::new(self, 10)
    }
    #[doc = "Bit 11 - Message Object 11 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben11(&mut self) -> Fdmoben11W<'_, MobfdenSpec> {
        Fdmoben11W::new(self, 11)
    }
    #[doc = "Bit 12 - Message Object 12 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben12(&mut self) -> Fdmoben12W<'_, MobfdenSpec> {
        Fdmoben12W::new(self, 12)
    }
    #[doc = "Bit 13 - Message Object 13 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben13(&mut self) -> Fdmoben13W<'_, MobfdenSpec> {
        Fdmoben13W::new(self, 13)
    }
    #[doc = "Bit 14 - Message Object 14 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben14(&mut self) -> Fdmoben14W<'_, MobfdenSpec> {
        Fdmoben14W::new(self, 14)
    }
    #[doc = "Bit 15 - Message Object 15 FD Mode Enable"]
    #[inline(always)]
    pub fn fdmoben15(&mut self) -> Fdmoben15W<'_, MobfdenSpec> {
        Fdmoben15W::new(self, 15)
    }
}
#[doc = "CAN MOb FD Mode Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobfden::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobfden::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MobfdenSpec;
impl crate::RegisterSpec for MobfdenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mobfden::R`](R) reader structure"]
impl crate::Readable for MobfdenSpec {}
#[doc = "`write(|w| ..)` method takes [`mobfden::W`](W) writer structure"]
impl crate::Writable for MobfdenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOBFDEN to value 0"]
impl crate::Resettable for MobfdenSpec {}
