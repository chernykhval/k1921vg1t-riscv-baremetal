#[doc = "Register `CH_INTSTAT` reader"]
pub type R = crate::R<ChIntstatSpec>;
#[doc = "Field `CH0` reader - Interrupt caused by Channel 0"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - Interrupt caused by Channel 1"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - Interrupt caused by Channel 2"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - Interrupt caused by Channel 3"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH4` reader - Interrupt caused by Channel 4"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH5` reader - Interrupt caused by Channel 5"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH6` reader - Interrupt caused by Channel 6"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH7` reader - Interrupt caused by Channel 7"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH8` reader - Interrupt caused by Channel 8"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH9` reader - Interrupt caused by Channel 9"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH10` reader - Interrupt caused by Channel 10"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH11` reader - Interrupt caused by Channel 11"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH12` reader - Interrupt caused by Channel 12"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH13` reader - Interrupt caused by Channel 13"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH14` reader - Interrupt caused by Channel 14"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH15` reader - Interrupt caused by Channel 15"]
pub type Ch15R = crate::BitReader;
#[doc = "Field `CH16` reader - Interrupt caused by Channel 16"]
pub type Ch16R = crate::BitReader;
#[doc = "Field `CH17` reader - Interrupt caused by Channel 17"]
pub type Ch17R = crate::BitReader;
#[doc = "Field `CH18` reader - Interrupt caused by Channel 18"]
pub type Ch18R = crate::BitReader;
#[doc = "Field `CH19` reader - Interrupt caused by Channel 19"]
pub type Ch19R = crate::BitReader;
#[doc = "Field `CH20` reader - Interrupt caused by Channel 20"]
pub type Ch20R = crate::BitReader;
#[doc = "Field `CH21` reader - Interrupt caused by Channel 21"]
pub type Ch21R = crate::BitReader;
#[doc = "Field `CH22` reader - Interrupt caused by Channel 22"]
pub type Ch22R = crate::BitReader;
#[doc = "Field `CH23` reader - Interrupt caused by Channel 23"]
pub type Ch23R = crate::BitReader;
#[doc = "Field `CH24` reader - Interrupt caused by Channel 24"]
pub type Ch24R = crate::BitReader;
#[doc = "Field `CH25` reader - Interrupt caused by Channel 25"]
pub type Ch25R = crate::BitReader;
#[doc = "Field `CH26` reader - Interrupt caused by Channel 26"]
pub type Ch26R = crate::BitReader;
#[doc = "Field `CH27` reader - Interrupt caused by Channel 27"]
pub type Ch27R = crate::BitReader;
#[doc = "Field `CH28` reader - Interrupt caused by Channel 28"]
pub type Ch28R = crate::BitReader;
#[doc = "Field `CH29` reader - Interrupt caused by Channel 29"]
pub type Ch29R = crate::BitReader;
#[doc = "Field `CH30` reader - Interrupt caused by Channel 30"]
pub type Ch30R = crate::BitReader;
#[doc = "Field `CH31` reader - Interrupt caused by Channel 31"]
pub type Ch31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt caused by Channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt caused by Channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt caused by Channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt caused by Channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt caused by Channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt caused by Channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt caused by Channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt caused by Channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt caused by Channel 8"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt caused by Channel 9"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt caused by Channel 10"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt caused by Channel 11"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt caused by Channel 12"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt caused by Channel 13"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt caused by Channel 14"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt caused by Channel 15"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt caused by Channel 16"]
    #[inline(always)]
    pub fn ch16(&self) -> Ch16R {
        Ch16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt caused by Channel 17"]
    #[inline(always)]
    pub fn ch17(&self) -> Ch17R {
        Ch17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt caused by Channel 18"]
    #[inline(always)]
    pub fn ch18(&self) -> Ch18R {
        Ch18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt caused by Channel 19"]
    #[inline(always)]
    pub fn ch19(&self) -> Ch19R {
        Ch19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt caused by Channel 20"]
    #[inline(always)]
    pub fn ch20(&self) -> Ch20R {
        Ch20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt caused by Channel 21"]
    #[inline(always)]
    pub fn ch21(&self) -> Ch21R {
        Ch21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt caused by Channel 22"]
    #[inline(always)]
    pub fn ch22(&self) -> Ch22R {
        Ch22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt caused by Channel 23"]
    #[inline(always)]
    pub fn ch23(&self) -> Ch23R {
        Ch23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt caused by Channel 24"]
    #[inline(always)]
    pub fn ch24(&self) -> Ch24R {
        Ch24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt caused by Channel 25"]
    #[inline(always)]
    pub fn ch25(&self) -> Ch25R {
        Ch25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt caused by Channel 26"]
    #[inline(always)]
    pub fn ch26(&self) -> Ch26R {
        Ch26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt caused by Channel 27"]
    #[inline(always)]
    pub fn ch27(&self) -> Ch27R {
        Ch27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt caused by Channel 28"]
    #[inline(always)]
    pub fn ch28(&self) -> Ch28R {
        Ch28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt caused by Channel 29"]
    #[inline(always)]
    pub fn ch29(&self) -> Ch29R {
        Ch29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt caused by Channel 30"]
    #[inline(always)]
    pub fn ch30(&self) -> Ch30R {
        Ch30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt caused by Channel 31"]
    #[inline(always)]
    pub fn ch31(&self) -> Ch31R {
        Ch31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DMA Core Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_intstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChIntstatSpec;
impl crate::RegisterSpec for ChIntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_intstat::R`](R) reader structure"]
impl crate::Readable for ChIntstatSpec {}
#[doc = "`reset()` method sets CH_INTSTAT to value 0"]
impl crate::Resettable for ChIntstatSpec {}
