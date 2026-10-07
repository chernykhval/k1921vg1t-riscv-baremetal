#[doc = "Register `CGCFGAPB0` reader"]
pub type R = crate::R<Cgcfgapb0Spec>;
#[doc = "Register `CGCFGAPB0` writer"]
pub type W = crate::W<Cgcfgapb0Spec>;
#[doc = "Field `DMAEN` reader - "]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - "]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCEN` reader - "]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - "]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTEN` reader - "]
pub type WdtenR = crate::BitReader;
#[doc = "Field `WDTEN` writer - "]
pub type WdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNGEN` reader - "]
pub type TrngenR = crate::BitReader;
#[doc = "Field `TRNGEN` writer - "]
pub type TrngenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMCEN` reader - "]
pub type EmcenR = crate::BitReader;
#[doc = "Field `EMCEN` writer - "]
pub type EmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDRAMEN` reader - "]
pub type SdramenR = crate::BitReader;
#[doc = "Field `SDRAMEN` writer - "]
pub type SdramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHEN` reader - "]
pub type EthenR = crate::BitReader;
#[doc = "Field `ETHEN` writer - "]
pub type EthenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0EN` reader - "]
pub type Pwm0enR = crate::BitReader;
#[doc = "Field `PWM0EN` writer - "]
pub type Pwm0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1EN` reader - "]
pub type Pwm1enR = crate::BitReader;
#[doc = "Field `PWM1EN` writer - "]
pub type Pwm1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2EN` reader - "]
pub type Pwm2enR = crate::BitReader;
#[doc = "Field `PWM2EN` writer - "]
pub type Pwm2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3EN` reader - "]
pub type Pwm3enR = crate::BitReader;
#[doc = "Field `PWM3EN` writer - "]
pub type Pwm3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM4EN` reader - "]
pub type Pwm4enR = crate::BitReader;
#[doc = "Field `PWM4EN` writer - "]
pub type Pwm4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM5EN` reader - "]
pub type Pwm5enR = crate::BitReader;
#[doc = "Field `PWM5EN` writer - "]
pub type Pwm5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM6EN` reader - "]
pub type Pwm6enR = crate::BitReader;
#[doc = "Field `PWM6EN` writer - "]
pub type Pwm6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM7EN` reader - "]
pub type Pwm7enR = crate::BitReader;
#[doc = "Field `PWM7EN` writer - "]
pub type Pwm7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM8EN` reader - "]
pub type Pwm8enR = crate::BitReader;
#[doc = "Field `PWM8EN` writer - "]
pub type Pwm8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM9EN` reader - "]
pub type Pwm9enR = crate::BitReader;
#[doc = "Field `PWM9EN` writer - "]
pub type Pwm9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM10EN` reader - "]
pub type Pwm10enR = crate::BitReader;
#[doc = "Field `PWM10EN` writer - "]
pub type Pwm10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM11EN` reader - "]
pub type Pwm11enR = crate::BitReader;
#[doc = "Field `PWM11EN` writer - "]
pub type Pwm11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM12EN` reader - "]
pub type Pwm12enR = crate::BitReader;
#[doc = "Field `PWM12EN` writer - "]
pub type Pwm12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM13EN` reader - "]
pub type Pwm13enR = crate::BitReader;
#[doc = "Field `PWM13EN` writer - "]
pub type Pwm13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM14EN` reader - "]
pub type Pwm14enR = crate::BitReader;
#[doc = "Field `PWM14EN` writer - "]
pub type Pwm14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM15EN` reader - "]
pub type Pwm15enR = crate::BitReader;
#[doc = "Field `PWM15EN` writer - "]
pub type Pwm15enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wdten(&self) -> WdtenR {
        WdtenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trngen(&self) -> TrngenR {
        TrngenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn emcen(&self) -> EmcenR {
        EmcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sdramen(&self) -> SdramenR {
        SdramenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ethen(&self) -> EthenR {
        EthenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pwm0en(&self) -> Pwm0enR {
        Pwm0enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwm1en(&self) -> Pwm1enR {
        Pwm1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pwm2en(&self) -> Pwm2enR {
        Pwm2enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwm3en(&self) -> Pwm3enR {
        Pwm3enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pwm4en(&self) -> Pwm4enR {
        Pwm4enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pwm5en(&self) -> Pwm5enR {
        Pwm5enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwm6en(&self) -> Pwm6enR {
        Pwm6enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwm7en(&self) -> Pwm7enR {
        Pwm7enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm8en(&self) -> Pwm8enR {
        Pwm8enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm9en(&self) -> Pwm9enR {
        Pwm9enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pwm10en(&self) -> Pwm10enR {
        Pwm10enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pwm11en(&self) -> Pwm11enR {
        Pwm11enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm12en(&self) -> Pwm12enR {
        Pwm12enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pwm13en(&self) -> Pwm13enR {
        Pwm13enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pwm14en(&self) -> Pwm14enR {
        Pwm14enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pwm15en(&self) -> Pwm15enR {
        Pwm15enR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Cgcfgapb0Spec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<'_, Cgcfgapb0Spec> {
        RtcenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wdten(&mut self) -> WdtenW<'_, Cgcfgapb0Spec> {
        WdtenW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trngen(&mut self) -> TrngenW<'_, Cgcfgapb0Spec> {
        TrngenW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn emcen(&mut self) -> EmcenW<'_, Cgcfgapb0Spec> {
        EmcenW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sdramen(&mut self) -> SdramenW<'_, Cgcfgapb0Spec> {
        SdramenW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ethen(&mut self) -> EthenW<'_, Cgcfgapb0Spec> {
        EthenW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pwm0en(&mut self) -> Pwm0enW<'_, Cgcfgapb0Spec> {
        Pwm0enW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwm1en(&mut self) -> Pwm1enW<'_, Cgcfgapb0Spec> {
        Pwm1enW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pwm2en(&mut self) -> Pwm2enW<'_, Cgcfgapb0Spec> {
        Pwm2enW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwm3en(&mut self) -> Pwm3enW<'_, Cgcfgapb0Spec> {
        Pwm3enW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pwm4en(&mut self) -> Pwm4enW<'_, Cgcfgapb0Spec> {
        Pwm4enW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pwm5en(&mut self) -> Pwm5enW<'_, Cgcfgapb0Spec> {
        Pwm5enW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwm6en(&mut self) -> Pwm6enW<'_, Cgcfgapb0Spec> {
        Pwm6enW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwm7en(&mut self) -> Pwm7enW<'_, Cgcfgapb0Spec> {
        Pwm7enW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm8en(&mut self) -> Pwm8enW<'_, Cgcfgapb0Spec> {
        Pwm8enW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm9en(&mut self) -> Pwm9enW<'_, Cgcfgapb0Spec> {
        Pwm9enW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pwm10en(&mut self) -> Pwm10enW<'_, Cgcfgapb0Spec> {
        Pwm10enW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pwm11en(&mut self) -> Pwm11enW<'_, Cgcfgapb0Spec> {
        Pwm11enW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm12en(&mut self) -> Pwm12enW<'_, Cgcfgapb0Spec> {
        Pwm12enW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pwm13en(&mut self) -> Pwm13enW<'_, Cgcfgapb0Spec> {
        Pwm13enW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pwm14en(&mut self) -> Pwm14enW<'_, Cgcfgapb0Spec> {
        Pwm14enW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pwm15en(&mut self) -> Pwm15enW<'_, Cgcfgapb0Spec> {
        Pwm15enW::new(self, 23)
    }
}
#[doc = "Clockgate Config APB0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcfgapb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcfgapb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgcfgapb0Spec;
impl crate::RegisterSpec for Cgcfgapb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgcfgapb0::R`](R) reader structure"]
impl crate::Readable for Cgcfgapb0Spec {}
#[doc = "`write(|w| ..)` method takes [`cgcfgapb0::W`](W) writer structure"]
impl crate::Writable for Cgcfgapb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGCFGAPB0 to value 0"]
impl crate::Resettable for Cgcfgapb0Spec {}
