#[doc = "Register `SDC` reader"]
pub type R = crate::R<SdcSpec>;
#[doc = "Register `SDC` writer"]
pub type W = crate::W<SdcSpec>;
#[doc = "Field `DC0` reader - Enable DC 0"]
pub type Dc0R = crate::BitReader;
#[doc = "Field `DC0` writer - Enable DC 0"]
pub type Dc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC1` reader - Enable DC 1"]
pub type Dc1R = crate::BitReader;
#[doc = "Field `DC1` writer - Enable DC 1"]
pub type Dc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC2` reader - Enable DC 2"]
pub type Dc2R = crate::BitReader;
#[doc = "Field `DC2` writer - Enable DC 2"]
pub type Dc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC3` reader - Enable DC 3"]
pub type Dc3R = crate::BitReader;
#[doc = "Field `DC3` writer - Enable DC 3"]
pub type Dc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC4` reader - Enable DC 4"]
pub type Dc4R = crate::BitReader;
#[doc = "Field `DC4` writer - Enable DC 4"]
pub type Dc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC5` reader - Enable DC 5"]
pub type Dc5R = crate::BitReader;
#[doc = "Field `DC5` writer - Enable DC 5"]
pub type Dc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC6` reader - Enable DC 6"]
pub type Dc6R = crate::BitReader;
#[doc = "Field `DC6` writer - Enable DC 6"]
pub type Dc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC7` reader - Enable DC 7"]
pub type Dc7R = crate::BitReader;
#[doc = "Field `DC7` writer - Enable DC 7"]
pub type Dc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC8` reader - Enable DC 8"]
pub type Dc8R = crate::BitReader;
#[doc = "Field `DC8` writer - Enable DC 8"]
pub type Dc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC9` reader - Enable DC 9"]
pub type Dc9R = crate::BitReader;
#[doc = "Field `DC9` writer - Enable DC 9"]
pub type Dc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC10` reader - Enable DC 10"]
pub type Dc10R = crate::BitReader;
#[doc = "Field `DC10` writer - Enable DC 10"]
pub type Dc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC11` reader - Enable DC 11"]
pub type Dc11R = crate::BitReader;
#[doc = "Field `DC11` writer - Enable DC 11"]
pub type Dc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC12` reader - Enable DC 12"]
pub type Dc12R = crate::BitReader;
#[doc = "Field `DC12` writer - Enable DC 12"]
pub type Dc12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC13` reader - Enable DC 13"]
pub type Dc13R = crate::BitReader;
#[doc = "Field `DC13` writer - Enable DC 13"]
pub type Dc13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC14` reader - Enable DC 14"]
pub type Dc14R = crate::BitReader;
#[doc = "Field `DC14` writer - Enable DC 14"]
pub type Dc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC15` reader - Enable DC 15"]
pub type Dc15R = crate::BitReader;
#[doc = "Field `DC15` writer - Enable DC 15"]
pub type Dc15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC16` reader - Enable DC 16"]
pub type Dc16R = crate::BitReader;
#[doc = "Field `DC16` writer - Enable DC 16"]
pub type Dc16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC17` reader - Enable DC 17"]
pub type Dc17R = crate::BitReader;
#[doc = "Field `DC17` writer - Enable DC 17"]
pub type Dc17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC18` reader - Enable DC 18"]
pub type Dc18R = crate::BitReader;
#[doc = "Field `DC18` writer - Enable DC 18"]
pub type Dc18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC19` reader - Enable DC 19"]
pub type Dc19R = crate::BitReader;
#[doc = "Field `DC19` writer - Enable DC 19"]
pub type Dc19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC20` reader - Enable DC 20"]
pub type Dc20R = crate::BitReader;
#[doc = "Field `DC20` writer - Enable DC 20"]
pub type Dc20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC21` reader - Enable DC 21"]
pub type Dc21R = crate::BitReader;
#[doc = "Field `DC21` writer - Enable DC 21"]
pub type Dc21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC22` reader - Enable DC 22"]
pub type Dc22R = crate::BitReader;
#[doc = "Field `DC22` writer - Enable DC 22"]
pub type Dc22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC23` reader - Enable DC 23"]
pub type Dc23R = crate::BitReader;
#[doc = "Field `DC23` writer - Enable DC 23"]
pub type Dc23W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable DC 0"]
    #[inline(always)]
    pub fn dc0(&self) -> Dc0R {
        Dc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DC 1"]
    #[inline(always)]
    pub fn dc1(&self) -> Dc1R {
        Dc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable DC 2"]
    #[inline(always)]
    pub fn dc2(&self) -> Dc2R {
        Dc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable DC 3"]
    #[inline(always)]
    pub fn dc3(&self) -> Dc3R {
        Dc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable DC 4"]
    #[inline(always)]
    pub fn dc4(&self) -> Dc4R {
        Dc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable DC 5"]
    #[inline(always)]
    pub fn dc5(&self) -> Dc5R {
        Dc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable DC 6"]
    #[inline(always)]
    pub fn dc6(&self) -> Dc6R {
        Dc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable DC 7"]
    #[inline(always)]
    pub fn dc7(&self) -> Dc7R {
        Dc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DC 8"]
    #[inline(always)]
    pub fn dc8(&self) -> Dc8R {
        Dc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable DC 9"]
    #[inline(always)]
    pub fn dc9(&self) -> Dc9R {
        Dc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable DC 10"]
    #[inline(always)]
    pub fn dc10(&self) -> Dc10R {
        Dc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DC 11"]
    #[inline(always)]
    pub fn dc11(&self) -> Dc11R {
        Dc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable DC 12"]
    #[inline(always)]
    pub fn dc12(&self) -> Dc12R {
        Dc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable DC 13"]
    #[inline(always)]
    pub fn dc13(&self) -> Dc13R {
        Dc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable DC 14"]
    #[inline(always)]
    pub fn dc14(&self) -> Dc14R {
        Dc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable DC 15"]
    #[inline(always)]
    pub fn dc15(&self) -> Dc15R {
        Dc15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable DC 16"]
    #[inline(always)]
    pub fn dc16(&self) -> Dc16R {
        Dc16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable DC 17"]
    #[inline(always)]
    pub fn dc17(&self) -> Dc17R {
        Dc17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable DC 18"]
    #[inline(always)]
    pub fn dc18(&self) -> Dc18R {
        Dc18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable DC 19"]
    #[inline(always)]
    pub fn dc19(&self) -> Dc19R {
        Dc19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable DC 20"]
    #[inline(always)]
    pub fn dc20(&self) -> Dc20R {
        Dc20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable DC 21"]
    #[inline(always)]
    pub fn dc21(&self) -> Dc21R {
        Dc21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable DC 22"]
    #[inline(always)]
    pub fn dc22(&self) -> Dc22R {
        Dc22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable DC 23"]
    #[inline(always)]
    pub fn dc23(&self) -> Dc23R {
        Dc23R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DC 0"]
    #[inline(always)]
    pub fn dc0(&mut self) -> Dc0W<'_, SdcSpec> {
        Dc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable DC 1"]
    #[inline(always)]
    pub fn dc1(&mut self) -> Dc1W<'_, SdcSpec> {
        Dc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable DC 2"]
    #[inline(always)]
    pub fn dc2(&mut self) -> Dc2W<'_, SdcSpec> {
        Dc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable DC 3"]
    #[inline(always)]
    pub fn dc3(&mut self) -> Dc3W<'_, SdcSpec> {
        Dc3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable DC 4"]
    #[inline(always)]
    pub fn dc4(&mut self) -> Dc4W<'_, SdcSpec> {
        Dc4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable DC 5"]
    #[inline(always)]
    pub fn dc5(&mut self) -> Dc5W<'_, SdcSpec> {
        Dc5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable DC 6"]
    #[inline(always)]
    pub fn dc6(&mut self) -> Dc6W<'_, SdcSpec> {
        Dc6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable DC 7"]
    #[inline(always)]
    pub fn dc7(&mut self) -> Dc7W<'_, SdcSpec> {
        Dc7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable DC 8"]
    #[inline(always)]
    pub fn dc8(&mut self) -> Dc8W<'_, SdcSpec> {
        Dc8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable DC 9"]
    #[inline(always)]
    pub fn dc9(&mut self) -> Dc9W<'_, SdcSpec> {
        Dc9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable DC 10"]
    #[inline(always)]
    pub fn dc10(&mut self) -> Dc10W<'_, SdcSpec> {
        Dc10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable DC 11"]
    #[inline(always)]
    pub fn dc11(&mut self) -> Dc11W<'_, SdcSpec> {
        Dc11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable DC 12"]
    #[inline(always)]
    pub fn dc12(&mut self) -> Dc12W<'_, SdcSpec> {
        Dc12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable DC 13"]
    #[inline(always)]
    pub fn dc13(&mut self) -> Dc13W<'_, SdcSpec> {
        Dc13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable DC 14"]
    #[inline(always)]
    pub fn dc14(&mut self) -> Dc14W<'_, SdcSpec> {
        Dc14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable DC 15"]
    #[inline(always)]
    pub fn dc15(&mut self) -> Dc15W<'_, SdcSpec> {
        Dc15W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable DC 16"]
    #[inline(always)]
    pub fn dc16(&mut self) -> Dc16W<'_, SdcSpec> {
        Dc16W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable DC 17"]
    #[inline(always)]
    pub fn dc17(&mut self) -> Dc17W<'_, SdcSpec> {
        Dc17W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable DC 18"]
    #[inline(always)]
    pub fn dc18(&mut self) -> Dc18W<'_, SdcSpec> {
        Dc18W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable DC 19"]
    #[inline(always)]
    pub fn dc19(&mut self) -> Dc19W<'_, SdcSpec> {
        Dc19W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable DC 20"]
    #[inline(always)]
    pub fn dc20(&mut self) -> Dc20W<'_, SdcSpec> {
        Dc20W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable DC 21"]
    #[inline(always)]
    pub fn dc21(&mut self) -> Dc21W<'_, SdcSpec> {
        Dc21W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable DC 22"]
    #[inline(always)]
    pub fn dc22(&mut self) -> Dc22W<'_, SdcSpec> {
        Dc22W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable DC 23"]
    #[inline(always)]
    pub fn dc23(&mut self) -> Dc23W<'_, SdcSpec> {
        Dc23W::new(self, 23)
    }
}
#[doc = "Sequencer digital comparator selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdcSpec;
impl crate::RegisterSpec for SdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdc::R`](R) reader structure"]
impl crate::Readable for SdcSpec {}
#[doc = "`write(|w| ..)` method takes [`sdc::W`](W) writer structure"]
impl crate::Writable for SdcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDC to value 0"]
impl crate::Resettable for SdcSpec {}
