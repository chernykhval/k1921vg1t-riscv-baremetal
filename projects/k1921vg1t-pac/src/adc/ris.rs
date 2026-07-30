#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `SEQRIS0` reader - Sequencer 0 raw interrupt status"]
pub type Seqris0R = crate::BitReader;
#[doc = "Field `SEQRIS1` reader - Sequencer 1 raw interrupt status"]
pub type Seqris1R = crate::BitReader;
#[doc = "Field `SEQRIS2` reader - Sequencer 2 raw interrupt status"]
pub type Seqris2R = crate::BitReader;
#[doc = "Field `SEQRIS3` reader - Sequencer 3 raw interrupt status"]
pub type Seqris3R = crate::BitReader;
#[doc = "Field `SEQRIS4` reader - Sequencer 4 raw interrupt status"]
pub type Seqris4R = crate::BitReader;
#[doc = "Field `SEQRIS5` reader - Sequencer 5 raw interrupt status"]
pub type Seqris5R = crate::BitReader;
#[doc = "Field `SEQRIS6` reader - Sequencer 6 raw interrupt status"]
pub type Seqris6R = crate::BitReader;
#[doc = "Field `SEQRIS7` reader - Sequencer 7 raw interrupt status"]
pub type Seqris7R = crate::BitReader;
#[doc = "Field `DCRIS0` reader - Raw interrupt status of Digital Comparator 0"]
pub type Dcris0R = crate::BitReader;
#[doc = "Field `DCRIS1` reader - Raw interrupt status of Digital Comparator 1"]
pub type Dcris1R = crate::BitReader;
#[doc = "Field `DCRIS2` reader - Raw interrupt status of Digital Comparator 2"]
pub type Dcris2R = crate::BitReader;
#[doc = "Field `DCRIS3` reader - Raw interrupt status of Digital Comparator 3"]
pub type Dcris3R = crate::BitReader;
#[doc = "Field `DCRIS4` reader - Raw interrupt status of Digital Comparator 4"]
pub type Dcris4R = crate::BitReader;
#[doc = "Field `DCRIS5` reader - Raw interrupt status of Digital Comparator 5"]
pub type Dcris5R = crate::BitReader;
#[doc = "Field `DCRIS6` reader - Raw interrupt status of Digital Comparator 6"]
pub type Dcris6R = crate::BitReader;
#[doc = "Field `DCRIS7` reader - Raw interrupt status of Digital Comparator 7"]
pub type Dcris7R = crate::BitReader;
#[doc = "Field `DCRIS8` reader - Raw interrupt status of Digital Comparator 8"]
pub type Dcris8R = crate::BitReader;
#[doc = "Field `DCRIS9` reader - Raw interrupt status of Digital Comparator 9"]
pub type Dcris9R = crate::BitReader;
#[doc = "Field `DCRIS10` reader - Raw interrupt status of Digital Comparator 10"]
pub type Dcris10R = crate::BitReader;
#[doc = "Field `DCRIS11` reader - Raw interrupt status of Digital Comparator 11"]
pub type Dcris11R = crate::BitReader;
#[doc = "Field `DCRIS12` reader - Raw interrupt status of Digital Comparator 12"]
pub type Dcris12R = crate::BitReader;
#[doc = "Field `DCRIS13` reader - Raw interrupt status of Digital Comparator 13"]
pub type Dcris13R = crate::BitReader;
#[doc = "Field `DCRIS14` reader - Raw interrupt status of Digital Comparator 14"]
pub type Dcris14R = crate::BitReader;
#[doc = "Field `DCRIS15` reader - Raw interrupt status of Digital Comparator 15"]
pub type Dcris15R = crate::BitReader;
#[doc = "Field `DCRIS16` reader - Raw interrupt status of Digital Comparator 16"]
pub type Dcris16R = crate::BitReader;
#[doc = "Field `DCRIS17` reader - Raw interrupt status of Digital Comparator 17"]
pub type Dcris17R = crate::BitReader;
#[doc = "Field `DCRIS18` reader - Raw interrupt status of Digital Comparator 18"]
pub type Dcris18R = crate::BitReader;
#[doc = "Field `DCRIS19` reader - Raw interrupt status of Digital Comparator 19"]
pub type Dcris19R = crate::BitReader;
#[doc = "Field `DCRIS20` reader - Raw interrupt status of Digital Comparator 20"]
pub type Dcris20R = crate::BitReader;
#[doc = "Field `DCRIS21` reader - Raw interrupt status of Digital Comparator 21"]
pub type Dcris21R = crate::BitReader;
#[doc = "Field `DCRIS22` reader - Raw interrupt status of Digital Comparator 22"]
pub type Dcris22R = crate::BitReader;
#[doc = "Field `DCRIS23` reader - Raw interrupt status of Digital Comparator 23"]
pub type Dcris23R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sequencer 0 raw interrupt status"]
    #[inline(always)]
    pub fn seqris0(&self) -> Seqris0R {
        Seqris0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 raw interrupt status"]
    #[inline(always)]
    pub fn seqris1(&self) -> Seqris1R {
        Seqris1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 raw interrupt status"]
    #[inline(always)]
    pub fn seqris2(&self) -> Seqris2R {
        Seqris2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 raw interrupt status"]
    #[inline(always)]
    pub fn seqris3(&self) -> Seqris3R {
        Seqris3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 raw interrupt status"]
    #[inline(always)]
    pub fn seqris4(&self) -> Seqris4R {
        Seqris4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 raw interrupt status"]
    #[inline(always)]
    pub fn seqris5(&self) -> Seqris5R {
        Seqris5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 raw interrupt status"]
    #[inline(always)]
    pub fn seqris6(&self) -> Seqris6R {
        Seqris6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 raw interrupt status"]
    #[inline(always)]
    pub fn seqris7(&self) -> Seqris7R {
        Seqris7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status of Digital Comparator 0"]
    #[inline(always)]
    pub fn dcris0(&self) -> Dcris0R {
        Dcris0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status of Digital Comparator 1"]
    #[inline(always)]
    pub fn dcris1(&self) -> Dcris1R {
        Dcris1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status of Digital Comparator 2"]
    #[inline(always)]
    pub fn dcris2(&self) -> Dcris2R {
        Dcris2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status of Digital Comparator 3"]
    #[inline(always)]
    pub fn dcris3(&self) -> Dcris3R {
        Dcris3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status of Digital Comparator 4"]
    #[inline(always)]
    pub fn dcris4(&self) -> Dcris4R {
        Dcris4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status of Digital Comparator 5"]
    #[inline(always)]
    pub fn dcris5(&self) -> Dcris5R {
        Dcris5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status of Digital Comparator 6"]
    #[inline(always)]
    pub fn dcris6(&self) -> Dcris6R {
        Dcris6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw interrupt status of Digital Comparator 7"]
    #[inline(always)]
    pub fn dcris7(&self) -> Dcris7R {
        Dcris7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw interrupt status of Digital Comparator 8"]
    #[inline(always)]
    pub fn dcris8(&self) -> Dcris8R {
        Dcris8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw interrupt status of Digital Comparator 9"]
    #[inline(always)]
    pub fn dcris9(&self) -> Dcris9R {
        Dcris9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw interrupt status of Digital Comparator 10"]
    #[inline(always)]
    pub fn dcris10(&self) -> Dcris10R {
        Dcris10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw interrupt status of Digital Comparator 11"]
    #[inline(always)]
    pub fn dcris11(&self) -> Dcris11R {
        Dcris11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Raw interrupt status of Digital Comparator 12"]
    #[inline(always)]
    pub fn dcris12(&self) -> Dcris12R {
        Dcris12R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Raw interrupt status of Digital Comparator 13"]
    #[inline(always)]
    pub fn dcris13(&self) -> Dcris13R {
        Dcris13R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Raw interrupt status of Digital Comparator 14"]
    #[inline(always)]
    pub fn dcris14(&self) -> Dcris14R {
        Dcris14R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Raw interrupt status of Digital Comparator 15"]
    #[inline(always)]
    pub fn dcris15(&self) -> Dcris15R {
        Dcris15R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Raw interrupt status of Digital Comparator 16"]
    #[inline(always)]
    pub fn dcris16(&self) -> Dcris16R {
        Dcris16R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Raw interrupt status of Digital Comparator 17"]
    #[inline(always)]
    pub fn dcris17(&self) -> Dcris17R {
        Dcris17R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Raw interrupt status of Digital Comparator 18"]
    #[inline(always)]
    pub fn dcris18(&self) -> Dcris18R {
        Dcris18R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Raw interrupt status of Digital Comparator 19"]
    #[inline(always)]
    pub fn dcris19(&self) -> Dcris19R {
        Dcris19R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Raw interrupt status of Digital Comparator 20"]
    #[inline(always)]
    pub fn dcris20(&self) -> Dcris20R {
        Dcris20R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Raw interrupt status of Digital Comparator 21"]
    #[inline(always)]
    pub fn dcris21(&self) -> Dcris21R {
        Dcris21R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Raw interrupt status of Digital Comparator 22"]
    #[inline(always)]
    pub fn dcris22(&self) -> Dcris22R {
        Dcris22R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Raw interrupt status of Digital Comparator 23"]
    #[inline(always)]
    pub fn dcris23(&self) -> Dcris23R {
        Dcris23R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {}
