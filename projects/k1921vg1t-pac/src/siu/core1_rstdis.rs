#[doc = "Register `CORE1_RSTDIS` writer"]
pub type W = crate::W<Core1RstdisSpec>;
#[doc = "Field `RSTDIS` writer - Reset disable for Core 1"]
pub type RstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODE` writer - Write key 0xACCE55 to change RSTDIS bit value when in DUALCORE mode"]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bit 0 - Reset disable for Core 1"]
    #[inline(always)]
    pub fn rstdis(&mut self) -> RstdisW<'_, Core1RstdisSpec> {
        RstdisW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write key 0xACCE55 to change RSTDIS bit value when in DUALCORE mode"]
    #[inline(always)]
    pub fn code(&mut self) -> CodeW<'_, Core1RstdisSpec> {
        CodeW::new(self, 8)
    }
}
#[doc = "Core 1 reset disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_rstdis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1RstdisSpec;
impl crate::RegisterSpec for Core1RstdisSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core1_rstdis::W`](W) writer structure"]
impl crate::Writable for Core1RstdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE1_RSTDIS to value 0x01"]
impl crate::Resettable for Core1RstdisSpec {
    const RESET_VALUE: u32 = 0x01;
}
