#[doc = "Register `FWM` reader"]
pub type R = crate::R<FwmSpec>;
#[doc = "Register `FWM` writer"]
pub type W = crate::W<FwmSpec>;
#[doc = "Field `TLEC` reader - Tx FIFO Low Element Count limit ( Low Watermark )"]
pub type TlecR = crate::FieldReader;
#[doc = "Field `TLEC` writer - Tx FIFO Low Element Count limit ( Low Watermark )"]
pub type TlecW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RHEC` reader - Rx FIFO High Element Count limit ( High Watermark )"]
pub type RhecR = crate::FieldReader;
#[doc = "Field `RHEC` writer - Rx FIFO High Element Count limit ( High Watermark )"]
pub type RhecW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Tx FIFO Low Element Count limit ( Low Watermark )"]
    #[inline(always)]
    pub fn tlec(&self) -> TlecR {
        TlecR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Rx FIFO High Element Count limit ( High Watermark )"]
    #[inline(always)]
    pub fn rhec(&self) -> RhecR {
        RhecR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Tx FIFO Low Element Count limit ( Low Watermark )"]
    #[inline(always)]
    pub fn tlec(&mut self) -> TlecW<'_, FwmSpec> {
        TlecW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Rx FIFO High Element Count limit ( High Watermark )"]
    #[inline(always)]
    pub fn rhec(&mut self) -> RhecW<'_, FwmSpec> {
        RhecW::new(self, 8)
    }
}
#[doc = "Fifo's WaterMarks Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fwm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwmSpec;
impl crate::RegisterSpec for FwmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwm::R`](R) reader structure"]
impl crate::Readable for FwmSpec {}
#[doc = "`write(|w| ..)` method takes [`fwm::W`](W) writer structure"]
impl crate::Writable for FwmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FWM to value 0x0e02"]
impl crate::Resettable for FwmSpec {
    const RESET_VALUE: u32 = 0x0e02;
}
