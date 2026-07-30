#[doc = "Register `SEQEN` reader"]
pub type R = crate::R<SeqenSpec>;
#[doc = "Register `SEQEN` writer"]
pub type W = crate::W<SeqenSpec>;
#[doc = "Field `SEQEN0` reader - Enable sequencer 0"]
pub type Seqen0R = crate::BitReader;
#[doc = "Field `SEQEN0` writer - Enable sequencer 0"]
pub type Seqen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQEN1` reader - Enable sequencer 1"]
pub type Seqen1R = crate::BitReader;
#[doc = "Field `SEQEN1` writer - Enable sequencer 1"]
pub type Seqen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQEN2` reader - Enable sequencer 2"]
pub type Seqen2R = crate::BitReader;
#[doc = "Field `SEQEN2` writer - Enable sequencer 2"]
pub type Seqen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQEN3` reader - Enable sequencer 3"]
pub type Seqen3R = crate::BitReader;
#[doc = "Field `SEQEN3` writer - Enable sequencer 3"]
pub type Seqen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQEN4` reader - Enable sequencer 4"]
pub type Seqen4R = crate::BitReader;
#[doc = "Field `SEQEN4` writer - Enable sequencer 4"]
pub type Seqen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQEN5` reader - Enable sequencer 5"]
pub type Seqen5R = crate::BitReader;
#[doc = "Field `SEQEN5` writer - Enable sequencer 5"]
pub type Seqen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQEN6` reader - Enable sequencer 6"]
pub type Seqen6R = crate::BitReader;
#[doc = "Field `SEQEN6` writer - Enable sequencer 6"]
pub type Seqen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQEN7` reader - Enable sequencer 7"]
pub type Seqen7R = crate::BitReader;
#[doc = "Field `SEQEN7` writer - Enable sequencer 7"]
pub type Seqen7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable sequencer 0"]
    #[inline(always)]
    pub fn seqen0(&self) -> Seqen0R {
        Seqen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1"]
    #[inline(always)]
    pub fn seqen1(&self) -> Seqen1R {
        Seqen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable sequencer 2"]
    #[inline(always)]
    pub fn seqen2(&self) -> Seqen2R {
        Seqen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable sequencer 3"]
    #[inline(always)]
    pub fn seqen3(&self) -> Seqen3R {
        Seqen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable sequencer 4"]
    #[inline(always)]
    pub fn seqen4(&self) -> Seqen4R {
        Seqen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable sequencer 5"]
    #[inline(always)]
    pub fn seqen5(&self) -> Seqen5R {
        Seqen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable sequencer 6"]
    #[inline(always)]
    pub fn seqen6(&self) -> Seqen6R {
        Seqen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable sequencer 7"]
    #[inline(always)]
    pub fn seqen7(&self) -> Seqen7R {
        Seqen7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sequencer 0"]
    #[inline(always)]
    pub fn seqen0(&mut self) -> Seqen0W<'_, SeqenSpec> {
        Seqen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable sequencer 1"]
    #[inline(always)]
    pub fn seqen1(&mut self) -> Seqen1W<'_, SeqenSpec> {
        Seqen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable sequencer 2"]
    #[inline(always)]
    pub fn seqen2(&mut self) -> Seqen2W<'_, SeqenSpec> {
        Seqen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable sequencer 3"]
    #[inline(always)]
    pub fn seqen3(&mut self) -> Seqen3W<'_, SeqenSpec> {
        Seqen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable sequencer 4"]
    #[inline(always)]
    pub fn seqen4(&mut self) -> Seqen4W<'_, SeqenSpec> {
        Seqen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable sequencer 5"]
    #[inline(always)]
    pub fn seqen5(&mut self) -> Seqen5W<'_, SeqenSpec> {
        Seqen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable sequencer 6"]
    #[inline(always)]
    pub fn seqen6(&mut self) -> Seqen6W<'_, SeqenSpec> {
        Seqen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable sequencer 7"]
    #[inline(always)]
    pub fn seqen7(&mut self) -> Seqen7W<'_, SeqenSpec> {
        Seqen7W::new(self, 7)
    }
}
#[doc = "Enable sequencer register\n\nYou can [`read`](crate::Reg::read) this register and get [`seqen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqenSpec;
impl crate::RegisterSpec for SeqenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqen::R`](R) reader structure"]
impl crate::Readable for SeqenSpec {}
#[doc = "`write(|w| ..)` method takes [`seqen::W`](W) writer structure"]
impl crate::Writable for SeqenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQEN to value 0"]
impl crate::Resettable for SeqenSpec {}
