#[doc = "Register `FLASHCFG` reader"]
pub type R = crate::R<FlashcfgSpec>;
#[doc = "Register `FLASHCFG` writer"]
pub type W = crate::W<FlashcfgSpec>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock source select (depends on CPE)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksel {
    #[doc = "0: HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    Refclk = 0,
    #[doc = "1: HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    Srcclk = 1,
}
impl From<Clksel> for bool {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSEL` reader - Clock source select (depends on CPE)"]
pub type ClkselR = crate::BitReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            false => Clksel::Refclk,
            true => Clksel::Srcclk,
        }
    }
    #[doc = "HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    #[inline(always)]
    pub fn is_refclk(&self) -> bool {
        *self == Clksel::Refclk
    }
    #[doc = "HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    #[inline(always)]
    pub fn is_srcclk(&self) -> bool {
        *self == Clksel::Srcclk
    }
}
#[doc = "Field `CLKSEL` writer - Clock source select (depends on CPE)"]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    #[inline(always)]
    pub fn refclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Refclk)
    }
    #[doc = "HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    #[inline(always)]
    pub fn srcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Srcclk)
    }
}
#[doc = "Field `DIVEN` reader - Enable divider (depends on CPE)"]
pub type DivenR = crate::BitReader;
#[doc = "Field `DIVEN` writer - Enable divider (depends on CPE)"]
pub type DivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVN` reader - Divider coefficient"]
pub type DivnR = crate::FieldReader;
#[doc = "Field `DIVN` writer - Divider coefficient"]
pub type DivnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Clock source select (depends on CPE)"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable divider (depends on CPE)"]
    #[inline(always)]
    pub fn diven(&self) -> DivenR {
        DivenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&self) -> DivnR {
        DivnR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, FlashcfgSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bit 8 - Clock source select (depends on CPE)"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, FlashcfgSpec> {
        ClkselW::new(self, 8)
    }
    #[doc = "Bit 12 - Enable divider (depends on CPE)"]
    #[inline(always)]
    pub fn diven(&mut self) -> DivenW<'_, FlashcfgSpec> {
        DivenW::new(self, 12)
    }
    #[doc = "Bits 16:21 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&mut self) -> DivnW<'_, FlashcfgSpec> {
        DivnW::new(self, 16)
    }
}
#[doc = "Flash Clock Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcfgSpec;
impl crate::RegisterSpec for FlashcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcfg::R`](R) reader structure"]
impl crate::Readable for FlashcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcfg::W`](W) writer structure"]
impl crate::Writable for FlashcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCFG to value 0x0001_0001"]
impl crate::Resettable for FlashcfgSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
