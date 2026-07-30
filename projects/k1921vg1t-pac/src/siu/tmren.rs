#[doc = "Register `TMREN` reader"]
pub type R = crate::R<TmrenSpec>;
#[doc = "Register `TMREN` writer"]
pub type W = crate::W<TmrenSpec>;
#[doc = "Field `TMR0EN` reader - Enable count for TMR 0"]
pub type Tmr0enR = crate::BitReader;
#[doc = "Field `TMR0EN` writer - Enable count for TMR 0"]
pub type Tmr0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1EN` reader - Enable count for TMR 1"]
pub type Tmr1enR = crate::BitReader;
#[doc = "Field `TMR1EN` writer - Enable count for TMR 1"]
pub type Tmr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2EN` reader - Enable count for timer 2"]
pub type Tmr2enR = crate::BitReader;
#[doc = "Field `TMR2EN` writer - Enable count for timer 2"]
pub type Tmr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3EN` reader - Enable count for timer 3"]
pub type Tmr3enR = crate::BitReader;
#[doc = "Field `TMR3EN` writer - Enable count for timer 3"]
pub type Tmr3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4EN` reader - Enable count for timer 4"]
pub type Tmr4enR = crate::BitReader;
#[doc = "Field `TMR4EN` writer - Enable count for timer 4"]
pub type Tmr4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5EN` reader - Enable count for timer 5"]
pub type Tmr5enR = crate::BitReader;
#[doc = "Field `TMR5EN` writer - Enable count for timer 5"]
pub type Tmr5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6EN` reader - Enable count for timer 6"]
pub type Tmr6enR = crate::BitReader;
#[doc = "Field `TMR6EN` writer - Enable count for timer 6"]
pub type Tmr6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7EN` reader - Enable count for timer 7"]
pub type Tmr7enR = crate::BitReader;
#[doc = "Field `TMR7EN` writer - Enable count for timer 7"]
pub type Tmr7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8EN` reader - Enable count for timer 8"]
pub type Tmr8enR = crate::BitReader;
#[doc = "Field `TMR8EN` writer - Enable count for timer 8"]
pub type Tmr8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9EN` reader - Enable count for timer 9"]
pub type Tmr9enR = crate::BitReader;
#[doc = "Field `TMR9EN` writer - Enable count for timer 9"]
pub type Tmr9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10EN` reader - Enable count for timer 10"]
pub type Tmr10enR = crate::BitReader;
#[doc = "Field `TMR10EN` writer - Enable count for timer 10"]
pub type Tmr10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11EN` reader - Enable count for timer 11"]
pub type Tmr11enR = crate::BitReader;
#[doc = "Field `TMR11EN` writer - Enable count for timer 11"]
pub type Tmr11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR12EN` reader - Enable count for timer 12"]
pub type Tmr12enR = crate::BitReader;
#[doc = "Field `TMR12EN` writer - Enable count for timer 12"]
pub type Tmr12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13EN` reader - Enable count for timer 13"]
pub type Tmr13enR = crate::BitReader;
#[doc = "Field `TMR13EN` writer - Enable count for timer 13"]
pub type Tmr13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14EN` reader - Enable count for timer 14"]
pub type Tmr14enR = crate::BitReader;
#[doc = "Field `TMR14EN` writer - Enable count for timer 14"]
pub type Tmr14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15EN` reader - Enable count for timer 15"]
pub type Tmr15enR = crate::BitReader;
#[doc = "Field `TMR15EN` writer - Enable count for timer 15"]
pub type Tmr15enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable count for TMR 0"]
    #[inline(always)]
    pub fn tmr0en(&self) -> Tmr0enR {
        Tmr0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable count for TMR 1"]
    #[inline(always)]
    pub fn tmr1en(&self) -> Tmr1enR {
        Tmr1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable count for timer 2"]
    #[inline(always)]
    pub fn tmr2en(&self) -> Tmr2enR {
        Tmr2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable count for timer 3"]
    #[inline(always)]
    pub fn tmr3en(&self) -> Tmr3enR {
        Tmr3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable count for timer 4"]
    #[inline(always)]
    pub fn tmr4en(&self) -> Tmr4enR {
        Tmr4enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable count for timer 5"]
    #[inline(always)]
    pub fn tmr5en(&self) -> Tmr5enR {
        Tmr5enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable count for timer 6"]
    #[inline(always)]
    pub fn tmr6en(&self) -> Tmr6enR {
        Tmr6enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable count for timer 7"]
    #[inline(always)]
    pub fn tmr7en(&self) -> Tmr7enR {
        Tmr7enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable count for timer 8"]
    #[inline(always)]
    pub fn tmr8en(&self) -> Tmr8enR {
        Tmr8enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable count for timer 9"]
    #[inline(always)]
    pub fn tmr9en(&self) -> Tmr9enR {
        Tmr9enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable count for timer 10"]
    #[inline(always)]
    pub fn tmr10en(&self) -> Tmr10enR {
        Tmr10enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable count for timer 11"]
    #[inline(always)]
    pub fn tmr11en(&self) -> Tmr11enR {
        Tmr11enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable count for timer 12"]
    #[inline(always)]
    pub fn tmr12en(&self) -> Tmr12enR {
        Tmr12enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable count for timer 13"]
    #[inline(always)]
    pub fn tmr13en(&self) -> Tmr13enR {
        Tmr13enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable count for timer 14"]
    #[inline(always)]
    pub fn tmr14en(&self) -> Tmr14enR {
        Tmr14enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable count for timer 15"]
    #[inline(always)]
    pub fn tmr15en(&self) -> Tmr15enR {
        Tmr15enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable count for TMR 0"]
    #[inline(always)]
    pub fn tmr0en(&mut self) -> Tmr0enW<'_, TmrenSpec> {
        Tmr0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable count for TMR 1"]
    #[inline(always)]
    pub fn tmr1en(&mut self) -> Tmr1enW<'_, TmrenSpec> {
        Tmr1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable count for timer 2"]
    #[inline(always)]
    pub fn tmr2en(&mut self) -> Tmr2enW<'_, TmrenSpec> {
        Tmr2enW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable count for timer 3"]
    #[inline(always)]
    pub fn tmr3en(&mut self) -> Tmr3enW<'_, TmrenSpec> {
        Tmr3enW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable count for timer 4"]
    #[inline(always)]
    pub fn tmr4en(&mut self) -> Tmr4enW<'_, TmrenSpec> {
        Tmr4enW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable count for timer 5"]
    #[inline(always)]
    pub fn tmr5en(&mut self) -> Tmr5enW<'_, TmrenSpec> {
        Tmr5enW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable count for timer 6"]
    #[inline(always)]
    pub fn tmr6en(&mut self) -> Tmr6enW<'_, TmrenSpec> {
        Tmr6enW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable count for timer 7"]
    #[inline(always)]
    pub fn tmr7en(&mut self) -> Tmr7enW<'_, TmrenSpec> {
        Tmr7enW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable count for timer 8"]
    #[inline(always)]
    pub fn tmr8en(&mut self) -> Tmr8enW<'_, TmrenSpec> {
        Tmr8enW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable count for timer 9"]
    #[inline(always)]
    pub fn tmr9en(&mut self) -> Tmr9enW<'_, TmrenSpec> {
        Tmr9enW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable count for timer 10"]
    #[inline(always)]
    pub fn tmr10en(&mut self) -> Tmr10enW<'_, TmrenSpec> {
        Tmr10enW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable count for timer 11"]
    #[inline(always)]
    pub fn tmr11en(&mut self) -> Tmr11enW<'_, TmrenSpec> {
        Tmr11enW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable count for timer 12"]
    #[inline(always)]
    pub fn tmr12en(&mut self) -> Tmr12enW<'_, TmrenSpec> {
        Tmr12enW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable count for timer 13"]
    #[inline(always)]
    pub fn tmr13en(&mut self) -> Tmr13enW<'_, TmrenSpec> {
        Tmr13enW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable count for timer 14"]
    #[inline(always)]
    pub fn tmr14en(&mut self) -> Tmr14enW<'_, TmrenSpec> {
        Tmr14enW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable count for timer 15"]
    #[inline(always)]
    pub fn tmr15en(&mut self) -> Tmr15enW<'_, TmrenSpec> {
        Tmr15enW::new(self, 15)
    }
}
#[doc = "Timers counting enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrenSpec;
impl crate::RegisterSpec for TmrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmren::R`](R) reader structure"]
impl crate::Readable for TmrenSpec {}
#[doc = "`write(|w| ..)` method takes [`tmren::W`](W) writer structure"]
impl crate::Writable for TmrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMREN to value 0"]
impl crate::Resettable for TmrenSpec {}
