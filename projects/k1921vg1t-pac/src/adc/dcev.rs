#[doc = "Register `DCEV` writer"]
pub type W = crate::W<DcevSpec>;
#[doc = "Field `DCEV0` writer - Digital compare event 0"]
pub type Dcev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV1` writer - Digital compare event 1"]
pub type Dcev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV2` writer - Digital compare event 2"]
pub type Dcev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV3` writer - Digital compare event 3"]
pub type Dcev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV4` writer - Digital compare event 4"]
pub type Dcev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV5` writer - Digital compare event 5"]
pub type Dcev5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV6` writer - Digital compare event 6"]
pub type Dcev6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV7` writer - Digital compare event 7"]
pub type Dcev7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV8` writer - Digital compare event 8"]
pub type Dcev8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV9` writer - Digital compare event 9"]
pub type Dcev9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV10` writer - Digital compare event 10"]
pub type Dcev10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV11` writer - Digital compare event 11"]
pub type Dcev11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV12` writer - Digital compare event 12"]
pub type Dcev12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV13` writer - Digital compare event 13"]
pub type Dcev13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV14` writer - Digital compare event 14"]
pub type Dcev14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV15` writer - Digital compare event 15"]
pub type Dcev15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV16` writer - Digital compare event 16"]
pub type Dcev16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV17` writer - Digital compare event 17"]
pub type Dcev17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV18` writer - Digital compare event 18"]
pub type Dcev18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV19` writer - Digital compare event 19"]
pub type Dcev19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV20` writer - Digital compare event 20"]
pub type Dcev20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV21` writer - Digital compare event 21"]
pub type Dcev21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV22` writer - Digital compare event 22"]
pub type Dcev22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEV23` writer - Digital compare event 23"]
pub type Dcev23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Digital compare event 0"]
    #[inline(always)]
    pub fn dcev0(&mut self) -> Dcev0W<'_, DcevSpec> {
        Dcev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Digital compare event 1"]
    #[inline(always)]
    pub fn dcev1(&mut self) -> Dcev1W<'_, DcevSpec> {
        Dcev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Digital compare event 2"]
    #[inline(always)]
    pub fn dcev2(&mut self) -> Dcev2W<'_, DcevSpec> {
        Dcev2W::new(self, 2)
    }
    #[doc = "Bit 3 - Digital compare event 3"]
    #[inline(always)]
    pub fn dcev3(&mut self) -> Dcev3W<'_, DcevSpec> {
        Dcev3W::new(self, 3)
    }
    #[doc = "Bit 4 - Digital compare event 4"]
    #[inline(always)]
    pub fn dcev4(&mut self) -> Dcev4W<'_, DcevSpec> {
        Dcev4W::new(self, 4)
    }
    #[doc = "Bit 5 - Digital compare event 5"]
    #[inline(always)]
    pub fn dcev5(&mut self) -> Dcev5W<'_, DcevSpec> {
        Dcev5W::new(self, 5)
    }
    #[doc = "Bit 6 - Digital compare event 6"]
    #[inline(always)]
    pub fn dcev6(&mut self) -> Dcev6W<'_, DcevSpec> {
        Dcev6W::new(self, 6)
    }
    #[doc = "Bit 7 - Digital compare event 7"]
    #[inline(always)]
    pub fn dcev7(&mut self) -> Dcev7W<'_, DcevSpec> {
        Dcev7W::new(self, 7)
    }
    #[doc = "Bit 8 - Digital compare event 8"]
    #[inline(always)]
    pub fn dcev8(&mut self) -> Dcev8W<'_, DcevSpec> {
        Dcev8W::new(self, 8)
    }
    #[doc = "Bit 9 - Digital compare event 9"]
    #[inline(always)]
    pub fn dcev9(&mut self) -> Dcev9W<'_, DcevSpec> {
        Dcev9W::new(self, 9)
    }
    #[doc = "Bit 10 - Digital compare event 10"]
    #[inline(always)]
    pub fn dcev10(&mut self) -> Dcev10W<'_, DcevSpec> {
        Dcev10W::new(self, 10)
    }
    #[doc = "Bit 11 - Digital compare event 11"]
    #[inline(always)]
    pub fn dcev11(&mut self) -> Dcev11W<'_, DcevSpec> {
        Dcev11W::new(self, 11)
    }
    #[doc = "Bit 12 - Digital compare event 12"]
    #[inline(always)]
    pub fn dcev12(&mut self) -> Dcev12W<'_, DcevSpec> {
        Dcev12W::new(self, 12)
    }
    #[doc = "Bit 13 - Digital compare event 13"]
    #[inline(always)]
    pub fn dcev13(&mut self) -> Dcev13W<'_, DcevSpec> {
        Dcev13W::new(self, 13)
    }
    #[doc = "Bit 14 - Digital compare event 14"]
    #[inline(always)]
    pub fn dcev14(&mut self) -> Dcev14W<'_, DcevSpec> {
        Dcev14W::new(self, 14)
    }
    #[doc = "Bit 15 - Digital compare event 15"]
    #[inline(always)]
    pub fn dcev15(&mut self) -> Dcev15W<'_, DcevSpec> {
        Dcev15W::new(self, 15)
    }
    #[doc = "Bit 16 - Digital compare event 16"]
    #[inline(always)]
    pub fn dcev16(&mut self) -> Dcev16W<'_, DcevSpec> {
        Dcev16W::new(self, 16)
    }
    #[doc = "Bit 17 - Digital compare event 17"]
    #[inline(always)]
    pub fn dcev17(&mut self) -> Dcev17W<'_, DcevSpec> {
        Dcev17W::new(self, 17)
    }
    #[doc = "Bit 18 - Digital compare event 18"]
    #[inline(always)]
    pub fn dcev18(&mut self) -> Dcev18W<'_, DcevSpec> {
        Dcev18W::new(self, 18)
    }
    #[doc = "Bit 19 - Digital compare event 19"]
    #[inline(always)]
    pub fn dcev19(&mut self) -> Dcev19W<'_, DcevSpec> {
        Dcev19W::new(self, 19)
    }
    #[doc = "Bit 20 - Digital compare event 20"]
    #[inline(always)]
    pub fn dcev20(&mut self) -> Dcev20W<'_, DcevSpec> {
        Dcev20W::new(self, 20)
    }
    #[doc = "Bit 21 - Digital compare event 21"]
    #[inline(always)]
    pub fn dcev21(&mut self) -> Dcev21W<'_, DcevSpec> {
        Dcev21W::new(self, 21)
    }
    #[doc = "Bit 22 - Digital compare event 22"]
    #[inline(always)]
    pub fn dcev22(&mut self) -> Dcev22W<'_, DcevSpec> {
        Dcev22W::new(self, 22)
    }
    #[doc = "Bit 23 - Digital compare event 23"]
    #[inline(always)]
    pub fn dcev23(&mut self) -> Dcev23W<'_, DcevSpec> {
        Dcev23W::new(self, 23)
    }
}
#[doc = "Digital comparator compare event status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcev::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcevSpec;
impl crate::RegisterSpec for DcevSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dcev::W`](W) writer structure"]
impl crate::Writable for DcevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCEV to value 0"]
impl crate::Resettable for DcevSpec {}
