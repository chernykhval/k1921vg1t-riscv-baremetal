#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `SEQMIS0` reader - Sequencer 0 masked interrupt status"]
pub type Seqmis0R = crate::BitReader;
#[doc = "Field `SEQMIS1` reader - Sequencer 1 masked interrupt status"]
pub type Seqmis1R = crate::BitReader;
#[doc = "Field `SEQMIS2` reader - Sequencer 2 masked interrupt status"]
pub type Seqmis2R = crate::BitReader;
#[doc = "Field `SEQMIS3` reader - Sequencer 3 masked interrupt status"]
pub type Seqmis3R = crate::BitReader;
#[doc = "Field `SEQMIS4` reader - Sequencer 4 masked interrupt status"]
pub type Seqmis4R = crate::BitReader;
#[doc = "Field `SEQMIS5` reader - Sequencer 5 masked interrupt status"]
pub type Seqmis5R = crate::BitReader;
#[doc = "Field `SEQMIS6` reader - Sequencer 6 masked interrupt status"]
pub type Seqmis6R = crate::BitReader;
#[doc = "Field `SEQMIS7` reader - Sequencer 7 masked interrupt status"]
pub type Seqmis7R = crate::BitReader;
#[doc = "Field `DCMIS0` reader - DC 0 masked interrupt status"]
pub type Dcmis0R = crate::BitReader;
#[doc = "Field `DCMIS1` reader - DC 1 masked interrupt status"]
pub type Dcmis1R = crate::BitReader;
#[doc = "Field `DCMIS2` reader - DC 2 masked interrupt status"]
pub type Dcmis2R = crate::BitReader;
#[doc = "Field `DCMIS3` reader - DC 3 masked interrupt status"]
pub type Dcmis3R = crate::BitReader;
#[doc = "Field `DCMIS4` reader - DC 4 masked interrupt status"]
pub type Dcmis4R = crate::BitReader;
#[doc = "Field `DCMIS5` reader - DC 5 masked interrupt status"]
pub type Dcmis5R = crate::BitReader;
#[doc = "Field `DCMIS6` reader - DC 6 masked interrupt status"]
pub type Dcmis6R = crate::BitReader;
#[doc = "Field `DCMIS7` reader - DC 7 masked interrupt status"]
pub type Dcmis7R = crate::BitReader;
#[doc = "Field `DCMIS8` reader - DC 8 masked interrupt status"]
pub type Dcmis8R = crate::BitReader;
#[doc = "Field `DCMIS9` reader - DC 9 masked interrupt status"]
pub type Dcmis9R = crate::BitReader;
#[doc = "Field `DCMIS10` reader - DC 10 masked interrupt status"]
pub type Dcmis10R = crate::BitReader;
#[doc = "Field `DCMIS11` reader - DC 11 masked interrupt status"]
pub type Dcmis11R = crate::BitReader;
#[doc = "Field `DCMIS12` reader - DC 12 masked interrupt status"]
pub type Dcmis12R = crate::BitReader;
#[doc = "Field `DCMIS13` reader - DC 13 masked interrupt status"]
pub type Dcmis13R = crate::BitReader;
#[doc = "Field `DCMIS14` reader - DC 14 masked interrupt status"]
pub type Dcmis14R = crate::BitReader;
#[doc = "Field `DCMIS15` reader - DC 15 masked interrupt status"]
pub type Dcmis15R = crate::BitReader;
#[doc = "Field `DCMIS16` reader - DC 16 masked interrupt status"]
pub type Dcmis16R = crate::BitReader;
#[doc = "Field `DCMIS17` reader - DC 17 masked interrupt status"]
pub type Dcmis17R = crate::BitReader;
#[doc = "Field `DCMIS18` reader - DC 18 masked interrupt status"]
pub type Dcmis18R = crate::BitReader;
#[doc = "Field `DCMIS19` reader - DC 19 masked interrupt status"]
pub type Dcmis19R = crate::BitReader;
#[doc = "Field `DCMIS20` reader - DC 20 masked interrupt status"]
pub type Dcmis20R = crate::BitReader;
#[doc = "Field `DCMIS21` reader - DC 21 masked interrupt status"]
pub type Dcmis21R = crate::BitReader;
#[doc = "Field `DCMIS22` reader - DC 22 masked interrupt status"]
pub type Dcmis22R = crate::BitReader;
#[doc = "Field `DCMIS23` reader - DC 23 masked interrupt status"]
pub type Dcmis23R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sequencer 0 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis0(&self) -> Seqmis0R {
        Seqmis0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis1(&self) -> Seqmis1R {
        Seqmis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis2(&self) -> Seqmis2R {
        Seqmis2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis3(&self) -> Seqmis3R {
        Seqmis3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis4(&self) -> Seqmis4R {
        Seqmis4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis5(&self) -> Seqmis5R {
        Seqmis5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis6(&self) -> Seqmis6R {
        Seqmis6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 masked interrupt status"]
    #[inline(always)]
    pub fn seqmis7(&self) -> Seqmis7R {
        Seqmis7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DC 0 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis0(&self) -> Dcmis0R {
        Dcmis0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DC 1 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis1(&self) -> Dcmis1R {
        Dcmis1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DC 2 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis2(&self) -> Dcmis2R {
        Dcmis2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DC 3 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis3(&self) -> Dcmis3R {
        Dcmis3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DC 4 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis4(&self) -> Dcmis4R {
        Dcmis4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DC 5 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis5(&self) -> Dcmis5R {
        Dcmis5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DC 6 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis6(&self) -> Dcmis6R {
        Dcmis6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DC 7 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis7(&self) -> Dcmis7R {
        Dcmis7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DC 8 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis8(&self) -> Dcmis8R {
        Dcmis8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DC 9 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis9(&self) -> Dcmis9R {
        Dcmis9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DC 10 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis10(&self) -> Dcmis10R {
        Dcmis10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DC 11 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis11(&self) -> Dcmis11R {
        Dcmis11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DC 12 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis12(&self) -> Dcmis12R {
        Dcmis12R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DC 13 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis13(&self) -> Dcmis13R {
        Dcmis13R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DC 14 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis14(&self) -> Dcmis14R {
        Dcmis14R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DC 15 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis15(&self) -> Dcmis15R {
        Dcmis15R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DC 16 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis16(&self) -> Dcmis16R {
        Dcmis16R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DC 17 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis17(&self) -> Dcmis17R {
        Dcmis17R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DC 18 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis18(&self) -> Dcmis18R {
        Dcmis18R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DC 19 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis19(&self) -> Dcmis19R {
        Dcmis19R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DC 20 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis20(&self) -> Dcmis20R {
        Dcmis20R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DC 21 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis21(&self) -> Dcmis21R {
        Dcmis21R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DC 22 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis22(&self) -> Dcmis22R {
        Dcmis22R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DC 23 masked interrupt status"]
    #[inline(always)]
    pub fn dcmis23(&self) -> Dcmis23R {
        Dcmis23R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Masked interrupt status and clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
