#[doc = "Register `TZSEL` reader"]
pub type R = crate::R<TzselSpec>;
#[doc = "Register `TZSEL` writer"]
pub type W = crate::W<TzselSpec>;
#[doc = "Field `CBC0` reader - Cycle-by-Cycle trip-zone 0 enable"]
pub type Cbc0R = crate::BitReader;
#[doc = "Field `CBC0` writer - Cycle-by-Cycle trip-zone 0 enable"]
pub type Cbc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC1` reader - Cycle-by-Cycle trip-zone 1 enable"]
pub type Cbc1R = crate::BitReader;
#[doc = "Field `CBC1` writer - Cycle-by-Cycle trip-zone 1 enable"]
pub type Cbc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC2` reader - Cycle-by-Cycle trip-zone 2 enable"]
pub type Cbc2R = crate::BitReader;
#[doc = "Field `CBC2` writer - Cycle-by-Cycle trip-zone 2 enable"]
pub type Cbc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC3` reader - Cycle-by-Cycle trip-zone 3 enable"]
pub type Cbc3R = crate::BitReader;
#[doc = "Field `CBC3` writer - Cycle-by-Cycle trip-zone 3 enable"]
pub type Cbc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC4` reader - Cycle-by-Cycle trip-zone 4 enable"]
pub type Cbc4R = crate::BitReader;
#[doc = "Field `CBC4` writer - Cycle-by-Cycle trip-zone 4 enable"]
pub type Cbc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC5` reader - Cycle-by-Cycle trip-zone 5 enable"]
pub type Cbc5R = crate::BitReader;
#[doc = "Field `CBC5` writer - Cycle-by-Cycle trip-zone 5 enable"]
pub type Cbc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST0` reader - One-Shot trip-zone 0 enable"]
pub type Ost0R = crate::BitReader;
#[doc = "Field `OST0` writer - One-Shot trip-zone 0 enable"]
pub type Ost0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST1` reader - One-Shot trip-zone 1 enable"]
pub type Ost1R = crate::BitReader;
#[doc = "Field `OST1` writer - One-Shot trip-zone 1 enable"]
pub type Ost1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST2` reader - One-Shot trip-zone 2 enable"]
pub type Ost2R = crate::BitReader;
#[doc = "Field `OST2` writer - One-Shot trip-zone 2 enable"]
pub type Ost2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST3` reader - One-Shot trip-zone 3 enable"]
pub type Ost3R = crate::BitReader;
#[doc = "Field `OST3` writer - One-Shot trip-zone 3 enable"]
pub type Ost3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST4` reader - One-Shot trip-zone 4 enable"]
pub type Ost4R = crate::BitReader;
#[doc = "Field `OST4` writer - One-Shot trip-zone 4 enable"]
pub type Ost4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST5` reader - One-Shot trip-zone 5 enable"]
pub type Ost5R = crate::BitReader;
#[doc = "Field `OST5` writer - One-Shot trip-zone 5 enable"]
pub type Ost5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cycle-by-Cycle trip-zone 0 enable"]
    #[inline(always)]
    pub fn cbc0(&self) -> Cbc0R {
        Cbc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cycle-by-Cycle trip-zone 1 enable"]
    #[inline(always)]
    pub fn cbc1(&self) -> Cbc1R {
        Cbc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cycle-by-Cycle trip-zone 2 enable"]
    #[inline(always)]
    pub fn cbc2(&self) -> Cbc2R {
        Cbc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cycle-by-Cycle trip-zone 3 enable"]
    #[inline(always)]
    pub fn cbc3(&self) -> Cbc3R {
        Cbc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cycle-by-Cycle trip-zone 4 enable"]
    #[inline(always)]
    pub fn cbc4(&self) -> Cbc4R {
        Cbc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cycle-by-Cycle trip-zone 5 enable"]
    #[inline(always)]
    pub fn cbc5(&self) -> Cbc5R {
        Cbc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - One-Shot trip-zone 0 enable"]
    #[inline(always)]
    pub fn ost0(&self) -> Ost0R {
        Ost0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - One-Shot trip-zone 1 enable"]
    #[inline(always)]
    pub fn ost1(&self) -> Ost1R {
        Ost1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - One-Shot trip-zone 2 enable"]
    #[inline(always)]
    pub fn ost2(&self) -> Ost2R {
        Ost2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One-Shot trip-zone 3 enable"]
    #[inline(always)]
    pub fn ost3(&self) -> Ost3R {
        Ost3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - One-Shot trip-zone 4 enable"]
    #[inline(always)]
    pub fn ost4(&self) -> Ost4R {
        Ost4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - One-Shot trip-zone 5 enable"]
    #[inline(always)]
    pub fn ost5(&self) -> Ost5R {
        Ost5R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cycle-by-Cycle trip-zone 0 enable"]
    #[inline(always)]
    pub fn cbc0(&mut self) -> Cbc0W<'_, TzselSpec> {
        Cbc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Cycle-by-Cycle trip-zone 1 enable"]
    #[inline(always)]
    pub fn cbc1(&mut self) -> Cbc1W<'_, TzselSpec> {
        Cbc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Cycle-by-Cycle trip-zone 2 enable"]
    #[inline(always)]
    pub fn cbc2(&mut self) -> Cbc2W<'_, TzselSpec> {
        Cbc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Cycle-by-Cycle trip-zone 3 enable"]
    #[inline(always)]
    pub fn cbc3(&mut self) -> Cbc3W<'_, TzselSpec> {
        Cbc3W::new(self, 3)
    }
    #[doc = "Bit 4 - Cycle-by-Cycle trip-zone 4 enable"]
    #[inline(always)]
    pub fn cbc4(&mut self) -> Cbc4W<'_, TzselSpec> {
        Cbc4W::new(self, 4)
    }
    #[doc = "Bit 5 - Cycle-by-Cycle trip-zone 5 enable"]
    #[inline(always)]
    pub fn cbc5(&mut self) -> Cbc5W<'_, TzselSpec> {
        Cbc5W::new(self, 5)
    }
    #[doc = "Bit 8 - One-Shot trip-zone 0 enable"]
    #[inline(always)]
    pub fn ost0(&mut self) -> Ost0W<'_, TzselSpec> {
        Ost0W::new(self, 8)
    }
    #[doc = "Bit 9 - One-Shot trip-zone 1 enable"]
    #[inline(always)]
    pub fn ost1(&mut self) -> Ost1W<'_, TzselSpec> {
        Ost1W::new(self, 9)
    }
    #[doc = "Bit 10 - One-Shot trip-zone 2 enable"]
    #[inline(always)]
    pub fn ost2(&mut self) -> Ost2W<'_, TzselSpec> {
        Ost2W::new(self, 10)
    }
    #[doc = "Bit 11 - One-Shot trip-zone 3 enable"]
    #[inline(always)]
    pub fn ost3(&mut self) -> Ost3W<'_, TzselSpec> {
        Ost3W::new(self, 11)
    }
    #[doc = "Bit 12 - One-Shot trip-zone 4 enable"]
    #[inline(always)]
    pub fn ost4(&mut self) -> Ost4W<'_, TzselSpec> {
        Ost4W::new(self, 12)
    }
    #[doc = "Bit 13 - One-Shot trip-zone 5 enable"]
    #[inline(always)]
    pub fn ost5(&mut self) -> Ost5W<'_, TzselSpec> {
        Ost5W::new(self, 13)
    }
}
#[doc = "Trip-Zone Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzselSpec;
impl crate::RegisterSpec for TzselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsel::R`](R) reader structure"]
impl crate::Readable for TzselSpec {}
#[doc = "`write(|w| ..)` method takes [`tzsel::W`](W) writer structure"]
impl crate::Writable for TzselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZSEL to value 0"]
impl crate::Resettable for TzselSpec {}
