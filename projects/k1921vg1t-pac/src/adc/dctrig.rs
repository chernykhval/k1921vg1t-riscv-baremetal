#[doc = "Register `DCTRIG` writer"]
pub type W = crate::W<DctrigSpec>;
#[doc = "Field `TOS0` writer - DC 0 output trigger status"]
pub type Tos0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS1` writer - DC 1 output trigger status"]
pub type Tos1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS2` writer - DC 2 output trigger status"]
pub type Tos2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS3` writer - DC 3 output trigger status"]
pub type Tos3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS4` writer - DC 4 output trigger status"]
pub type Tos4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS5` writer - DC 5 output trigger status"]
pub type Tos5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS6` writer - DC 6 output trigger status"]
pub type Tos6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS7` writer - DC 7 output trigger status"]
pub type Tos7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS8` writer - DC 8 output trigger status"]
pub type Tos8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS9` writer - DC 9 output trigger status"]
pub type Tos9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS10` writer - DC 10 output trigger status"]
pub type Tos10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS11` writer - DC 11 output trigger status"]
pub type Tos11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS12` writer - DC 12 output trigger status"]
pub type Tos12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS13` writer - DC 13 output trigger status"]
pub type Tos13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS14` writer - DC 14 output trigger status"]
pub type Tos14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS15` writer - DC 15 output trigger status"]
pub type Tos15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS16` writer - DC 16 output trigger status"]
pub type Tos16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS17` writer - DC 17 output trigger status"]
pub type Tos17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS18` writer - DC 18 output trigger status"]
pub type Tos18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS19` writer - DC 19 output trigger status"]
pub type Tos19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS20` writer - DC 20 output trigger status"]
pub type Tos20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS21` writer - DC 21 output trigger status"]
pub type Tos21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS22` writer - DC 22 output trigger status"]
pub type Tos22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS23` writer - DC 23 output trigger status"]
pub type Tos23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DC 0 output trigger status"]
    #[inline(always)]
    pub fn tos0(&mut self) -> Tos0W<'_, DctrigSpec> {
        Tos0W::new(self, 0)
    }
    #[doc = "Bit 1 - DC 1 output trigger status"]
    #[inline(always)]
    pub fn tos1(&mut self) -> Tos1W<'_, DctrigSpec> {
        Tos1W::new(self, 1)
    }
    #[doc = "Bit 2 - DC 2 output trigger status"]
    #[inline(always)]
    pub fn tos2(&mut self) -> Tos2W<'_, DctrigSpec> {
        Tos2W::new(self, 2)
    }
    #[doc = "Bit 3 - DC 3 output trigger status"]
    #[inline(always)]
    pub fn tos3(&mut self) -> Tos3W<'_, DctrigSpec> {
        Tos3W::new(self, 3)
    }
    #[doc = "Bit 4 - DC 4 output trigger status"]
    #[inline(always)]
    pub fn tos4(&mut self) -> Tos4W<'_, DctrigSpec> {
        Tos4W::new(self, 4)
    }
    #[doc = "Bit 5 - DC 5 output trigger status"]
    #[inline(always)]
    pub fn tos5(&mut self) -> Tos5W<'_, DctrigSpec> {
        Tos5W::new(self, 5)
    }
    #[doc = "Bit 6 - DC 6 output trigger status"]
    #[inline(always)]
    pub fn tos6(&mut self) -> Tos6W<'_, DctrigSpec> {
        Tos6W::new(self, 6)
    }
    #[doc = "Bit 7 - DC 7 output trigger status"]
    #[inline(always)]
    pub fn tos7(&mut self) -> Tos7W<'_, DctrigSpec> {
        Tos7W::new(self, 7)
    }
    #[doc = "Bit 8 - DC 8 output trigger status"]
    #[inline(always)]
    pub fn tos8(&mut self) -> Tos8W<'_, DctrigSpec> {
        Tos8W::new(self, 8)
    }
    #[doc = "Bit 9 - DC 9 output trigger status"]
    #[inline(always)]
    pub fn tos9(&mut self) -> Tos9W<'_, DctrigSpec> {
        Tos9W::new(self, 9)
    }
    #[doc = "Bit 10 - DC 10 output trigger status"]
    #[inline(always)]
    pub fn tos10(&mut self) -> Tos10W<'_, DctrigSpec> {
        Tos10W::new(self, 10)
    }
    #[doc = "Bit 11 - DC 11 output trigger status"]
    #[inline(always)]
    pub fn tos11(&mut self) -> Tos11W<'_, DctrigSpec> {
        Tos11W::new(self, 11)
    }
    #[doc = "Bit 12 - DC 12 output trigger status"]
    #[inline(always)]
    pub fn tos12(&mut self) -> Tos12W<'_, DctrigSpec> {
        Tos12W::new(self, 12)
    }
    #[doc = "Bit 13 - DC 13 output trigger status"]
    #[inline(always)]
    pub fn tos13(&mut self) -> Tos13W<'_, DctrigSpec> {
        Tos13W::new(self, 13)
    }
    #[doc = "Bit 14 - DC 14 output trigger status"]
    #[inline(always)]
    pub fn tos14(&mut self) -> Tos14W<'_, DctrigSpec> {
        Tos14W::new(self, 14)
    }
    #[doc = "Bit 15 - DC 15 output trigger status"]
    #[inline(always)]
    pub fn tos15(&mut self) -> Tos15W<'_, DctrigSpec> {
        Tos15W::new(self, 15)
    }
    #[doc = "Bit 16 - DC 16 output trigger status"]
    #[inline(always)]
    pub fn tos16(&mut self) -> Tos16W<'_, DctrigSpec> {
        Tos16W::new(self, 16)
    }
    #[doc = "Bit 17 - DC 17 output trigger status"]
    #[inline(always)]
    pub fn tos17(&mut self) -> Tos17W<'_, DctrigSpec> {
        Tos17W::new(self, 17)
    }
    #[doc = "Bit 18 - DC 18 output trigger status"]
    #[inline(always)]
    pub fn tos18(&mut self) -> Tos18W<'_, DctrigSpec> {
        Tos18W::new(self, 18)
    }
    #[doc = "Bit 19 - DC 19 output trigger status"]
    #[inline(always)]
    pub fn tos19(&mut self) -> Tos19W<'_, DctrigSpec> {
        Tos19W::new(self, 19)
    }
    #[doc = "Bit 20 - DC 20 output trigger status"]
    #[inline(always)]
    pub fn tos20(&mut self) -> Tos20W<'_, DctrigSpec> {
        Tos20W::new(self, 20)
    }
    #[doc = "Bit 21 - DC 21 output trigger status"]
    #[inline(always)]
    pub fn tos21(&mut self) -> Tos21W<'_, DctrigSpec> {
        Tos21W::new(self, 21)
    }
    #[doc = "Bit 22 - DC 22 output trigger status"]
    #[inline(always)]
    pub fn tos22(&mut self) -> Tos22W<'_, DctrigSpec> {
        Tos22W::new(self, 22)
    }
    #[doc = "Bit 23 - DC 23 output trigger status"]
    #[inline(always)]
    pub fn tos23(&mut self) -> Tos23W<'_, DctrigSpec> {
        Tos23W::new(self, 23)
    }
}
#[doc = "Digital comparator output trigger status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrig::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctrigSpec;
impl crate::RegisterSpec for DctrigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dctrig::W`](W) writer structure"]
impl crate::Writable for DctrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCTRIG to value 0"]
impl crate::Resettable for DctrigSpec {}
