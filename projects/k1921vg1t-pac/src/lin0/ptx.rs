#[doc = "Register `PTX` reader"]
pub type R = crate::R<PtxSpec>;
#[doc = "Register `PTX` writer"]
pub type W = crate::W<PtxSpec>;
#[doc = "Field `PIDTX` reader - PID is sent in the next data frame"]
pub type PidtxR = crate::FieldReader;
#[doc = "Field `PIDTX` writer - PID is sent in the next data frame"]
pub type PidtxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PID is sent in the next data frame"]
    #[inline(always)]
    pub fn pidtx(&self) -> PidtxR {
        PidtxR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PID is sent in the next data frame"]
    #[inline(always)]
    pub fn pidtx(&mut self) -> PidtxW<'_, PtxSpec> {
        PidtxW::new(self, 0)
    }
}
#[doc = "Register of the PID being sent\n\nYou can [`read`](crate::Reg::read) this register and get [`ptx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtxSpec;
impl crate::RegisterSpec for PtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptx::R`](R) reader structure"]
impl crate::Readable for PtxSpec {}
#[doc = "`write(|w| ..)` method takes [`ptx::W`](W) writer structure"]
impl crate::Writable for PtxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTX to value 0"]
impl crate::Resettable for PtxSpec {}
