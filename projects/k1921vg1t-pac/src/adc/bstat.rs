#[doc = "Register `BSTAT` reader"]
pub type R = crate::R<BstatSpec>;
#[doc = "Field `SEQBUSY0` reader - Sequencer 0 busy"]
pub type Seqbusy0R = crate::BitReader;
#[doc = "Field `SEQBUSY1` reader - Sequencer 1 busy"]
pub type Seqbusy1R = crate::BitReader;
#[doc = "Field `SEQBUSY2` reader - Sequencer 2 busy"]
pub type Seqbusy2R = crate::BitReader;
#[doc = "Field `SEQBUSY3` reader - Sequencer 3 busy"]
pub type Seqbusy3R = crate::BitReader;
#[doc = "Field `SEQBUSY4` reader - Sequencer 4 busy"]
pub type Seqbusy4R = crate::BitReader;
#[doc = "Field `SEQBUSY5` reader - Sequencer 5 busy"]
pub type Seqbusy5R = crate::BitReader;
#[doc = "Field `SEQBUSY6` reader - Sequencer 6 busy"]
pub type Seqbusy6R = crate::BitReader;
#[doc = "Field `SEQBUSY7` reader - Sequencer 7 busy"]
pub type Seqbusy7R = crate::BitReader;
#[doc = "Field `ADCBUSY0` reader - ADC module 0 conversion busy"]
pub type Adcbusy0R = crate::BitReader;
#[doc = "Field `ADCBUSY1` reader - ADC module 1 conversion busy"]
pub type Adcbusy1R = crate::BitReader;
#[doc = "Field `ADCBUSY2` reader - ADC module 2 conversion busy"]
pub type Adcbusy2R = crate::BitReader;
#[doc = "Field `ADCBUSY3` reader - ADC module 3 conversion busy"]
pub type Adcbusy3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sequencer 0 busy"]
    #[inline(always)]
    pub fn seqbusy0(&self) -> Seqbusy0R {
        Seqbusy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 busy"]
    #[inline(always)]
    pub fn seqbusy1(&self) -> Seqbusy1R {
        Seqbusy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 busy"]
    #[inline(always)]
    pub fn seqbusy2(&self) -> Seqbusy2R {
        Seqbusy2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 busy"]
    #[inline(always)]
    pub fn seqbusy3(&self) -> Seqbusy3R {
        Seqbusy3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 busy"]
    #[inline(always)]
    pub fn seqbusy4(&self) -> Seqbusy4R {
        Seqbusy4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 busy"]
    #[inline(always)]
    pub fn seqbusy5(&self) -> Seqbusy5R {
        Seqbusy5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 busy"]
    #[inline(always)]
    pub fn seqbusy6(&self) -> Seqbusy6R {
        Seqbusy6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 busy"]
    #[inline(always)]
    pub fn seqbusy7(&self) -> Seqbusy7R {
        Seqbusy7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC module 0 conversion busy"]
    #[inline(always)]
    pub fn adcbusy0(&self) -> Adcbusy0R {
        Adcbusy0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC module 1 conversion busy"]
    #[inline(always)]
    pub fn adcbusy1(&self) -> Adcbusy1R {
        Adcbusy1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC module 2 conversion busy"]
    #[inline(always)]
    pub fn adcbusy2(&self) -> Adcbusy2R {
        Adcbusy2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC module 3 conversion busy"]
    #[inline(always)]
    pub fn adcbusy3(&self) -> Adcbusy3R {
        Adcbusy3R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Busy status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BstatSpec;
impl crate::RegisterSpec for BstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bstat::R`](R) reader structure"]
impl crate::Readable for BstatSpec {}
#[doc = "`reset()` method sets BSTAT to value 0"]
impl crate::Resettable for BstatSpec {}
