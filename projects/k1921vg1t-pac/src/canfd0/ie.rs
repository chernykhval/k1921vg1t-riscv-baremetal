#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "Field `EBOVRT` reader - Enable CAN Timer Overrun Interrupt"]
pub type EbovrtR = crate::BitReader;
#[doc = "Field `EBOVRT` writer - Enable CAN Timer Overrun Interrupt"]
pub type EbovrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENERG` reader - Enable Error Interrupt"]
pub type EnergR = crate::BitReader;
#[doc = "Field `ENERG` writer - Enable Error Interrupt"]
pub type EnergW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBX` reader - "]
pub type EnbxR = crate::BitReader;
#[doc = "Field `ENBX` writer - "]
pub type EnbxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENERR` reader - Enable MOb Error Interrupts"]
pub type EnerrR = crate::BitReader;
#[doc = "Field `ENERR` writer - Enable MOb Error Interrupts"]
pub type EnerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTX` reader - Transmit Interrupt Enable"]
pub type EntxR = crate::BitReader;
#[doc = "Field `ENTX` writer - Transmit Interrupt Enable"]
pub type EntxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENRX` reader - Receive Interrupt Enable"]
pub type EnrxR = crate::BitReader;
#[doc = "Field `ENRX` writer - Receive Interrupt Enable"]
pub type EnrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENBOFF` reader - Enable Bus Disable Interrupt"]
pub type EnboffR = crate::BitReader;
#[doc = "Field `ENBOFF` writer - Enable Bus Disable Interrupt"]
pub type EnboffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENIT` reader - Enable All Interrupts"]
pub type EnitR = crate::BitReader;
#[doc = "Field `ENIT` writer - Enable All Interrupts"]
pub type EnitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNCORR_ERR` reader - Enable interrupt when double error detected"]
pub type UncorrErrR = crate::BitReader;
#[doc = "Field `UNCORR_ERR` writer - Enable interrupt when double error detected"]
pub type UncorrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_MEM` reader - Enable interrupt when single error detected"]
pub type ErrMemR = crate::BitReader;
#[doc = "Field `ERR_MEM` writer - Enable interrupt when single error detected"]
pub type ErrMemW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable CAN Timer Overrun Interrupt"]
    #[inline(always)]
    pub fn ebovrt(&self) -> EbovrtR {
        EbovrtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt"]
    #[inline(always)]
    pub fn energ(&self) -> EnergR {
        EnergR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn enbx(&self) -> EnbxR {
        EnbxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable MOb Error Interrupts"]
    #[inline(always)]
    pub fn enerr(&self) -> EnerrR {
        EnerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn entx(&self) -> EntxR {
        EntxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn enrx(&self) -> EnrxR {
        EnrxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Bus Disable Interrupt"]
    #[inline(always)]
    pub fn enboff(&self) -> EnboffR {
        EnboffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable All Interrupts"]
    #[inline(always)]
    pub fn enit(&self) -> EnitR {
        EnitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable interrupt when double error detected"]
    #[inline(always)]
    pub fn uncorr_err(&self) -> UncorrErrR {
        UncorrErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable interrupt when single error detected"]
    #[inline(always)]
    pub fn err_mem(&self) -> ErrMemR {
        ErrMemR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CAN Timer Overrun Interrupt"]
    #[inline(always)]
    pub fn ebovrt(&mut self) -> EbovrtW<'_, IeSpec> {
        EbovrtW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt"]
    #[inline(always)]
    pub fn energ(&mut self) -> EnergW<'_, IeSpec> {
        EnergW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn enbx(&mut self) -> EnbxW<'_, IeSpec> {
        EnbxW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable MOb Error Interrupts"]
    #[inline(always)]
    pub fn enerr(&mut self) -> EnerrW<'_, IeSpec> {
        EnerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn entx(&mut self) -> EntxW<'_, IeSpec> {
        EntxW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn enrx(&mut self) -> EnrxW<'_, IeSpec> {
        EnrxW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Bus Disable Interrupt"]
    #[inline(always)]
    pub fn enboff(&mut self) -> EnboffW<'_, IeSpec> {
        EnboffW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable All Interrupts"]
    #[inline(always)]
    pub fn enit(&mut self) -> EnitW<'_, IeSpec> {
        EnitW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable interrupt when double error detected"]
    #[inline(always)]
    pub fn uncorr_err(&mut self) -> UncorrErrW<'_, IeSpec> {
        UncorrErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable interrupt when single error detected"]
    #[inline(always)]
    pub fn err_mem(&mut self) -> ErrMemW<'_, IeSpec> {
        ErrMemW::new(self, 9)
    }
}
#[doc = "CAN General Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IeSpec {}
