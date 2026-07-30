#[doc = "Register `FSTAT` reader"]
pub type R = crate::R<FstatSpec>;
#[doc = "Register `FSTAT` writer"]
pub type W = crate::W<FstatSpec>;
#[doc = "Field `OV0` reader - Sequencer 0 FIFO overflow"]
pub type Ov0R = crate::BitReader;
#[doc = "Field `OV0` writer - Sequencer 0 FIFO overflow"]
pub type Ov0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV1` reader - Sequencer 1 FIFO overflow"]
pub type Ov1R = crate::BitReader;
#[doc = "Field `OV1` writer - Sequencer 1 FIFO overflow"]
pub type Ov1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV2` reader - Sequencer 2 FIFO overflow"]
pub type Ov2R = crate::BitReader;
#[doc = "Field `OV2` writer - Sequencer 2 FIFO overflow"]
pub type Ov2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV3` reader - Sequencer 3 FIFO overflow"]
pub type Ov3R = crate::BitReader;
#[doc = "Field `OV3` writer - Sequencer 3 FIFO overflow"]
pub type Ov3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV4` reader - Sequencer 4 FIFO overflow"]
pub type Ov4R = crate::BitReader;
#[doc = "Field `OV4` writer - Sequencer 4 FIFO overflow"]
pub type Ov4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV5` reader - Sequencer 5 FIFO overflow"]
pub type Ov5R = crate::BitReader;
#[doc = "Field `OV5` writer - Sequencer 5 FIFO overflow"]
pub type Ov5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV6` reader - Sequencer 6 FIFO overflow"]
pub type Ov6R = crate::BitReader;
#[doc = "Field `OV6` writer - Sequencer 6 FIFO overflow"]
pub type Ov6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV7` reader - Sequencer 7 FIFO overflow"]
pub type Ov7R = crate::BitReader;
#[doc = "Field `OV7` writer - Sequencer 7 FIFO overflow"]
pub type Ov7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN0` reader - Sequencer 0 FIFO underflow"]
pub type Un0R = crate::BitReader;
#[doc = "Field `UN0` writer - Sequencer 0 FIFO underflow"]
pub type Un0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN1` reader - Sequencer 1 FIFO underflow"]
pub type Un1R = crate::BitReader;
#[doc = "Field `UN1` writer - Sequencer 1 FIFO underflow"]
pub type Un1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN2` reader - Sequencer 2 FIFO underflow"]
pub type Un2R = crate::BitReader;
#[doc = "Field `UN2` writer - Sequencer 2 FIFO underflow"]
pub type Un2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN3` reader - Sequencer 3 FIFO underflow"]
pub type Un3R = crate::BitReader;
#[doc = "Field `UN3` writer - Sequencer 3 FIFO underflow"]
pub type Un3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN4` reader - Sequencer 4 FIFO underflow"]
pub type Un4R = crate::BitReader;
#[doc = "Field `UN4` writer - Sequencer 4 FIFO underflow"]
pub type Un4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN5` reader - Sequencer 5 FIFO underflow"]
pub type Un5R = crate::BitReader;
#[doc = "Field `UN5` writer - Sequencer 5 FIFO underflow"]
pub type Un5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN6` reader - Sequencer 6 FIFO underflow"]
pub type Un6R = crate::BitReader;
#[doc = "Field `UN6` writer - Sequencer 6 FIFO underflow"]
pub type Un6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UN7` reader - Sequencer 7 FIFO underflow"]
pub type Un7R = crate::BitReader;
#[doc = "Field `UN7` writer - Sequencer 7 FIFO underflow"]
pub type Un7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV0` reader - Sequencer 0 FIFO DMA request overflow"]
pub type Dov0R = crate::BitReader;
#[doc = "Field `DOV0` writer - Sequencer 0 FIFO DMA request overflow"]
pub type Dov0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV1` reader - Sequencer 1 FIFO DMA request overflow"]
pub type Dov1R = crate::BitReader;
#[doc = "Field `DOV1` writer - Sequencer 1 FIFO DMA request overflow"]
pub type Dov1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV2` reader - Sequencer 2 FIFO DMA request overflow"]
pub type Dov2R = crate::BitReader;
#[doc = "Field `DOV2` writer - Sequencer 2 FIFO DMA request overflow"]
pub type Dov2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV3` reader - Sequencer 3 FIFO DMA request overflow"]
pub type Dov3R = crate::BitReader;
#[doc = "Field `DOV3` writer - Sequencer 3 FIFO DMA request overflow"]
pub type Dov3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV4` reader - Sequencer 4 FIFO DMA request overflow"]
pub type Dov4R = crate::BitReader;
#[doc = "Field `DOV4` writer - Sequencer 4 FIFO DMA request overflow"]
pub type Dov4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV5` reader - Sequencer 5 FIFO DMA request overflow"]
pub type Dov5R = crate::BitReader;
#[doc = "Field `DOV5` writer - Sequencer 5 FIFO DMA request overflow"]
pub type Dov5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV6` reader - Sequencer 6 FIFO DMA request overflow"]
pub type Dov6R = crate::BitReader;
#[doc = "Field `DOV6` writer - Sequencer 6 FIFO DMA request overflow"]
pub type Dov6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOV7` reader - Sequencer 7 FIFO DMA request overflow"]
pub type Dov7R = crate::BitReader;
#[doc = "Field `DOV7` writer - Sequencer 7 FIFO DMA request overflow"]
pub type Dov7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sequencer 0 FIFO overflow"]
    #[inline(always)]
    pub fn ov0(&self) -> Ov0R {
        Ov0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer 1 FIFO overflow"]
    #[inline(always)]
    pub fn ov1(&self) -> Ov1R {
        Ov1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sequencer 2 FIFO overflow"]
    #[inline(always)]
    pub fn ov2(&self) -> Ov2R {
        Ov2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sequencer 3 FIFO overflow"]
    #[inline(always)]
    pub fn ov3(&self) -> Ov3R {
        Ov3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sequencer 4 FIFO overflow"]
    #[inline(always)]
    pub fn ov4(&self) -> Ov4R {
        Ov4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sequencer 5 FIFO overflow"]
    #[inline(always)]
    pub fn ov5(&self) -> Ov5R {
        Ov5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sequencer 6 FIFO overflow"]
    #[inline(always)]
    pub fn ov6(&self) -> Ov6R {
        Ov6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sequencer 7 FIFO overflow"]
    #[inline(always)]
    pub fn ov7(&self) -> Ov7R {
        Ov7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sequencer 0 FIFO underflow"]
    #[inline(always)]
    pub fn un0(&self) -> Un0R {
        Un0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sequencer 1 FIFO underflow"]
    #[inline(always)]
    pub fn un1(&self) -> Un1R {
        Un1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Sequencer 2 FIFO underflow"]
    #[inline(always)]
    pub fn un2(&self) -> Un2R {
        Un2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sequencer 3 FIFO underflow"]
    #[inline(always)]
    pub fn un3(&self) -> Un3R {
        Un3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sequencer 4 FIFO underflow"]
    #[inline(always)]
    pub fn un4(&self) -> Un4R {
        Un4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Sequencer 5 FIFO underflow"]
    #[inline(always)]
    pub fn un5(&self) -> Un5R {
        Un5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Sequencer 6 FIFO underflow"]
    #[inline(always)]
    pub fn un6(&self) -> Un6R {
        Un6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sequencer 7 FIFO underflow"]
    #[inline(always)]
    pub fn un7(&self) -> Un7R {
        Un7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Sequencer 0 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov0(&self) -> Dov0R {
        Dov0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sequencer 1 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov1(&self) -> Dov1R {
        Dov1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Sequencer 2 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov2(&self) -> Dov2R {
        Dov2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Sequencer 3 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov3(&self) -> Dov3R {
        Dov3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Sequencer 4 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov4(&self) -> Dov4R {
        Dov4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Sequencer 5 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov5(&self) -> Dov5R {
        Dov5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Sequencer 6 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov6(&self) -> Dov6R {
        Dov6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sequencer 7 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov7(&self) -> Dov7R {
        Dov7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sequencer 0 FIFO overflow"]
    #[inline(always)]
    pub fn ov0(&mut self) -> Ov0W<'_, FstatSpec> {
        Ov0W::new(self, 0)
    }
    #[doc = "Bit 1 - Sequencer 1 FIFO overflow"]
    #[inline(always)]
    pub fn ov1(&mut self) -> Ov1W<'_, FstatSpec> {
        Ov1W::new(self, 1)
    }
    #[doc = "Bit 2 - Sequencer 2 FIFO overflow"]
    #[inline(always)]
    pub fn ov2(&mut self) -> Ov2W<'_, FstatSpec> {
        Ov2W::new(self, 2)
    }
    #[doc = "Bit 3 - Sequencer 3 FIFO overflow"]
    #[inline(always)]
    pub fn ov3(&mut self) -> Ov3W<'_, FstatSpec> {
        Ov3W::new(self, 3)
    }
    #[doc = "Bit 4 - Sequencer 4 FIFO overflow"]
    #[inline(always)]
    pub fn ov4(&mut self) -> Ov4W<'_, FstatSpec> {
        Ov4W::new(self, 4)
    }
    #[doc = "Bit 5 - Sequencer 5 FIFO overflow"]
    #[inline(always)]
    pub fn ov5(&mut self) -> Ov5W<'_, FstatSpec> {
        Ov5W::new(self, 5)
    }
    #[doc = "Bit 6 - Sequencer 6 FIFO overflow"]
    #[inline(always)]
    pub fn ov6(&mut self) -> Ov6W<'_, FstatSpec> {
        Ov6W::new(self, 6)
    }
    #[doc = "Bit 7 - Sequencer 7 FIFO overflow"]
    #[inline(always)]
    pub fn ov7(&mut self) -> Ov7W<'_, FstatSpec> {
        Ov7W::new(self, 7)
    }
    #[doc = "Bit 8 - Sequencer 0 FIFO underflow"]
    #[inline(always)]
    pub fn un0(&mut self) -> Un0W<'_, FstatSpec> {
        Un0W::new(self, 8)
    }
    #[doc = "Bit 9 - Sequencer 1 FIFO underflow"]
    #[inline(always)]
    pub fn un1(&mut self) -> Un1W<'_, FstatSpec> {
        Un1W::new(self, 9)
    }
    #[doc = "Bit 10 - Sequencer 2 FIFO underflow"]
    #[inline(always)]
    pub fn un2(&mut self) -> Un2W<'_, FstatSpec> {
        Un2W::new(self, 10)
    }
    #[doc = "Bit 11 - Sequencer 3 FIFO underflow"]
    #[inline(always)]
    pub fn un3(&mut self) -> Un3W<'_, FstatSpec> {
        Un3W::new(self, 11)
    }
    #[doc = "Bit 12 - Sequencer 4 FIFO underflow"]
    #[inline(always)]
    pub fn un4(&mut self) -> Un4W<'_, FstatSpec> {
        Un4W::new(self, 12)
    }
    #[doc = "Bit 13 - Sequencer 5 FIFO underflow"]
    #[inline(always)]
    pub fn un5(&mut self) -> Un5W<'_, FstatSpec> {
        Un5W::new(self, 13)
    }
    #[doc = "Bit 14 - Sequencer 6 FIFO underflow"]
    #[inline(always)]
    pub fn un6(&mut self) -> Un6W<'_, FstatSpec> {
        Un6W::new(self, 14)
    }
    #[doc = "Bit 15 - Sequencer 7 FIFO underflow"]
    #[inline(always)]
    pub fn un7(&mut self) -> Un7W<'_, FstatSpec> {
        Un7W::new(self, 15)
    }
    #[doc = "Bit 16 - Sequencer 0 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov0(&mut self) -> Dov0W<'_, FstatSpec> {
        Dov0W::new(self, 16)
    }
    #[doc = "Bit 17 - Sequencer 1 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov1(&mut self) -> Dov1W<'_, FstatSpec> {
        Dov1W::new(self, 17)
    }
    #[doc = "Bit 18 - Sequencer 2 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov2(&mut self) -> Dov2W<'_, FstatSpec> {
        Dov2W::new(self, 18)
    }
    #[doc = "Bit 19 - Sequencer 3 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov3(&mut self) -> Dov3W<'_, FstatSpec> {
        Dov3W::new(self, 19)
    }
    #[doc = "Bit 20 - Sequencer 4 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov4(&mut self) -> Dov4W<'_, FstatSpec> {
        Dov4W::new(self, 20)
    }
    #[doc = "Bit 21 - Sequencer 5 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov5(&mut self) -> Dov5W<'_, FstatSpec> {
        Dov5W::new(self, 21)
    }
    #[doc = "Bit 22 - Sequencer 6 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov6(&mut self) -> Dov6W<'_, FstatSpec> {
        Dov6W::new(self, 22)
    }
    #[doc = "Bit 23 - Sequencer 7 FIFO DMA request overflow"]
    #[inline(always)]
    pub fn dov7(&mut self) -> Dov7W<'_, FstatSpec> {
        Dov7W::new(self, 23)
    }
}
#[doc = "FIFO overflow status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FstatSpec;
impl crate::RegisterSpec for FstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstat::R`](R) reader structure"]
impl crate::Readable for FstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fstat::W`](W) writer structure"]
impl crate::Writable for FstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSTAT to value 0xff00"]
impl crate::Resettable for FstatSpec {
    const RESET_VALUE: u32 = 0xff00;
}
