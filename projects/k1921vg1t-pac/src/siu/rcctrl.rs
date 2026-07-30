#[doc = "Register `RCCTRL` reader"]
pub type R = crate::R<RcctrlSpec>;
#[doc = "Register `RCCTRL` writer"]
pub type W = crate::W<RcctrlSpec>;
#[doc = "Field `TUNE` reader - Tune RC generator period"]
pub type TuneR = crate::FieldReader;
#[doc = "Field `TUNE` writer - Tune RC generator period"]
pub type TuneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EN` reader - Internal oscillator enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Internal oscillator enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Tune RC generator period"]
    #[inline(always)]
    pub fn tune(&self) -> TuneR {
        TuneR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Internal oscillator enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tune RC generator period"]
    #[inline(always)]
    pub fn tune(&mut self) -> TuneW<'_, RcctrlSpec> {
        TuneW::new(self, 0)
    }
    #[doc = "Bit 8 - Internal oscillator enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, RcctrlSpec> {
        EnW::new(self, 8)
    }
}
#[doc = "RC generator control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcctrlSpec;
impl crate::RegisterSpec for RcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcctrl::R`](R) reader structure"]
impl crate::Readable for RcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rcctrl::W`](W) writer structure"]
impl crate::Writable for RcctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCCTRL to value 0x0142"]
impl crate::Resettable for RcctrlSpec {
    const RESET_VALUE: u32 = 0x0142;
}
