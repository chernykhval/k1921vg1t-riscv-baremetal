#[doc = "Register `CGCFGAPB1` reader"]
pub type R = crate::R<Cgcfgapb1Spec>;
#[doc = "Register `CGCFGAPB1` writer"]
pub type W = crate::W<Cgcfgapb1Spec>;
#[doc = "Field `ACMP0EN` reader - "]
pub type Acmp0enR = crate::BitReader;
#[doc = "Field `ACMP0EN` writer - "]
pub type Acmp0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1EN` reader - "]
pub type Acmp1enR = crate::BitReader;
#[doc = "Field `ACMP1EN` writer - "]
pub type Acmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP2EN` reader - "]
pub type Acmp2enR = crate::BitReader;
#[doc = "Field `ACMP2EN` writer - "]
pub type Acmp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP3EN` reader - "]
pub type Acmp3enR = crate::BitReader;
#[doc = "Field `ACMP3EN` writer - "]
pub type Acmp3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC0EN` reader - "]
pub type Dac0enR = crate::BitReader;
#[doc = "Field `DAC0EN` writer - "]
pub type Dac0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1EN` reader - "]
pub type Dac1enR = crate::BitReader;
#[doc = "Field `DAC1EN` writer - "]
pub type Dac1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0EN` reader - "]
pub type I2s0enR = crate::BitReader;
#[doc = "Field `I2S0EN` writer - "]
pub type I2s0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1EN` reader - "]
pub type I2s1enR = crate::BitReader;
#[doc = "Field `I2S1EN` writer - "]
pub type I2s1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0EN` reader - "]
pub type Tmr0enR = crate::BitReader;
#[doc = "Field `TMR0EN` writer - "]
pub type Tmr0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1EN` reader - "]
pub type Tmr1enR = crate::BitReader;
#[doc = "Field `TMR1EN` writer - "]
pub type Tmr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2EN` reader - "]
pub type Tmr2enR = crate::BitReader;
#[doc = "Field `TMR2EN` writer - "]
pub type Tmr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3EN` reader - "]
pub type Tmr3enR = crate::BitReader;
#[doc = "Field `TMR3EN` writer - "]
pub type Tmr3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4EN` reader - "]
pub type Tmr4enR = crate::BitReader;
#[doc = "Field `TMR4EN` writer - "]
pub type Tmr4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5EN` reader - "]
pub type Tmr5enR = crate::BitReader;
#[doc = "Field `TMR5EN` writer - "]
pub type Tmr5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6EN` reader - "]
pub type Tmr6enR = crate::BitReader;
#[doc = "Field `TMR6EN` writer - "]
pub type Tmr6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7EN` reader - "]
pub type Tmr7enR = crate::BitReader;
#[doc = "Field `TMR7EN` writer - "]
pub type Tmr7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8EN` reader - "]
pub type Tmr8enR = crate::BitReader;
#[doc = "Field `TMR8EN` writer - "]
pub type Tmr8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9EN` reader - "]
pub type Tmr9enR = crate::BitReader;
#[doc = "Field `TMR9EN` writer - "]
pub type Tmr9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10EN` reader - "]
pub type Tmr10enR = crate::BitReader;
#[doc = "Field `TMR10EN` writer - "]
pub type Tmr10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11EN` reader - "]
pub type Tmr11enR = crate::BitReader;
#[doc = "Field `TMR11EN` writer - "]
pub type Tmr11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR12EN` reader - "]
pub type Tmr12enR = crate::BitReader;
#[doc = "Field `TMR12EN` writer - "]
pub type Tmr12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13EN` reader - "]
pub type Tmr13enR = crate::BitReader;
#[doc = "Field `TMR13EN` writer - "]
pub type Tmr13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14EN` reader - "]
pub type Tmr14enR = crate::BitReader;
#[doc = "Field `TMR14EN` writer - "]
pub type Tmr14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15EN` reader - "]
pub type Tmr15enR = crate::BitReader;
#[doc = "Field `TMR15EN` writer - "]
pub type Tmr15enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acmp0en(&self) -> Acmp0enR {
        Acmp0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acmp1en(&self) -> Acmp1enR {
        Acmp1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn acmp2en(&self) -> Acmp2enR {
        Acmp2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn acmp3en(&self) -> Acmp3enR {
        Acmp3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dac0en(&self) -> Dac0enR {
        Dac0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dac1en(&self) -> Dac1enR {
        Dac1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s0en(&self) -> I2s0enR {
        I2s0enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s1en(&self) -> I2s1enR {
        I2s1enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tmr0en(&self) -> Tmr0enR {
        Tmr0enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tmr1en(&self) -> Tmr1enR {
        Tmr1enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tmr2en(&self) -> Tmr2enR {
        Tmr2enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tmr3en(&self) -> Tmr3enR {
        Tmr3enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tmr4en(&self) -> Tmr4enR {
        Tmr4enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tmr5en(&self) -> Tmr5enR {
        Tmr5enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tmr6en(&self) -> Tmr6enR {
        Tmr6enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tmr7en(&self) -> Tmr7enR {
        Tmr7enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tmr8en(&self) -> Tmr8enR {
        Tmr8enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tmr9en(&self) -> Tmr9enR {
        Tmr9enR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tmr10en(&self) -> Tmr10enR {
        Tmr10enR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tmr11en(&self) -> Tmr11enR {
        Tmr11enR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tmr12en(&self) -> Tmr12enR {
        Tmr12enR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tmr13en(&self) -> Tmr13enR {
        Tmr13enR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tmr14en(&self) -> Tmr14enR {
        Tmr14enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tmr15en(&self) -> Tmr15enR {
        Tmr15enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acmp0en(&mut self) -> Acmp0enW<'_, Cgcfgapb1Spec> {
        Acmp0enW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acmp1en(&mut self) -> Acmp1enW<'_, Cgcfgapb1Spec> {
        Acmp1enW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn acmp2en(&mut self) -> Acmp2enW<'_, Cgcfgapb1Spec> {
        Acmp2enW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn acmp3en(&mut self) -> Acmp3enW<'_, Cgcfgapb1Spec> {
        Acmp3enW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dac0en(&mut self) -> Dac0enW<'_, Cgcfgapb1Spec> {
        Dac0enW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dac1en(&mut self) -> Dac1enW<'_, Cgcfgapb1Spec> {
        Dac1enW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s0en(&mut self) -> I2s0enW<'_, Cgcfgapb1Spec> {
        I2s0enW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s1en(&mut self) -> I2s1enW<'_, Cgcfgapb1Spec> {
        I2s1enW::new(self, 7)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tmr0en(&mut self) -> Tmr0enW<'_, Cgcfgapb1Spec> {
        Tmr0enW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tmr1en(&mut self) -> Tmr1enW<'_, Cgcfgapb1Spec> {
        Tmr1enW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tmr2en(&mut self) -> Tmr2enW<'_, Cgcfgapb1Spec> {
        Tmr2enW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tmr3en(&mut self) -> Tmr3enW<'_, Cgcfgapb1Spec> {
        Tmr3enW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tmr4en(&mut self) -> Tmr4enW<'_, Cgcfgapb1Spec> {
        Tmr4enW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tmr5en(&mut self) -> Tmr5enW<'_, Cgcfgapb1Spec> {
        Tmr5enW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tmr6en(&mut self) -> Tmr6enW<'_, Cgcfgapb1Spec> {
        Tmr6enW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tmr7en(&mut self) -> Tmr7enW<'_, Cgcfgapb1Spec> {
        Tmr7enW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tmr8en(&mut self) -> Tmr8enW<'_, Cgcfgapb1Spec> {
        Tmr8enW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tmr9en(&mut self) -> Tmr9enW<'_, Cgcfgapb1Spec> {
        Tmr9enW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tmr10en(&mut self) -> Tmr10enW<'_, Cgcfgapb1Spec> {
        Tmr10enW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tmr11en(&mut self) -> Tmr11enW<'_, Cgcfgapb1Spec> {
        Tmr11enW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tmr12en(&mut self) -> Tmr12enW<'_, Cgcfgapb1Spec> {
        Tmr12enW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tmr13en(&mut self) -> Tmr13enW<'_, Cgcfgapb1Spec> {
        Tmr13enW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tmr14en(&mut self) -> Tmr14enW<'_, Cgcfgapb1Spec> {
        Tmr14enW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tmr15en(&mut self) -> Tmr15enW<'_, Cgcfgapb1Spec> {
        Tmr15enW::new(self, 31)
    }
}
#[doc = "Clockgate Config APB1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcfgapb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcfgapb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgcfgapb1Spec;
impl crate::RegisterSpec for Cgcfgapb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgcfgapb1::R`](R) reader structure"]
impl crate::Readable for Cgcfgapb1Spec {}
#[doc = "`write(|w| ..)` method takes [`cgcfgapb1::W`](W) writer structure"]
impl crate::Writable for Cgcfgapb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGCFGAPB1 to value 0"]
impl crate::Resettable for Cgcfgapb1Spec {}
