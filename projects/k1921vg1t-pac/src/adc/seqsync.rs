#[doc = "Register `SEQSYNC` reader"]
pub type R = crate::R<SeqsyncSpec>;
#[doc = "Register `SEQSYNC` writer"]
pub type W = crate::W<SeqsyncSpec>;
#[doc = "Field `SYNC0` reader - Enable sequencer 0 software sync"]
pub type Sync0R = crate::BitReader;
#[doc = "Field `SYNC0` writer - Enable sequencer 0 software sync"]
pub type Sync0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1` reader - Enable sequencer 1 software sync"]
pub type Sync1R = crate::BitReader;
#[doc = "Field `SYNC1` writer - Enable sequencer 1 software sync"]
pub type Sync1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC2` reader - Enable sequencer 2 software sync"]
pub type Sync2R = crate::BitReader;
#[doc = "Field `SYNC2` writer - Enable sequencer 2 software sync"]
pub type Sync2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC3` reader - Enable sequencer 3 software sync"]
pub type Sync3R = crate::BitReader;
#[doc = "Field `SYNC3` writer - Enable sequencer 3 software sync"]
pub type Sync3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC4` reader - Enable sequencer 4 software sync"]
pub type Sync4R = crate::BitReader;
#[doc = "Field `SYNC4` writer - Enable sequencer 4 software sync"]
pub type Sync4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC5` reader - Enable sequencer 5 software sync"]
pub type Sync5R = crate::BitReader;
#[doc = "Field `SYNC5` writer - Enable sequencer 5 software sync"]
pub type Sync5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC6` reader - Enable sequencer 6 software sync"]
pub type Sync6R = crate::BitReader;
#[doc = "Field `SYNC6` writer - Enable sequencer 6 software sync"]
pub type Sync6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC7` reader - Enable sequencer 7 software sync"]
pub type Sync7R = crate::BitReader;
#[doc = "Field `SYNC7` writer - Enable sequencer 7 software sync"]
pub type Sync7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSYNC` writer - Sync all sequencers"]
pub type GsyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable sequencer 0 software sync"]
    #[inline(always)]
    pub fn sync0(&self) -> Sync0R {
        Sync0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1 software sync"]
    #[inline(always)]
    pub fn sync1(&self) -> Sync1R {
        Sync1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable sequencer 2 software sync"]
    #[inline(always)]
    pub fn sync2(&self) -> Sync2R {
        Sync2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable sequencer 3 software sync"]
    #[inline(always)]
    pub fn sync3(&self) -> Sync3R {
        Sync3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable sequencer 4 software sync"]
    #[inline(always)]
    pub fn sync4(&self) -> Sync4R {
        Sync4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable sequencer 5 software sync"]
    #[inline(always)]
    pub fn sync5(&self) -> Sync5R {
        Sync5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable sequencer 6 software sync"]
    #[inline(always)]
    pub fn sync6(&self) -> Sync6R {
        Sync6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable sequencer 7 software sync"]
    #[inline(always)]
    pub fn sync7(&self) -> Sync7R {
        Sync7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sequencer 0 software sync"]
    #[inline(always)]
    pub fn sync0(&mut self) -> Sync0W<'_, SeqsyncSpec> {
        Sync0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1 software sync"]
    #[inline(always)]
    pub fn sync1(&mut self) -> Sync1W<'_, SeqsyncSpec> {
        Sync1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable sequencer 2 software sync"]
    #[inline(always)]
    pub fn sync2(&mut self) -> Sync2W<'_, SeqsyncSpec> {
        Sync2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable sequencer 3 software sync"]
    #[inline(always)]
    pub fn sync3(&mut self) -> Sync3W<'_, SeqsyncSpec> {
        Sync3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable sequencer 4 software sync"]
    #[inline(always)]
    pub fn sync4(&mut self) -> Sync4W<'_, SeqsyncSpec> {
        Sync4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable sequencer 5 software sync"]
    #[inline(always)]
    pub fn sync5(&mut self) -> Sync5W<'_, SeqsyncSpec> {
        Sync5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable sequencer 6 software sync"]
    #[inline(always)]
    pub fn sync6(&mut self) -> Sync6W<'_, SeqsyncSpec> {
        Sync6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable sequencer 7 software sync"]
    #[inline(always)]
    pub fn sync7(&mut self) -> Sync7W<'_, SeqsyncSpec> {
        Sync7W::new(self, 7)
    }
    #[doc = "Bit 31 - Sync all sequencers"]
    #[inline(always)]
    pub fn gsync(&mut self) -> GsyncW<'_, SeqsyncSpec> {
        GsyncW::new(self, 31)
    }
}
#[doc = "Sequencer sync register\n\nYou can [`read`](crate::Reg::read) this register and get [`seqsync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqsync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqsyncSpec;
impl crate::RegisterSpec for SeqsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqsync::R`](R) reader structure"]
impl crate::Readable for SeqsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`seqsync::W`](W) writer structure"]
impl crate::Writable for SeqsyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQSYNC to value 0"]
impl crate::Resettable for SeqsyncSpec {}
