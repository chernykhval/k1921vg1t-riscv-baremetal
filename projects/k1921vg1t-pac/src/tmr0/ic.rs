#[doc = "Register `IC` writer"]
pub type W = crate::W<IcSpec>;
#[doc = "Field `TMR` writer - Timer Interrupt Clear"]
pub type TmrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM0_0` writer - Capcom0 Interrupt Clear"]
pub type Capcom0_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM0_1` writer - Capcom1 Interrupt Clear"]
pub type Capcom0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM1_0` writer - Capcom2 Interrupt Clear"]
pub type Capcom1_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM1_1` writer - Capcom3 Interrupt Clear"]
pub type Capcom1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM2_0` writer - Capcom4 Interrupt Clear"]
pub type Capcom2_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM2_1` writer - Capcom5 Interrupt Clear"]
pub type Capcom2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM3_0` writer - Capcom6 Interrupt Clear"]
pub type Capcom3_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPCOM3_1` writer - Capcom7 Interrupt Clear"]
pub type Capcom3_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Timer Interrupt Clear"]
    #[inline(always)]
    pub fn tmr(&mut self) -> TmrW<'_, IcSpec> {
        TmrW::new(self, 0)
    }
    #[doc = "Bit 1 - Capcom0 Interrupt Clear"]
    #[inline(always)]
    pub fn capcom0_0(&mut self) -> Capcom0_0W<'_, IcSpec> {
        Capcom0_0W::new(self, 1)
    }
    #[doc = "Bit 2 - Capcom1 Interrupt Clear"]
    #[inline(always)]
    pub fn capcom0_1(&mut self) -> Capcom0_1W<'_, IcSpec> {
        Capcom0_1W::new(self, 2)
    }
    #[doc = "Bit 3 - Capcom2 Interrupt Clear"]
    #[inline(always)]
    pub fn capcom1_0(&mut self) -> Capcom1_0W<'_, IcSpec> {
        Capcom1_0W::new(self, 3)
    }
    #[doc = "Bit 4 - Capcom3 Interrupt Clear"]
    #[inline(always)]
    pub fn capcom1_1(&mut self) -> Capcom1_1W<'_, IcSpec> {
        Capcom1_1W::new(self, 4)
    }
    #[doc = "Bit 5 - Capcom4 Interrupt Clear"]
    #[inline(always)]
    pub fn capcom2_0(&mut self) -> Capcom2_0W<'_, IcSpec> {
        Capcom2_0W::new(self, 5)
    }
    #[doc = "Bit 6 - Capcom5 Interrupt Clear"]
    #[inline(always)]
    pub fn capcom2_1(&mut self) -> Capcom2_1W<'_, IcSpec> {
        Capcom2_1W::new(self, 6)
    }
    #[doc = "Bit 7 - Capcom6 Interrupt Clear"]
    #[inline(always)]
    pub fn capcom3_0(&mut self) -> Capcom3_0W<'_, IcSpec> {
        Capcom3_0W::new(self, 7)
    }
    #[doc = "Bit 8 - Capcom7 Interrupt Clear"]
    #[inline(always)]
    pub fn capcom3_1(&mut self) -> Capcom3_1W<'_, IcSpec> {
        Capcom3_1W::new(self, 8)
    }
}
#[doc = "Clear Interrupt Status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
