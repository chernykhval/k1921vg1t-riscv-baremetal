#[doc = "Register `MOBIE` reader"]
pub type R = crate::R<MobieSpec>;
#[doc = "Register `MOBIE` writer"]
pub type W = crate::W<MobieSpec>;
#[doc = "Field `IEMOB0` reader - Message Object 0 Interrupt Enable"]
pub type Iemob0R = crate::BitReader;
#[doc = "Field `IEMOB0` writer - Message Object 0 Interrupt Enable"]
pub type Iemob0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB1` reader - Message Object 1 Interrupt Enable"]
pub type Iemob1R = crate::BitReader;
#[doc = "Field `IEMOB1` writer - Message Object 1 Interrupt Enable"]
pub type Iemob1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB2` reader - Message Object 2 Interrupt Enable"]
pub type Iemob2R = crate::BitReader;
#[doc = "Field `IEMOB2` writer - Message Object 2 Interrupt Enable"]
pub type Iemob2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB3` reader - Message Object 3 Interrupt Enable"]
pub type Iemob3R = crate::BitReader;
#[doc = "Field `IEMOB3` writer - Message Object 3 Interrupt Enable"]
pub type Iemob3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB4` reader - Message Object 4 Interrupt Enable"]
pub type Iemob4R = crate::BitReader;
#[doc = "Field `IEMOB4` writer - Message Object 4 Interrupt Enable"]
pub type Iemob4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB5` reader - Message Object 5 Interrupt Enable"]
pub type Iemob5R = crate::BitReader;
#[doc = "Field `IEMOB5` writer - Message Object 5 Interrupt Enable"]
pub type Iemob5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB6` reader - Message Object 6 Interrupt Enable"]
pub type Iemob6R = crate::BitReader;
#[doc = "Field `IEMOB6` writer - Message Object 6 Interrupt Enable"]
pub type Iemob6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB7` reader - Message Object 7 Interrupt Enable"]
pub type Iemob7R = crate::BitReader;
#[doc = "Field `IEMOB7` writer - Message Object 7 Interrupt Enable"]
pub type Iemob7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB8` reader - Message Object 8 Interrupt Enable"]
pub type Iemob8R = crate::BitReader;
#[doc = "Field `IEMOB8` writer - Message Object 8 Interrupt Enable"]
pub type Iemob8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB9` reader - Message Object 9 Interrupt Enable"]
pub type Iemob9R = crate::BitReader;
#[doc = "Field `IEMOB9` writer - Message Object 9 Interrupt Enable"]
pub type Iemob9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB10` reader - Message Object 10 Interrupt Enable"]
pub type Iemob10R = crate::BitReader;
#[doc = "Field `IEMOB10` writer - Message Object 10 Interrupt Enable"]
pub type Iemob10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB11` reader - Message Object 11 Interrupt Enable"]
pub type Iemob11R = crate::BitReader;
#[doc = "Field `IEMOB11` writer - Message Object 11 Interrupt Enable"]
pub type Iemob11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB12` reader - Message Object 12 Interrupt Enable"]
pub type Iemob12R = crate::BitReader;
#[doc = "Field `IEMOB12` writer - Message Object 12 Interrupt Enable"]
pub type Iemob12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB13` reader - Message Object 13 Interrupt Enable"]
pub type Iemob13R = crate::BitReader;
#[doc = "Field `IEMOB13` writer - Message Object 13 Interrupt Enable"]
pub type Iemob13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB14` reader - Message Object 14 Interrupt Enable"]
pub type Iemob14R = crate::BitReader;
#[doc = "Field `IEMOB14` writer - Message Object 14 Interrupt Enable"]
pub type Iemob14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEMOB15` reader - Message Object 15 Interrupt Enable"]
pub type Iemob15R = crate::BitReader;
#[doc = "Field `IEMOB15` writer - Message Object 15 Interrupt Enable"]
pub type Iemob15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message Object 0 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob0(&self) -> Iemob0R {
        Iemob0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Message Object 1 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob1(&self) -> Iemob1R {
        Iemob1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message Object 2 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob2(&self) -> Iemob2R {
        Iemob2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message Object 3 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob3(&self) -> Iemob3R {
        Iemob3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Object 4 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob4(&self) -> Iemob4R {
        Iemob4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Message Object 5 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob5(&self) -> Iemob5R {
        Iemob5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Message Object 6 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob6(&self) -> Iemob6R {
        Iemob6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Message Object 7 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob7(&self) -> Iemob7R {
        Iemob7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Message Object 8 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob8(&self) -> Iemob8R {
        Iemob8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Message Object 9 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob9(&self) -> Iemob9R {
        Iemob9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Message Object 10 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob10(&self) -> Iemob10R {
        Iemob10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Object 11 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob11(&self) -> Iemob11R {
        Iemob11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Message Object 12 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob12(&self) -> Iemob12R {
        Iemob12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Message Object 13 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob13(&self) -> Iemob13R {
        Iemob13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Object 14 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob14(&self) -> Iemob14R {
        Iemob14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Message Object 15 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob15(&self) -> Iemob15R {
        Iemob15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Message Object 0 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob0(&mut self) -> Iemob0W<'_, MobieSpec> {
        Iemob0W::new(self, 0)
    }
    #[doc = "Bit 1 - Message Object 1 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob1(&mut self) -> Iemob1W<'_, MobieSpec> {
        Iemob1W::new(self, 1)
    }
    #[doc = "Bit 2 - Message Object 2 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob2(&mut self) -> Iemob2W<'_, MobieSpec> {
        Iemob2W::new(self, 2)
    }
    #[doc = "Bit 3 - Message Object 3 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob3(&mut self) -> Iemob3W<'_, MobieSpec> {
        Iemob3W::new(self, 3)
    }
    #[doc = "Bit 4 - Message Object 4 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob4(&mut self) -> Iemob4W<'_, MobieSpec> {
        Iemob4W::new(self, 4)
    }
    #[doc = "Bit 5 - Message Object 5 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob5(&mut self) -> Iemob5W<'_, MobieSpec> {
        Iemob5W::new(self, 5)
    }
    #[doc = "Bit 6 - Message Object 6 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob6(&mut self) -> Iemob6W<'_, MobieSpec> {
        Iemob6W::new(self, 6)
    }
    #[doc = "Bit 7 - Message Object 7 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob7(&mut self) -> Iemob7W<'_, MobieSpec> {
        Iemob7W::new(self, 7)
    }
    #[doc = "Bit 8 - Message Object 8 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob8(&mut self) -> Iemob8W<'_, MobieSpec> {
        Iemob8W::new(self, 8)
    }
    #[doc = "Bit 9 - Message Object 9 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob9(&mut self) -> Iemob9W<'_, MobieSpec> {
        Iemob9W::new(self, 9)
    }
    #[doc = "Bit 10 - Message Object 10 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob10(&mut self) -> Iemob10W<'_, MobieSpec> {
        Iemob10W::new(self, 10)
    }
    #[doc = "Bit 11 - Message Object 11 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob11(&mut self) -> Iemob11W<'_, MobieSpec> {
        Iemob11W::new(self, 11)
    }
    #[doc = "Bit 12 - Message Object 12 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob12(&mut self) -> Iemob12W<'_, MobieSpec> {
        Iemob12W::new(self, 12)
    }
    #[doc = "Bit 13 - Message Object 13 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob13(&mut self) -> Iemob13W<'_, MobieSpec> {
        Iemob13W::new(self, 13)
    }
    #[doc = "Bit 14 - Message Object 14 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob14(&mut self) -> Iemob14W<'_, MobieSpec> {
        Iemob14W::new(self, 14)
    }
    #[doc = "Bit 15 - Message Object 15 Interrupt Enable"]
    #[inline(always)]
    pub fn iemob15(&mut self) -> Iemob15W<'_, MobieSpec> {
        Iemob15W::new(self, 15)
    }
}
#[doc = "CAN Enable Interrupt MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MobieSpec;
impl crate::RegisterSpec for MobieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mobie::R`](R) reader structure"]
impl crate::Readable for MobieSpec {}
#[doc = "`write(|w| ..)` method takes [`mobie::W`](W) writer structure"]
impl crate::Writable for MobieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOBIE to value 0"]
impl crate::Resettable for MobieSpec {}
