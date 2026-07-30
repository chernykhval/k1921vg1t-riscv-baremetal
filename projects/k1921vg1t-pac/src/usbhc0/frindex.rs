#[doc = "Register `FRINDEX` reader"]
pub type R = crate::R<FrindexSpec>;
#[doc = "Register `FRINDEX` writer"]
pub type W = crate::W<FrindexSpec>;
#[doc = "Field `FRAME_IDX` reader - Frame Index"]
pub type FrameIdxR = crate::FieldReader<u16>;
#[doc = "Field `FRAME_IDX` writer - Frame Index"]
pub type FrameIdxW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frame_idx(&self) -> FrameIdxR {
        FrameIdxR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frame_idx(&mut self) -> FrameIdxW<'_, FrindexSpec> {
        FrameIdxW::new(self, 0)
    }
}
#[doc = "USBHC EHC Frame Index register\n\nYou can [`read`](crate::Reg::read) this register and get [`frindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrindexSpec;
impl crate::RegisterSpec for FrindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frindex::R`](R) reader structure"]
impl crate::Readable for FrindexSpec {}
#[doc = "`write(|w| ..)` method takes [`frindex::W`](W) writer structure"]
impl crate::Writable for FrindexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRINDEX to value 0"]
impl crate::Resettable for FrindexSpec {}
