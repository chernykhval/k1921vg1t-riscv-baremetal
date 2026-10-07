#[doc = "Register `MOBIF` reader"]
pub type R = crate::R<MobifSpec>;
#[doc = "Field `SIT0` reader - Interrupt Status 0"]
pub type Sit0R = crate::BitReader;
#[doc = "Field `SIT1` reader - Interrupt Status 1"]
pub type Sit1R = crate::BitReader;
#[doc = "Field `SIT2` reader - Interrupt Status 2"]
pub type Sit2R = crate::BitReader;
#[doc = "Field `SIT3` reader - Interrupt Status 3"]
pub type Sit3R = crate::BitReader;
#[doc = "Field `SIT4` reader - Interrupt Status 4"]
pub type Sit4R = crate::BitReader;
#[doc = "Field `SIT5` reader - Interrupt Status 5"]
pub type Sit5R = crate::BitReader;
#[doc = "Field `SIT6` reader - Interrupt Status 6"]
pub type Sit6R = crate::BitReader;
#[doc = "Field `SIT7` reader - Interrupt Status 7"]
pub type Sit7R = crate::BitReader;
#[doc = "Field `SIT8` reader - Interrupt Status 8"]
pub type Sit8R = crate::BitReader;
#[doc = "Field `SIT9` reader - Interrupt Status 9"]
pub type Sit9R = crate::BitReader;
#[doc = "Field `SIT10` reader - Interrupt Status 10"]
pub type Sit10R = crate::BitReader;
#[doc = "Field `SIT11` reader - Interrupt Status 11"]
pub type Sit11R = crate::BitReader;
#[doc = "Field `SIT12` reader - Interrupt Status 12"]
pub type Sit12R = crate::BitReader;
#[doc = "Field `SIT13` reader - Interrupt Status 13"]
pub type Sit13R = crate::BitReader;
#[doc = "Field `SIT14` reader - Interrupt Status 14"]
pub type Sit14R = crate::BitReader;
#[doc = "Field `SIT15` reader - Interrupt Status 15"]
pub type Sit15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status 0"]
    #[inline(always)]
    pub fn sit0(&self) -> Sit0R {
        Sit0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status 1"]
    #[inline(always)]
    pub fn sit1(&self) -> Sit1R {
        Sit1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status 2"]
    #[inline(always)]
    pub fn sit2(&self) -> Sit2R {
        Sit2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status 3"]
    #[inline(always)]
    pub fn sit3(&self) -> Sit3R {
        Sit3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status 4"]
    #[inline(always)]
    pub fn sit4(&self) -> Sit4R {
        Sit4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status 5"]
    #[inline(always)]
    pub fn sit5(&self) -> Sit5R {
        Sit5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status 6"]
    #[inline(always)]
    pub fn sit6(&self) -> Sit6R {
        Sit6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status 7"]
    #[inline(always)]
    pub fn sit7(&self) -> Sit7R {
        Sit7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status 8"]
    #[inline(always)]
    pub fn sit8(&self) -> Sit8R {
        Sit8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status 9"]
    #[inline(always)]
    pub fn sit9(&self) -> Sit9R {
        Sit9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status 10"]
    #[inline(always)]
    pub fn sit10(&self) -> Sit10R {
        Sit10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status 11"]
    #[inline(always)]
    pub fn sit11(&self) -> Sit11R {
        Sit11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status 12"]
    #[inline(always)]
    pub fn sit12(&self) -> Sit12R {
        Sit12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status 13"]
    #[inline(always)]
    pub fn sit13(&self) -> Sit13R {
        Sit13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status 14"]
    #[inline(always)]
    pub fn sit14(&self) -> Sit14R {
        Sit14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status 15"]
    #[inline(always)]
    pub fn sit15(&self) -> Sit15R {
        Sit15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "CAN Status Interrupt MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobif::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MobifSpec;
impl crate::RegisterSpec for MobifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mobif::R`](R) reader structure"]
impl crate::Readable for MobifSpec {}
#[doc = "`reset()` method sets MOBIF to value 0"]
impl crate::Resettable for MobifSpec {}
