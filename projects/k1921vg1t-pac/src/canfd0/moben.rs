#[doc = "Register `MOBEN` reader"]
pub type R = crate::R<MobenSpec>;
#[doc = "Field `ENMOB0` reader - Message Object 0 is Enabled"]
pub type Enmob0R = crate::BitReader;
#[doc = "Field `ENMOB1` reader - Message Object 1 is Enabled"]
pub type Enmob1R = crate::BitReader;
#[doc = "Field `ENMOB2` reader - Message Object 2 is Enabled"]
pub type Enmob2R = crate::BitReader;
#[doc = "Field `ENMOB3` reader - Message Object 3 is Enabled"]
pub type Enmob3R = crate::BitReader;
#[doc = "Field `ENMOB4` reader - Message Object 4 is Enabled"]
pub type Enmob4R = crate::BitReader;
#[doc = "Field `ENMOB5` reader - Message Object 5 is Enabled"]
pub type Enmob5R = crate::BitReader;
#[doc = "Field `ENMOB6` reader - Message Object 6 is Enabled"]
pub type Enmob6R = crate::BitReader;
#[doc = "Field `ENMOB7` reader - Message Object 7 is Enabled"]
pub type Enmob7R = crate::BitReader;
#[doc = "Field `ENMOB8` reader - Message Object 8 is Enabled"]
pub type Enmob8R = crate::BitReader;
#[doc = "Field `ENMOB9` reader - Message Object 9 is Enabled"]
pub type Enmob9R = crate::BitReader;
#[doc = "Field `ENMOB10` reader - Message Object 10 is Enabled"]
pub type Enmob10R = crate::BitReader;
#[doc = "Field `ENMOB11` reader - Message Object 11 is Enabled"]
pub type Enmob11R = crate::BitReader;
#[doc = "Field `ENMOB12` reader - Message Object 12 is Enabled"]
pub type Enmob12R = crate::BitReader;
#[doc = "Field `ENMOB13` reader - Message Object 13 is Enabled"]
pub type Enmob13R = crate::BitReader;
#[doc = "Field `ENMOB14` reader - Message Object 14 is Enabled"]
pub type Enmob14R = crate::BitReader;
#[doc = "Field `ENMOB15` reader - Message Object 15 is Enabled"]
pub type Enmob15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Message Object 0 is Enabled"]
    #[inline(always)]
    pub fn enmob0(&self) -> Enmob0R {
        Enmob0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Message Object 1 is Enabled"]
    #[inline(always)]
    pub fn enmob1(&self) -> Enmob1R {
        Enmob1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message Object 2 is Enabled"]
    #[inline(always)]
    pub fn enmob2(&self) -> Enmob2R {
        Enmob2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message Object 3 is Enabled"]
    #[inline(always)]
    pub fn enmob3(&self) -> Enmob3R {
        Enmob3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Message Object 4 is Enabled"]
    #[inline(always)]
    pub fn enmob4(&self) -> Enmob4R {
        Enmob4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Message Object 5 is Enabled"]
    #[inline(always)]
    pub fn enmob5(&self) -> Enmob5R {
        Enmob5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Message Object 6 is Enabled"]
    #[inline(always)]
    pub fn enmob6(&self) -> Enmob6R {
        Enmob6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Message Object 7 is Enabled"]
    #[inline(always)]
    pub fn enmob7(&self) -> Enmob7R {
        Enmob7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Message Object 8 is Enabled"]
    #[inline(always)]
    pub fn enmob8(&self) -> Enmob8R {
        Enmob8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Message Object 9 is Enabled"]
    #[inline(always)]
    pub fn enmob9(&self) -> Enmob9R {
        Enmob9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Message Object 10 is Enabled"]
    #[inline(always)]
    pub fn enmob10(&self) -> Enmob10R {
        Enmob10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Object 11 is Enabled"]
    #[inline(always)]
    pub fn enmob11(&self) -> Enmob11R {
        Enmob11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Message Object 12 is Enabled"]
    #[inline(always)]
    pub fn enmob12(&self) -> Enmob12R {
        Enmob12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Message Object 13 is Enabled"]
    #[inline(always)]
    pub fn enmob13(&self) -> Enmob13R {
        Enmob13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Object 14 is Enabled"]
    #[inline(always)]
    pub fn enmob14(&self) -> Enmob14R {
        Enmob14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Message Object 15 is Enabled"]
    #[inline(always)]
    pub fn enmob15(&self) -> Enmob15R {
        Enmob15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "CAN Enable MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`moben::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MobenSpec;
impl crate::RegisterSpec for MobenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moben::R`](R) reader structure"]
impl crate::Readable for MobenSpec {}
#[doc = "`reset()` method sets MOBEN to value 0"]
impl crate::Resettable for MobenSpec {}
