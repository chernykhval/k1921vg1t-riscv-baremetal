#[doc = "Register `BL` reader"]
pub type R = crate::R<BlSpec>;
#[doc = "Register `BL` writer"]
pub type W = crate::W<BlSpec>;
#[doc = "Field `BRKLEN` reader - Minimum length of the synchronization break field"]
pub type BrklenR = crate::FieldReader<u16>;
#[doc = "Field `BRKLEN` writer - Minimum length of the synchronization break field"]
pub type BrklenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Minimum length of the synchronization break field"]
    #[inline(always)]
    pub fn brklen(&self) -> BrklenR {
        BrklenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum length of the synchronization break field"]
    #[inline(always)]
    pub fn brklen(&mut self) -> BrklenW<'_, BlSpec> {
        BrklenW::new(self, 0)
    }
}
#[doc = "Register for setting the length of the BREAK field\n\nYou can [`read`](crate::Reg::read) this register and get [`bl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlSpec;
impl crate::RegisterSpec for BlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bl::R`](R) reader structure"]
impl crate::Readable for BlSpec {}
#[doc = "`write(|w| ..)` method takes [`bl::W`](W) writer structure"]
impl crate::Writable for BlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BL to value 0x01e7"]
impl crate::Resettable for BlSpec {
    const RESET_VALUE: u32 = 0x01e7;
}
