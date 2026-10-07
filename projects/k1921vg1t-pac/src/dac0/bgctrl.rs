#[doc = "Register `BGCTRL` reader"]
pub type R = crate::R<BgctrlSpec>;
#[doc = "Register `BGCTRL` writer"]
pub type W = crate::W<BgctrlSpec>;
#[doc = "Field `TUNE` reader - Tune bandgap voltage"]
pub type TuneR = crate::FieldReader<u16>;
#[doc = "Field `TUNE` writer - Tune bandgap voltage"]
pub type TuneW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EN` reader - Turn-on internal reference-voltage source"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Turn-on internal reference-voltage source"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Adjust output voltage of internal reference-voltage source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adjust {
    #[doc = "0: Output 1.5 V"]
    V1p5 = 0,
    #[doc = "1: Output 1.8 V"]
    V1p8 = 1,
    #[doc = "2: Output 2.048 V"]
    V2p048 = 2,
    #[doc = "3: Output 2.5 V"]
    V2p5 = 3,
}
impl From<Adjust> for u8 {
    #[inline(always)]
    fn from(variant: Adjust) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adjust {
    type Ux = u8;
}
impl crate::IsEnum for Adjust {}
#[doc = "Field `ADJUST` reader - Adjust output voltage of internal reference-voltage source"]
pub type AdjustR = crate::FieldReader<Adjust>;
impl AdjustR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adjust {
        match self.bits {
            0 => Adjust::V1p5,
            1 => Adjust::V1p8,
            2 => Adjust::V2p048,
            3 => Adjust::V2p5,
            _ => unreachable!(),
        }
    }
    #[doc = "Output 1.5 V"]
    #[inline(always)]
    pub fn is_v_1p5(&self) -> bool {
        *self == Adjust::V1p5
    }
    #[doc = "Output 1.8 V"]
    #[inline(always)]
    pub fn is_v_1p8(&self) -> bool {
        *self == Adjust::V1p8
    }
    #[doc = "Output 2.048 V"]
    #[inline(always)]
    pub fn is_v_2p048(&self) -> bool {
        *self == Adjust::V2p048
    }
    #[doc = "Output 2.5 V"]
    #[inline(always)]
    pub fn is_v_2p5(&self) -> bool {
        *self == Adjust::V2p5
    }
}
#[doc = "Field `ADJUST` writer - Adjust output voltage of internal reference-voltage source"]
pub type AdjustW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adjust, crate::Safe>;
impl<'a, REG> AdjustW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output 1.5 V"]
    #[inline(always)]
    pub fn v_1p5(self) -> &'a mut crate::W<REG> {
        self.variant(Adjust::V1p5)
    }
    #[doc = "Output 1.8 V"]
    #[inline(always)]
    pub fn v_1p8(self) -> &'a mut crate::W<REG> {
        self.variant(Adjust::V1p8)
    }
    #[doc = "Output 2.048 V"]
    #[inline(always)]
    pub fn v_2p048(self) -> &'a mut crate::W<REG> {
        self.variant(Adjust::V2p048)
    }
    #[doc = "Output 2.5 V"]
    #[inline(always)]
    pub fn v_2p5(self) -> &'a mut crate::W<REG> {
        self.variant(Adjust::V2p5)
    }
}
impl R {
    #[doc = "Bits 0:11 - Tune bandgap voltage"]
    #[inline(always)]
    pub fn tune(&self) -> TuneR {
        TuneR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Turn-on internal reference-voltage source"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Adjust output voltage of internal reference-voltage source"]
    #[inline(always)]
    pub fn adjust(&self) -> AdjustR {
        AdjustR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Tune bandgap voltage"]
    #[inline(always)]
    pub fn tune(&mut self) -> TuneW<'_, BgctrlSpec> {
        TuneW::new(self, 0)
    }
    #[doc = "Bit 16 - Turn-on internal reference-voltage source"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, BgctrlSpec> {
        EnW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Adjust output voltage of internal reference-voltage source"]
    #[inline(always)]
    pub fn adjust(&mut self) -> AdjustW<'_, BgctrlSpec> {
        AdjustW::new(self, 17)
    }
}
#[doc = "Internal reference-voltage source (adjustable BandGap) register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgctrlSpec;
impl crate::RegisterSpec for BgctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgctrl::R`](R) reader structure"]
impl crate::Readable for BgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bgctrl::W`](W) writer structure"]
impl crate::Writable for BgctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BGCTRL to value 0x0800"]
impl crate::Resettable for BgctrlSpec {
    const RESET_VALUE: u32 = 0x0800;
}
