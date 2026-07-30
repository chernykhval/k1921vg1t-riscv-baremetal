#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `MPSEL` reader - Field task position after waiting message bit transmit / receive messages"]
pub type MpselR = crate::FieldReader;
#[doc = "Field `MPSEL` writer - Field task position after waiting message bit transmit / receive messages"]
pub type MpselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 12:15 - Field task position after waiting message bit transmit / receive messages"]
    #[inline(always)]
    pub fn mpsel(&self) -> MpselR {
        MpselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Field task position after waiting message bit transmit / receive messages"]
    #[inline(always)]
    pub fn mpsel(&mut self) -> MpselW<'_, McrSpec> {
        MpselW::new(self, 12)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {}
