#[doc = "Register `DATAOUTSET` writer"]
pub type W = crate::W<DataoutsetSpec>;
#[doc = "Field `PIN0` writer - Data output set bit 0"]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN1` writer - Data output set bit 1"]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN2` writer - Data output set bit 2"]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN3` writer - Data output set bit 3"]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN4` writer - Data output set bit 4"]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN5` writer - Data output set bit 5"]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN6` writer - Data output set bit 6"]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN7` writer - Data output set bit 7"]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN8` writer - Data output set bit 8"]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN9` writer - Data output set bit 9"]
pub type Pin9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN10` writer - Data output set bit 10"]
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN11` writer - Data output set bit 11"]
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN12` writer - Data output set bit 12"]
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN13` writer - Data output set bit 13"]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN14` writer - Data output set bit 14"]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN15` writer - Data output set bit 15"]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Data output set bit 0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, DataoutsetSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Data output set bit 1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, DataoutsetSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Data output set bit 2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<'_, DataoutsetSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Data output set bit 3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, DataoutsetSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Data output set bit 4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> Pin4W<'_, DataoutsetSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Data output set bit 5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> Pin5W<'_, DataoutsetSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Data output set bit 6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, DataoutsetSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Data output set bit 7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, DataoutsetSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Data output set bit 8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> Pin8W<'_, DataoutsetSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9 - Data output set bit 9"]
    #[inline(always)]
    pub fn pin9(&mut self) -> Pin9W<'_, DataoutsetSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10 - Data output set bit 10"]
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<'_, DataoutsetSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11 - Data output set bit 11"]
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<'_, DataoutsetSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12 - Data output set bit 12"]
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<'_, DataoutsetSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13 - Data output set bit 13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<'_, DataoutsetSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - Data output set bit 14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<'_, DataoutsetSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - Data output set bit 15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<'_, DataoutsetSpec> {
        Pin15W::new(self, 15)
    }
}
#[doc = "Data output set bits register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataoutset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutsetSpec;
impl crate::RegisterSpec for DataoutsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dataoutset::W`](W) writer structure"]
impl crate::Writable for DataoutsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUTSET to value 0"]
impl crate::Resettable for DataoutsetSpec {}
