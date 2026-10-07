#[doc = "Register `CH_START` writer"]
pub type W = crate::W<ChStartSpec>;
#[doc = "Field `CH0` writer - Start Channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Start Channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Start Channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Start Channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Start Channel 4"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Start Channel 5"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Start Channel 6"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Start Channel 7"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Start Channel 8"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Start Channel 9"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Start Channel 10"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Start Channel 11"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Start Channel 12"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Start Channel 13"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Start Channel 14"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Start Channel 15"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH16` writer - Start Channel 16"]
pub type Ch16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH17` writer - Start Channel 17"]
pub type Ch17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH18` writer - Start Channel 18"]
pub type Ch18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH19` writer - Start Channel 19"]
pub type Ch19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH20` writer - Start Channel 20"]
pub type Ch20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH21` writer - Start Channel 21"]
pub type Ch21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH22` writer - Start Channel 22"]
pub type Ch22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH23` writer - Start Channel 23"]
pub type Ch23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH24` writer - Start Channel 24"]
pub type Ch24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH25` writer - Start Channel 25"]
pub type Ch25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH26` writer - Start Channel 26"]
pub type Ch26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH27` writer - Start Channel 27"]
pub type Ch27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH28` writer - Start Channel 28"]
pub type Ch28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH29` writer - Start Channel 29"]
pub type Ch29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH30` writer - Start Channel 30"]
pub type Ch30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH31` writer - Start Channel 31"]
pub type Ch31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start Channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, ChStartSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Start Channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, ChStartSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Start Channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, ChStartSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Start Channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, ChStartSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Start Channel 4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, ChStartSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Start Channel 5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, ChStartSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Start Channel 6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, ChStartSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Start Channel 7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, ChStartSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Start Channel 8"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, ChStartSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Start Channel 9"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, ChStartSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Start Channel 10"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<'_, ChStartSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Start Channel 11"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<'_, ChStartSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Start Channel 12"]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<'_, ChStartSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Start Channel 13"]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<'_, ChStartSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Start Channel 14"]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<'_, ChStartSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Start Channel 15"]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<'_, ChStartSpec> {
        Ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Start Channel 16"]
    #[inline(always)]
    pub fn ch16(&mut self) -> Ch16W<'_, ChStartSpec> {
        Ch16W::new(self, 16)
    }
    #[doc = "Bit 17 - Start Channel 17"]
    #[inline(always)]
    pub fn ch17(&mut self) -> Ch17W<'_, ChStartSpec> {
        Ch17W::new(self, 17)
    }
    #[doc = "Bit 18 - Start Channel 18"]
    #[inline(always)]
    pub fn ch18(&mut self) -> Ch18W<'_, ChStartSpec> {
        Ch18W::new(self, 18)
    }
    #[doc = "Bit 19 - Start Channel 19"]
    #[inline(always)]
    pub fn ch19(&mut self) -> Ch19W<'_, ChStartSpec> {
        Ch19W::new(self, 19)
    }
    #[doc = "Bit 20 - Start Channel 20"]
    #[inline(always)]
    pub fn ch20(&mut self) -> Ch20W<'_, ChStartSpec> {
        Ch20W::new(self, 20)
    }
    #[doc = "Bit 21 - Start Channel 21"]
    #[inline(always)]
    pub fn ch21(&mut self) -> Ch21W<'_, ChStartSpec> {
        Ch21W::new(self, 21)
    }
    #[doc = "Bit 22 - Start Channel 22"]
    #[inline(always)]
    pub fn ch22(&mut self) -> Ch22W<'_, ChStartSpec> {
        Ch22W::new(self, 22)
    }
    #[doc = "Bit 23 - Start Channel 23"]
    #[inline(always)]
    pub fn ch23(&mut self) -> Ch23W<'_, ChStartSpec> {
        Ch23W::new(self, 23)
    }
    #[doc = "Bit 24 - Start Channel 24"]
    #[inline(always)]
    pub fn ch24(&mut self) -> Ch24W<'_, ChStartSpec> {
        Ch24W::new(self, 24)
    }
    #[doc = "Bit 25 - Start Channel 25"]
    #[inline(always)]
    pub fn ch25(&mut self) -> Ch25W<'_, ChStartSpec> {
        Ch25W::new(self, 25)
    }
    #[doc = "Bit 26 - Start Channel 26"]
    #[inline(always)]
    pub fn ch26(&mut self) -> Ch26W<'_, ChStartSpec> {
        Ch26W::new(self, 26)
    }
    #[doc = "Bit 27 - Start Channel 27"]
    #[inline(always)]
    pub fn ch27(&mut self) -> Ch27W<'_, ChStartSpec> {
        Ch27W::new(self, 27)
    }
    #[doc = "Bit 28 - Start Channel 28"]
    #[inline(always)]
    pub fn ch28(&mut self) -> Ch28W<'_, ChStartSpec> {
        Ch28W::new(self, 28)
    }
    #[doc = "Bit 29 - Start Channel 29"]
    #[inline(always)]
    pub fn ch29(&mut self) -> Ch29W<'_, ChStartSpec> {
        Ch29W::new(self, 29)
    }
    #[doc = "Bit 30 - Start Channel 30"]
    #[inline(always)]
    pub fn ch30(&mut self) -> Ch30W<'_, ChStartSpec> {
        Ch30W::new(self, 30)
    }
    #[doc = "Bit 31 - Start Channel 31"]
    #[inline(always)]
    pub fn ch31(&mut self) -> Ch31W<'_, ChStartSpec> {
        Ch31W::new(self, 31)
    }
}
#[doc = "DMA channel start register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChStartSpec;
impl crate::RegisterSpec for ChStartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_start::W`](W) writer structure"]
impl crate::Writable for ChStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_START to value 0"]
impl crate::Resettable for ChStartSpec {}
