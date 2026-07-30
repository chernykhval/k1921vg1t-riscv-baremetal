#[doc = "Register `CORE1_CLKEN` writer"]
pub type W = crate::W<Core1ClkenSpec>;
#[doc = "Field `CLKEN` writer - Clock enable for Core 1"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODE` writer - Write key 0xACCE55 to change CLKEN bit value when in DUALCORE mode"]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bit 0 - Clock enable for Core 1"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, Core1ClkenSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write key 0xACCE55 to change CLKEN bit value when in DUALCORE mode"]
    #[inline(always)]
    pub fn code(&mut self) -> CodeW<'_, Core1ClkenSpec> {
        CodeW::new(self, 8)
    }
}
#[doc = "Core 1 clock enable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_clken::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1ClkenSpec;
impl crate::RegisterSpec for Core1ClkenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core1_clken::W`](W) writer structure"]
impl crate::Writable for Core1ClkenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE1_CLKEN to value 0x01"]
impl crate::Resettable for Core1ClkenSpec {
    const RESET_VALUE: u32 = 0x01;
}
