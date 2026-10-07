#[doc = "Register `PWMEN` reader"]
pub type R = crate::R<PwmenSpec>;
#[doc = "Register `PWMEN` writer"]
pub type W = crate::W<PwmenSpec>;
#[doc = "Field `PWM0EN` reader - Enable count for PWM 0"]
pub type Pwm0enR = crate::BitReader;
#[doc = "Field `PWM0EN` writer - Enable count for PWM 0"]
pub type Pwm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1EN` reader - Enable count for PWM 1"]
pub type Pwm1enR = crate::BitReader;
#[doc = "Field `PWM1EN` writer - Enable count for PWM 1"]
pub type Pwm1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2EN` reader - Enable count for PWM 2"]
pub type Pwm2enR = crate::BitReader;
#[doc = "Field `PWM2EN` writer - Enable count for PWM 2"]
pub type Pwm2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3EN` reader - Enable count for PWM 3"]
pub type Pwm3enR = crate::BitReader;
#[doc = "Field `PWM3EN` writer - Enable count for PWM 3"]
pub type Pwm3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4EN` reader - Enable count for PWM 4"]
pub type Pwm4enR = crate::BitReader;
#[doc = "Field `PWM4EN` writer - Enable count for PWM 4"]
pub type Pwm4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM5EN` reader - Enable count for PWM 5"]
pub type Pwm5enR = crate::BitReader;
#[doc = "Field `PWM5EN` writer - Enable count for PWM 5"]
pub type Pwm5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM6EN` reader - Enable count for PWM 6"]
pub type Pwm6enR = crate::BitReader;
#[doc = "Field `PWM6EN` writer - Enable count for PWM 6"]
pub type Pwm6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM7EN` reader - Enable count for PWM 7"]
pub type Pwm7enR = crate::BitReader;
#[doc = "Field `PWM7EN` writer - Enable count for PWM 7"]
pub type Pwm7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM8EN` reader - Enable count for PWM 8"]
pub type Pwm8enR = crate::BitReader;
#[doc = "Field `PWM8EN` writer - Enable count for PWM 8"]
pub type Pwm8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM9EN` reader - Enable count for PWM 9"]
pub type Pwm9enR = crate::BitReader;
#[doc = "Field `PWM9EN` writer - Enable count for PWM 9"]
pub type Pwm9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM10EN` reader - Enable count for PWM 10"]
pub type Pwm10enR = crate::BitReader;
#[doc = "Field `PWM10EN` writer - Enable count for PWM 10"]
pub type Pwm10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM11EN` reader - Enable count for PWM 11"]
pub type Pwm11enR = crate::BitReader;
#[doc = "Field `PWM11EN` writer - Enable count for PWM 11"]
pub type Pwm11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM12EN` reader - Enable count for PWM 12"]
pub type Pwm12enR = crate::BitReader;
#[doc = "Field `PWM12EN` writer - Enable count for PWM 12"]
pub type Pwm12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM13EN` reader - Enable count for PWM 13"]
pub type Pwm13enR = crate::BitReader;
#[doc = "Field `PWM13EN` writer - Enable count for PWM 13"]
pub type Pwm13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM14EN` reader - Enable count for PWM 14"]
pub type Pwm14enR = crate::BitReader;
#[doc = "Field `PWM14EN` writer - Enable count for PWM 14"]
pub type Pwm14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM15EN` reader - Enable count for PWM 15"]
pub type Pwm15enR = crate::BitReader;
#[doc = "Field `PWM15EN` writer - Enable count for PWM 15"]
pub type Pwm15enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable count for PWM 0"]
    #[inline(always)]
    pub fn pwm0en(&self) -> Pwm0enR {
        Pwm0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable count for PWM 1"]
    #[inline(always)]
    pub fn pwm1en(&self) -> Pwm1enR {
        Pwm1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable count for PWM 2"]
    #[inline(always)]
    pub fn pwm2en(&self) -> Pwm2enR {
        Pwm2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable count for PWM 3"]
    #[inline(always)]
    pub fn pwm3en(&self) -> Pwm3enR {
        Pwm3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable count for PWM 4"]
    #[inline(always)]
    pub fn pwm4en(&self) -> Pwm4enR {
        Pwm4enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable count for PWM 5"]
    #[inline(always)]
    pub fn pwm5en(&self) -> Pwm5enR {
        Pwm5enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable count for PWM 6"]
    #[inline(always)]
    pub fn pwm6en(&self) -> Pwm6enR {
        Pwm6enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable count for PWM 7"]
    #[inline(always)]
    pub fn pwm7en(&self) -> Pwm7enR {
        Pwm7enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable count for PWM 8"]
    #[inline(always)]
    pub fn pwm8en(&self) -> Pwm8enR {
        Pwm8enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable count for PWM 9"]
    #[inline(always)]
    pub fn pwm9en(&self) -> Pwm9enR {
        Pwm9enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable count for PWM 10"]
    #[inline(always)]
    pub fn pwm10en(&self) -> Pwm10enR {
        Pwm10enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable count for PWM 11"]
    #[inline(always)]
    pub fn pwm11en(&self) -> Pwm11enR {
        Pwm11enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable count for PWM 12"]
    #[inline(always)]
    pub fn pwm12en(&self) -> Pwm12enR {
        Pwm12enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable count for PWM 13"]
    #[inline(always)]
    pub fn pwm13en(&self) -> Pwm13enR {
        Pwm13enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable count for PWM 14"]
    #[inline(always)]
    pub fn pwm14en(&self) -> Pwm14enR {
        Pwm14enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable count for PWM 15"]
    #[inline(always)]
    pub fn pwm15en(&self) -> Pwm15enR {
        Pwm15enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable count for PWM 0"]
    #[inline(always)]
    pub fn pwm0en(&mut self) -> Pwm0enW<'_, PwmenSpec> {
        Pwm0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable count for PWM 1"]
    #[inline(always)]
    pub fn pwm1en(&mut self) -> Pwm1enW<'_, PwmenSpec> {
        Pwm1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable count for PWM 2"]
    #[inline(always)]
    pub fn pwm2en(&mut self) -> Pwm2enW<'_, PwmenSpec> {
        Pwm2enW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable count for PWM 3"]
    #[inline(always)]
    pub fn pwm3en(&mut self) -> Pwm3enW<'_, PwmenSpec> {
        Pwm3enW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable count for PWM 4"]
    #[inline(always)]
    pub fn pwm4en(&mut self) -> Pwm4enW<'_, PwmenSpec> {
        Pwm4enW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable count for PWM 5"]
    #[inline(always)]
    pub fn pwm5en(&mut self) -> Pwm5enW<'_, PwmenSpec> {
        Pwm5enW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable count for PWM 6"]
    #[inline(always)]
    pub fn pwm6en(&mut self) -> Pwm6enW<'_, PwmenSpec> {
        Pwm6enW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable count for PWM 7"]
    #[inline(always)]
    pub fn pwm7en(&mut self) -> Pwm7enW<'_, PwmenSpec> {
        Pwm7enW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable count for PWM 8"]
    #[inline(always)]
    pub fn pwm8en(&mut self) -> Pwm8enW<'_, PwmenSpec> {
        Pwm8enW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable count for PWM 9"]
    #[inline(always)]
    pub fn pwm9en(&mut self) -> Pwm9enW<'_, PwmenSpec> {
        Pwm9enW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable count for PWM 10"]
    #[inline(always)]
    pub fn pwm10en(&mut self) -> Pwm10enW<'_, PwmenSpec> {
        Pwm10enW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable count for PWM 11"]
    #[inline(always)]
    pub fn pwm11en(&mut self) -> Pwm11enW<'_, PwmenSpec> {
        Pwm11enW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable count for PWM 12"]
    #[inline(always)]
    pub fn pwm12en(&mut self) -> Pwm12enW<'_, PwmenSpec> {
        Pwm12enW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable count for PWM 13"]
    #[inline(always)]
    pub fn pwm13en(&mut self) -> Pwm13enW<'_, PwmenSpec> {
        Pwm13enW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable count for PWM 14"]
    #[inline(always)]
    pub fn pwm14en(&mut self) -> Pwm14enW<'_, PwmenSpec> {
        Pwm14enW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable count for PWM 15"]
    #[inline(always)]
    pub fn pwm15en(&mut self) -> Pwm15enW<'_, PwmenSpec> {
        Pwm15enW::new(self, 15)
    }
}
#[doc = "PWM counting enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmenSpec;
impl crate::RegisterSpec for PwmenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmen::R`](R) reader structure"]
impl crate::Readable for PwmenSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmen::W`](W) writer structure"]
impl crate::Writable for PwmenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMEN to value 0"]
impl crate::Resettable for PwmenSpec {}
