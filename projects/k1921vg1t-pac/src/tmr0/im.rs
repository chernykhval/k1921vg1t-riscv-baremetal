#[doc = "Register `IM` reader"]
pub type R = crate::R<ImSpec>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<ImSpec>;
#[doc = "Field `TMR` reader - Timer Interrupt Enable"]
pub type TmrR = crate::BitReader;
#[doc = "Field `TMR` writer - Timer Interrupt Enable"]
pub type TmrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM0_0` reader - Capcom0 Interrupt Enable"]
pub type Capcom0_0R = crate::BitReader;
#[doc = "Field `CAPCOM0_0` writer - Capcom0 Interrupt Enable"]
pub type Capcom0_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM0_1` reader - Capcom1 Interrupt Enable"]
pub type Capcom0_1R = crate::BitReader;
#[doc = "Field `CAPCOM0_1` writer - Capcom1 Interrupt Enable"]
pub type Capcom0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM1_0` reader - Capcom2 Interrupt Enable"]
pub type Capcom1_0R = crate::BitReader;
#[doc = "Field `CAPCOM1_0` writer - Capcom2 Interrupt Enable"]
pub type Capcom1_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM1_1` reader - Capcom3 Interrupt Enable"]
pub type Capcom1_1R = crate::BitReader;
#[doc = "Field `CAPCOM1_1` writer - Capcom3 Interrupt Enable"]
pub type Capcom1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM2_0` reader - Capcom4 Interrupt Enable"]
pub type Capcom2_0R = crate::BitReader;
#[doc = "Field `CAPCOM2_0` writer - Capcom4 Interrupt Enable"]
pub type Capcom2_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM2_1` reader - Capcom5 Interrupt Enable"]
pub type Capcom2_1R = crate::BitReader;
#[doc = "Field `CAPCOM2_1` writer - Capcom5 Interrupt Enable"]
pub type Capcom2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM3_0` reader - Capcom6 Interrupt Enable"]
pub type Capcom3_0R = crate::BitReader;
#[doc = "Field `CAPCOM3_0` writer - Capcom6 Interrupt Enable"]
pub type Capcom3_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM3_1` reader - Capcom7 Interrupt Enable"]
pub type Capcom3_1R = crate::BitReader;
#[doc = "Field `CAPCOM3_1` writer - Capcom7 Interrupt Enable"]
pub type Capcom3_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tmr(&self) -> TmrR {
        TmrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capcom0 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom0_0(&self) -> Capcom0_0R {
        Capcom0_0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capcom1 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom0_1(&self) -> Capcom0_1R {
        Capcom0_1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capcom2 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom1_0(&self) -> Capcom1_0R {
        Capcom1_0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capcom3 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom1_1(&self) -> Capcom1_1R {
        Capcom1_1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capcom4 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom2_0(&self) -> Capcom2_0R {
        Capcom2_0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capcom5 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom2_1(&self) -> Capcom2_1R {
        Capcom2_1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capcom6 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom3_0(&self) -> Capcom3_0R {
        Capcom3_0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capcom7 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom3_1(&self) -> Capcom3_1R {
        Capcom3_1R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tmr(&mut self) -> TmrW<'_, ImSpec> {
        TmrW::new(self, 0)
    }
    #[doc = "Bit 1 - Capcom0 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom0_0(&mut self) -> Capcom0_0W<'_, ImSpec> {
        Capcom0_0W::new(self, 1)
    }
    #[doc = "Bit 2 - Capcom1 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom0_1(&mut self) -> Capcom0_1W<'_, ImSpec> {
        Capcom0_1W::new(self, 2)
    }
    #[doc = "Bit 3 - Capcom2 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom1_0(&mut self) -> Capcom1_0W<'_, ImSpec> {
        Capcom1_0W::new(self, 3)
    }
    #[doc = "Bit 4 - Capcom3 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom1_1(&mut self) -> Capcom1_1W<'_, ImSpec> {
        Capcom1_1W::new(self, 4)
    }
    #[doc = "Bit 5 - Capcom4 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom2_0(&mut self) -> Capcom2_0W<'_, ImSpec> {
        Capcom2_0W::new(self, 5)
    }
    #[doc = "Bit 6 - Capcom5 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom2_1(&mut self) -> Capcom2_1W<'_, ImSpec> {
        Capcom2_1W::new(self, 6)
    }
    #[doc = "Bit 7 - Capcom6 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom3_0(&mut self) -> Capcom3_0W<'_, ImSpec> {
        Capcom3_0W::new(self, 7)
    }
    #[doc = "Bit 8 - Capcom7 Interrupt Enable"]
    #[inline(always)]
    pub fn capcom3_1(&mut self) -> Capcom3_1W<'_, ImSpec> {
        Capcom3_1W::new(self, 8)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`im::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImSpec;
impl crate::RegisterSpec for ImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for ImSpec {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for ImSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for ImSpec {}
