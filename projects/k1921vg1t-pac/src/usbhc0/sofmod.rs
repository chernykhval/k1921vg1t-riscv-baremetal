#[doc = "Register `SOFMOD` reader"]
pub type R = crate::R<SofmodSpec>;
#[doc = "Register `SOFMOD` writer"]
pub type W = crate::W<SofmodSpec>;
#[doc = "Field `SOFVAL` reader - SOF Timing Value"]
pub type SofvalR = crate::FieldReader;
#[doc = "Field `SOFVAL` writer - SOF Timing Value"]
pub type SofvalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - SOF Timing Value"]
    #[inline(always)]
    pub fn sofval(&self) -> SofvalR {
        SofvalR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SOF Timing Value"]
    #[inline(always)]
    pub fn sofval(&mut self) -> SofvalW<'_, SofmodSpec> {
        SofvalW::new(self, 0)
    }
}
#[doc = "USBHC UHC Start of Frame Modify register\n\nYou can [`read`](crate::Reg::read) this register and get [`sofmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SofmodSpec;
impl crate::RegisterSpec for SofmodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sofmod::R`](R) reader structure"]
impl crate::Readable for SofmodSpec {}
#[doc = "`write(|w| ..)` method takes [`sofmod::W`](W) writer structure"]
impl crate::Writable for SofmodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOFMOD to value 0x40"]
impl crate::Resettable for SofmodSpec {
    const RESET_VALUE: u32 = 0x40;
}
