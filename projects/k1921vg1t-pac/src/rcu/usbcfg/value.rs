#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: Clock from PLL OUT\\[0\\]"]
    Pllsysclk = 0,
    #[doc = "1: Clock from PLL OUT\\[1\\]"]
    Pll1clk = 1,
    #[doc = "2: Clock from PLL OUT\\[2\\]"]
    Pll2clk = 2,
    #[doc = "3: External clock source (GPIO)"]
    Extclk = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock source select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::Pllsysclk,
            1 => Clksel::Pll1clk,
            2 => Clksel::Pll2clk,
            3 => Clksel::Extclk,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock from PLL OUT\\[0\\]"]
    #[inline(always)]
    pub fn is_pllsysclk(&self) -> bool {
        *self == Clksel::Pllsysclk
    }
    #[doc = "Clock from PLL OUT\\[1\\]"]
    #[inline(always)]
    pub fn is_pll1clk(&self) -> bool {
        *self == Clksel::Pll1clk
    }
    #[doc = "Clock from PLL OUT\\[2\\]"]
    #[inline(always)]
    pub fn is_pll2clk(&self) -> bool {
        *self == Clksel::Pll2clk
    }
    #[doc = "External clock source (GPIO)"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == Clksel::Extclk
    }
}
#[doc = "Field `CLKSEL` writer - Clock source select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock from PLL OUT\\[0\\]"]
    #[inline(always)]
    pub fn pllsysclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Pllsysclk)
    }
    #[doc = "Clock from PLL OUT\\[1\\]"]
    #[inline(always)]
    pub fn pll1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Pll1clk)
    }
    #[doc = "Clock from PLL OUT\\[2\\]"]
    #[inline(always)]
    pub fn pll2clk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Pll2clk)
    }
    #[doc = "External clock source (GPIO)"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Extclk)
    }
}
#[doc = "Field `DIVEN` reader - Enable divider"]
pub type DivenR = crate::BitReader;
#[doc = "Field `DIVEN` writer - Enable divider"]
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
    #[doc = "Bits 8:9 - Clock source select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Enable divider"]
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
    pub fn clken(&mut self) -> ClkenW<'_, ValueSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Clock source select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, ValueSpec> {
        ClkselW::new(self, 8)
    }
    #[doc = "Bit 12 - Enable divider"]
    #[inline(always)]
    pub fn diven(&mut self) -> DivenW<'_, ValueSpec> {
        DivenW::new(self, 12)
    }
    #[doc = "Bits 16:21 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&mut self) -> DivnW<'_, ValueSpec> {
        DivnW::new(self, 16)
    }
}
#[doc = "USB Clock and Reset Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
