#[doc = "Register `BIT_PATTERN` reader"]
pub type R = crate::R<BitPatternSpec>;
#[doc = "Register `BIT_PATTERN` writer"]
pub type W = crate::W<BitPatternSpec>;
#[doc = "Field `P2M5` reader - Pattern for generating 2.5Mhz clock"]
pub type P2m5R = crate::FieldReader;
#[doc = "Field `P2M5` writer - Pattern for generating 2.5Mhz clock"]
pub type P2m5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `P25M` reader - Pattern for generating 25Mhz clock"]
pub type P25mR = crate::FieldReader;
#[doc = "Field `P25M` writer - Pattern for generating 25Mhz clock"]
pub type P25mW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `P125M` reader - Pattern for generating 125Mhz clock"]
pub type P125mR = crate::FieldReader;
#[doc = "Field `P125M` writer - Pattern for generating 125Mhz clock"]
pub type P125mW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - Pattern for generating 2.5Mhz clock"]
    #[inline(always)]
    pub fn p2m5(&self) -> P2m5R {
        P2m5R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Pattern for generating 25Mhz clock"]
    #[inline(always)]
    pub fn p25m(&self) -> P25mR {
        P25mR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - Pattern for generating 125Mhz clock"]
    #[inline(always)]
    pub fn p125m(&self) -> P125mR {
        P125mR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pattern for generating 2.5Mhz clock"]
    #[inline(always)]
    pub fn p2m5(&mut self) -> P2m5W<'_, BitPatternSpec> {
        P2m5W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Pattern for generating 25Mhz clock"]
    #[inline(always)]
    pub fn p25m(&mut self) -> P25mW<'_, BitPatternSpec> {
        P25mW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Pattern for generating 125Mhz clock"]
    #[inline(always)]
    pub fn p125m(&mut self) -> P125mW<'_, BitPatternSpec> {
        P125mW::new(self, 16)
    }
}
#[doc = "RGMII bit pattern register\n\nYou can [`read`](crate::Reg::read) this register and get [`bit_pattern::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit_pattern::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BitPatternSpec;
impl crate::RegisterSpec for BitPatternSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bit_pattern::R`](R) reader structure"]
impl crate::Readable for BitPatternSpec {}
#[doc = "`write(|w| ..)` method takes [`bit_pattern::W`](W) writer structure"]
impl crate::Writable for BitPatternSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIT_PATTERN to value 0x0002_3838"]
impl crate::Resettable for BitPatternSpec {
    const RESET_VALUE: u32 = 0x0002_3838;
}
