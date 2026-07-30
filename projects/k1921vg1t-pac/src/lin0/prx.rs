#[doc = "Register `PRX` reader"]
pub type R = crate::R<PrxSpec>;
#[doc = "Field `PIDRX` reader - Value of the received PID"]
pub type PidrxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Value of the received PID"]
    #[inline(always)]
    pub fn pidrx(&self) -> PidrxR {
        PidrxR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Register of the received PID\n\nYou can [`read`](crate::Reg::read) this register and get [`prx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrxSpec;
impl crate::RegisterSpec for PrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prx::R`](R) reader structure"]
impl crate::Readable for PrxSpec {}
#[doc = "`reset()` method sets PRX to value 0"]
impl crate::Resettable for PrxSpec {}
