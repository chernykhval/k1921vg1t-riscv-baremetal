#[doc = "Register `BGPCTRL` reader"]
pub type R = crate::R<BgpctrlSpec>;
#[doc = "Register `BGPCTRL` writer"]
pub type W = crate::W<BgpctrlSpec>;
#[doc = "Field `TUNE` reader - Tune bandgap voltage"]
pub type TuneR = crate::FieldReader<u16>;
#[doc = "Field `TUNE` writer - Tune bandgap voltage"]
pub type TuneW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EN` reader - Enable bandgap"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable bandgap"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Tune bandgap voltage"]
    #[inline(always)]
    pub fn tune(&self) -> TuneR {
        TuneR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Enable bandgap"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Tune bandgap voltage"]
    #[inline(always)]
    pub fn tune(&mut self) -> TuneW<'_, BgpctrlSpec> {
        TuneW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable bandgap"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, BgpctrlSpec> {
        EnW::new(self, 16)
    }
}
#[doc = "Bandgap control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgpctrlSpec;
impl crate::RegisterSpec for BgpctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgpctrl::R`](R) reader structure"]
impl crate::Readable for BgpctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bgpctrl::W`](W) writer structure"]
impl crate::Writable for BgpctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BGPCTRL to value 0x0001_0800"]
impl crate::Resettable for BgpctrlSpec {
    const RESET_VALUE: u32 = 0x0001_0800;
}
