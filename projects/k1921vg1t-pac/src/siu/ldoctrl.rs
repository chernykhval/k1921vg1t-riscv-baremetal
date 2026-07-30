#[doc = "Register `LDOCTRL` reader"]
pub type R = crate::R<LdoctrlSpec>;
#[doc = "Register `LDOCTRL` writer"]
pub type W = crate::W<LdoctrlSpec>;
#[doc = "Field `TUNE` reader - Tune LDO output voltage"]
pub type TuneR = crate::FieldReader;
#[doc = "Field `TUNE` writer - Tune LDO output voltage"]
pub type TuneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Disable 100kHz oscillator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disosc {
    #[doc = "0: Enable 100kHz oscillator"]
    Enableosc = 0,
    #[doc = "1: Disable 100kHz oscillator"]
    Disableosc = 1,
}
impl From<Disosc> for bool {
    #[inline(always)]
    fn from(variant: Disosc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISOSC` reader - Disable 100kHz oscillator"]
pub type DisoscR = crate::BitReader<Disosc>;
impl DisoscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disosc {
        match self.bits {
            false => Disosc::Enableosc,
            true => Disosc::Disableosc,
        }
    }
    #[doc = "Enable 100kHz oscillator"]
    #[inline(always)]
    pub fn is_enableosc(&self) -> bool {
        *self == Disosc::Enableosc
    }
    #[doc = "Disable 100kHz oscillator"]
    #[inline(always)]
    pub fn is_disableosc(&self) -> bool {
        *self == Disosc::Disableosc
    }
}
#[doc = "Field `DISOSC` writer - Disable 100kHz oscillator"]
pub type DisoscW<'a, REG> = crate::BitWriter<'a, REG, Disosc>;
impl<'a, REG> DisoscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable 100kHz oscillator"]
    #[inline(always)]
    pub fn enableosc(self) -> &'a mut crate::W<REG> {
        self.variant(Disosc::Enableosc)
    }
    #[doc = "Disable 100kHz oscillator"]
    #[inline(always)]
    pub fn disableosc(self) -> &'a mut crate::W<REG> {
        self.variant(Disosc::Disableosc)
    }
}
impl R {
    #[doc = "Bits 0:3 - Tune LDO output voltage"]
    #[inline(always)]
    pub fn tune(&self) -> TuneR {
        TuneR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Disable 100kHz oscillator"]
    #[inline(always)]
    pub fn disosc(&self) -> DisoscR {
        DisoscR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Tune LDO output voltage"]
    #[inline(always)]
    pub fn tune(&mut self) -> TuneW<'_, LdoctrlSpec> {
        TuneW::new(self, 0)
    }
    #[doc = "Bit 8 - Disable 100kHz oscillator"]
    #[inline(always)]
    pub fn disosc(&mut self) -> DisoscW<'_, LdoctrlSpec> {
        DisoscW::new(self, 8)
    }
}
#[doc = "LDO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LdoctrlSpec;
impl crate::RegisterSpec for LdoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldoctrl::R`](R) reader structure"]
impl crate::Readable for LdoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ldoctrl::W`](W) writer structure"]
impl crate::Writable for LdoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDOCTRL to value 0x0100"]
impl crate::Resettable for LdoctrlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
