#[doc = "Register `CICNT` reader"]
pub type R = crate::R<CicntSpec>;
#[doc = "Register `CICNT` writer"]
pub type W = crate::W<CicntSpec>;
#[doc = "Field `ICNT0` reader - Clear interrupt counter on sequencer 0 start"]
pub type Icnt0R = crate::BitReader;
#[doc = "Field `ICNT0` writer - Clear interrupt counter on sequencer 0 start"]
pub type Icnt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT1` reader - Clear interrupt counter on sequencer 1 start"]
pub type Icnt1R = crate::BitReader;
#[doc = "Field `ICNT1` writer - Clear interrupt counter on sequencer 1 start"]
pub type Icnt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT2` reader - Clear interrupt counter on sequencer 2 start"]
pub type Icnt2R = crate::BitReader;
#[doc = "Field `ICNT2` writer - Clear interrupt counter on sequencer 2 start"]
pub type Icnt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT3` reader - Clear interrupt counter on sequencer 3 start"]
pub type Icnt3R = crate::BitReader;
#[doc = "Field `ICNT3` writer - Clear interrupt counter on sequencer 3 start"]
pub type Icnt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT4` reader - Clear interrupt counter on sequencer 4 start"]
pub type Icnt4R = crate::BitReader;
#[doc = "Field `ICNT4` writer - Clear interrupt counter on sequencer 4 start"]
pub type Icnt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT5` reader - Clear interrupt counter on sequencer 5 start"]
pub type Icnt5R = crate::BitReader;
#[doc = "Field `ICNT5` writer - Clear interrupt counter on sequencer 5 start"]
pub type Icnt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT6` reader - Clear interrupt counter on sequencer 6 start"]
pub type Icnt6R = crate::BitReader;
#[doc = "Field `ICNT6` writer - Clear interrupt counter on sequencer 6 start"]
pub type Icnt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT7` reader - Clear interrupt counter on sequencer 7 start"]
pub type Icnt7R = crate::BitReader;
#[doc = "Field `ICNT7` writer - Clear interrupt counter on sequencer 7 start"]
pub type Icnt7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear interrupt counter on sequencer 0 start"]
    #[inline(always)]
    pub fn icnt0(&self) -> Icnt0R {
        Icnt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear interrupt counter on sequencer 1 start"]
    #[inline(always)]
    pub fn icnt1(&self) -> Icnt1R {
        Icnt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear interrupt counter on sequencer 2 start"]
    #[inline(always)]
    pub fn icnt2(&self) -> Icnt2R {
        Icnt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear interrupt counter on sequencer 3 start"]
    #[inline(always)]
    pub fn icnt3(&self) -> Icnt3R {
        Icnt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear interrupt counter on sequencer 4 start"]
    #[inline(always)]
    pub fn icnt4(&self) -> Icnt4R {
        Icnt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear interrupt counter on sequencer 5 start"]
    #[inline(always)]
    pub fn icnt5(&self) -> Icnt5R {
        Icnt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear interrupt counter on sequencer 6 start"]
    #[inline(always)]
    pub fn icnt6(&self) -> Icnt6R {
        Icnt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear interrupt counter on sequencer 7 start"]
    #[inline(always)]
    pub fn icnt7(&self) -> Icnt7R {
        Icnt7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear interrupt counter on sequencer 0 start"]
    #[inline(always)]
    pub fn icnt0(&mut self) -> Icnt0W<'_, CicntSpec> {
        Icnt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear interrupt counter on sequencer 1 start"]
    #[inline(always)]
    pub fn icnt1(&mut self) -> Icnt1W<'_, CicntSpec> {
        Icnt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear interrupt counter on sequencer 2 start"]
    #[inline(always)]
    pub fn icnt2(&mut self) -> Icnt2W<'_, CicntSpec> {
        Icnt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear interrupt counter on sequencer 3 start"]
    #[inline(always)]
    pub fn icnt3(&mut self) -> Icnt3W<'_, CicntSpec> {
        Icnt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear interrupt counter on sequencer 4 start"]
    #[inline(always)]
    pub fn icnt4(&mut self) -> Icnt4W<'_, CicntSpec> {
        Icnt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear interrupt counter on sequencer 5 start"]
    #[inline(always)]
    pub fn icnt5(&mut self) -> Icnt5W<'_, CicntSpec> {
        Icnt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear interrupt counter on sequencer 6 start"]
    #[inline(always)]
    pub fn icnt6(&mut self) -> Icnt6W<'_, CicntSpec> {
        Icnt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear interrupt counter on sequencer 7 start"]
    #[inline(always)]
    pub fn icnt7(&mut self) -> Icnt7W<'_, CicntSpec> {
        Icnt7W::new(self, 7)
    }
}
#[doc = "Interrupt counter clear control\n\nYou can [`read`](crate::Reg::read) this register and get [`cicnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicntSpec;
impl crate::RegisterSpec for CicntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cicnt::R`](R) reader structure"]
impl crate::Readable for CicntSpec {}
#[doc = "`write(|w| ..)` method takes [`cicnt::W`](W) writer structure"]
impl crate::Writable for CicntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CICNT to value 0"]
impl crate::Resettable for CicntSpec {}
