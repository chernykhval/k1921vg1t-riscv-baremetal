#[doc = "Register `IC` writer"]
pub type W = crate::W<IcSpec>;
#[doc = "Field `SEQIC0` writer - Sequencer 0 interrupt status clear"]
pub type Seqic0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIC1` writer - Sequencer 1 interrupt status clear"]
pub type Seqic1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIC2` writer - Sequencer 2 interrupt status clear"]
pub type Seqic2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIC3` writer - Sequencer 3 interrupt status clear"]
pub type Seqic3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIC4` writer - Sequencer 4 interrupt status clear"]
pub type Seqic4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIC5` writer - Sequencer 5 interrupt status clear"]
pub type Seqic5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIC6` writer - Sequencer 6 interrupt status clear"]
pub type Seqic6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQIC7` writer - Sequencer 7 interrupt status clear"]
pub type Seqic7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC0` writer - DC 0 interrupt status clear"]
pub type Dcic0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC1` writer - DC 1 interrupt status clear"]
pub type Dcic1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC2` writer - DC 2 interrupt status clear"]
pub type Dcic2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC3` writer - DC 3 interrupt status clear"]
pub type Dcic3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC4` writer - DC 4 interrupt status clear"]
pub type Dcic4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC5` writer - DC 5 interrupt status clear"]
pub type Dcic5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC6` writer - DC 6 interrupt status clear"]
pub type Dcic6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC7` writer - DC 7 interrupt status clear"]
pub type Dcic7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC8` writer - DC 8 interrupt status clear"]
pub type Dcic8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC9` writer - DC 9 interrupt status clear"]
pub type Dcic9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC10` writer - DC 10 interrupt status clear"]
pub type Dcic10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC11` writer - DC 11 interrupt status clear"]
pub type Dcic11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC12` writer - DC 12 interrupt status clear"]
pub type Dcic12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC13` writer - DC 13 interrupt status clear"]
pub type Dcic13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC14` writer - DC 14 interrupt status clear"]
pub type Dcic14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC15` writer - DC 15 interrupt status clear"]
pub type Dcic15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC16` writer - DC 16 interrupt status clear"]
pub type Dcic16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC17` writer - DC 17 interrupt status clear"]
pub type Dcic17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC18` writer - DC 18 interrupt status clear"]
pub type Dcic18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC19` writer - DC 19 interrupt status clear"]
pub type Dcic19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC20` writer - DC 20 interrupt status clear"]
pub type Dcic20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC21` writer - DC 21 interrupt status clear"]
pub type Dcic21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC22` writer - DC 22 interrupt status clear"]
pub type Dcic22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIC23` writer - DC 23 interrupt status clear"]
pub type Dcic23W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Sequencer 0 interrupt status clear"]
    #[inline(always)]
    pub fn seqic0(&mut self) -> Seqic0W<'_, IcSpec> {
        Seqic0W::new(self, 0)
    }
    #[doc = "Bit 1 - Sequencer 1 interrupt status clear"]
    #[inline(always)]
    pub fn seqic1(&mut self) -> Seqic1W<'_, IcSpec> {
        Seqic1W::new(self, 1)
    }
    #[doc = "Bit 2 - Sequencer 2 interrupt status clear"]
    #[inline(always)]
    pub fn seqic2(&mut self) -> Seqic2W<'_, IcSpec> {
        Seqic2W::new(self, 2)
    }
    #[doc = "Bit 3 - Sequencer 3 interrupt status clear"]
    #[inline(always)]
    pub fn seqic3(&mut self) -> Seqic3W<'_, IcSpec> {
        Seqic3W::new(self, 3)
    }
    #[doc = "Bit 4 - Sequencer 4 interrupt status clear"]
    #[inline(always)]
    pub fn seqic4(&mut self) -> Seqic4W<'_, IcSpec> {
        Seqic4W::new(self, 4)
    }
    #[doc = "Bit 5 - Sequencer 5 interrupt status clear"]
    #[inline(always)]
    pub fn seqic5(&mut self) -> Seqic5W<'_, IcSpec> {
        Seqic5W::new(self, 5)
    }
    #[doc = "Bit 6 - Sequencer 6 interrupt status clear"]
    #[inline(always)]
    pub fn seqic6(&mut self) -> Seqic6W<'_, IcSpec> {
        Seqic6W::new(self, 6)
    }
    #[doc = "Bit 7 - Sequencer 7 interrupt status clear"]
    #[inline(always)]
    pub fn seqic7(&mut self) -> Seqic7W<'_, IcSpec> {
        Seqic7W::new(self, 7)
    }
    #[doc = "Bit 8 - DC 0 interrupt status clear"]
    #[inline(always)]
    pub fn dcic0(&mut self) -> Dcic0W<'_, IcSpec> {
        Dcic0W::new(self, 8)
    }
    #[doc = "Bit 9 - DC 1 interrupt status clear"]
    #[inline(always)]
    pub fn dcic1(&mut self) -> Dcic1W<'_, IcSpec> {
        Dcic1W::new(self, 9)
    }
    #[doc = "Bit 10 - DC 2 interrupt status clear"]
    #[inline(always)]
    pub fn dcic2(&mut self) -> Dcic2W<'_, IcSpec> {
        Dcic2W::new(self, 10)
    }
    #[doc = "Bit 11 - DC 3 interrupt status clear"]
    #[inline(always)]
    pub fn dcic3(&mut self) -> Dcic3W<'_, IcSpec> {
        Dcic3W::new(self, 11)
    }
    #[doc = "Bit 12 - DC 4 interrupt status clear"]
    #[inline(always)]
    pub fn dcic4(&mut self) -> Dcic4W<'_, IcSpec> {
        Dcic4W::new(self, 12)
    }
    #[doc = "Bit 13 - DC 5 interrupt status clear"]
    #[inline(always)]
    pub fn dcic5(&mut self) -> Dcic5W<'_, IcSpec> {
        Dcic5W::new(self, 13)
    }
    #[doc = "Bit 14 - DC 6 interrupt status clear"]
    #[inline(always)]
    pub fn dcic6(&mut self) -> Dcic6W<'_, IcSpec> {
        Dcic6W::new(self, 14)
    }
    #[doc = "Bit 15 - DC 7 interrupt status clear"]
    #[inline(always)]
    pub fn dcic7(&mut self) -> Dcic7W<'_, IcSpec> {
        Dcic7W::new(self, 15)
    }
    #[doc = "Bit 16 - DC 8 interrupt status clear"]
    #[inline(always)]
    pub fn dcic8(&mut self) -> Dcic8W<'_, IcSpec> {
        Dcic8W::new(self, 16)
    }
    #[doc = "Bit 17 - DC 9 interrupt status clear"]
    #[inline(always)]
    pub fn dcic9(&mut self) -> Dcic9W<'_, IcSpec> {
        Dcic9W::new(self, 17)
    }
    #[doc = "Bit 18 - DC 10 interrupt status clear"]
    #[inline(always)]
    pub fn dcic10(&mut self) -> Dcic10W<'_, IcSpec> {
        Dcic10W::new(self, 18)
    }
    #[doc = "Bit 19 - DC 11 interrupt status clear"]
    #[inline(always)]
    pub fn dcic11(&mut self) -> Dcic11W<'_, IcSpec> {
        Dcic11W::new(self, 19)
    }
    #[doc = "Bit 20 - DC 12 interrupt status clear"]
    #[inline(always)]
    pub fn dcic12(&mut self) -> Dcic12W<'_, IcSpec> {
        Dcic12W::new(self, 20)
    }
    #[doc = "Bit 21 - DC 13 interrupt status clear"]
    #[inline(always)]
    pub fn dcic13(&mut self) -> Dcic13W<'_, IcSpec> {
        Dcic13W::new(self, 21)
    }
    #[doc = "Bit 22 - DC 14 interrupt status clear"]
    #[inline(always)]
    pub fn dcic14(&mut self) -> Dcic14W<'_, IcSpec> {
        Dcic14W::new(self, 22)
    }
    #[doc = "Bit 23 - DC 15 interrupt status clear"]
    #[inline(always)]
    pub fn dcic15(&mut self) -> Dcic15W<'_, IcSpec> {
        Dcic15W::new(self, 23)
    }
    #[doc = "Bit 24 - DC 16 interrupt status clear"]
    #[inline(always)]
    pub fn dcic16(&mut self) -> Dcic16W<'_, IcSpec> {
        Dcic16W::new(self, 24)
    }
    #[doc = "Bit 25 - DC 17 interrupt status clear"]
    #[inline(always)]
    pub fn dcic17(&mut self) -> Dcic17W<'_, IcSpec> {
        Dcic17W::new(self, 25)
    }
    #[doc = "Bit 26 - DC 18 interrupt status clear"]
    #[inline(always)]
    pub fn dcic18(&mut self) -> Dcic18W<'_, IcSpec> {
        Dcic18W::new(self, 26)
    }
    #[doc = "Bit 27 - DC 19 interrupt status clear"]
    #[inline(always)]
    pub fn dcic19(&mut self) -> Dcic19W<'_, IcSpec> {
        Dcic19W::new(self, 27)
    }
    #[doc = "Bit 28 - DC 20 interrupt status clear"]
    #[inline(always)]
    pub fn dcic20(&mut self) -> Dcic20W<'_, IcSpec> {
        Dcic20W::new(self, 28)
    }
    #[doc = "Bit 29 - DC 21 interrupt status clear"]
    #[inline(always)]
    pub fn dcic21(&mut self) -> Dcic21W<'_, IcSpec> {
        Dcic21W::new(self, 29)
    }
    #[doc = "Bit 30 - DC 22 interrupt status clear"]
    #[inline(always)]
    pub fn dcic22(&mut self) -> Dcic22W<'_, IcSpec> {
        Dcic22W::new(self, 30)
    }
    #[doc = "Bit 31 - DC 23 interrupt status clear"]
    #[inline(always)]
    pub fn dcic23(&mut self) -> Dcic23W<'_, IcSpec> {
        Dcic23W::new(self, 31)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSpec;
impl crate::RegisterSpec for IcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ic::W`](W) writer structure"]
impl crate::Writable for IcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IcSpec {}
