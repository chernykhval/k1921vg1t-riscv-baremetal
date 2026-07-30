#[doc = "Register `LATENCY` reader"]
pub type R = crate::R<LatencySpec>;
#[doc = "Register `LATENCY` writer"]
pub type W = crate::W<LatencySpec>;
#[doc = "Field `WR_LATENCY` reader - SRAM Write Latency"]
pub type WrLatencyR = crate::FieldReader;
#[doc = "Field `WR_LATENCY` writer - SRAM Write Latency"]
pub type WrLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RD_LATENCY` reader - SRAM Read Latency"]
pub type RdLatencyR = crate::FieldReader;
#[doc = "Field `RD_LATENCY` writer - SRAM Read Latency"]
pub type RdLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SRAM Write Latency"]
    #[inline(always)]
    pub fn wr_latency(&self) -> WrLatencyR {
        WrLatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 7:9 - SRAM Read Latency"]
    #[inline(always)]
    pub fn rd_latency(&self) -> RdLatencyR {
        RdLatencyR::new(((self.bits >> 7) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SRAM Write Latency"]
    #[inline(always)]
    pub fn wr_latency(&mut self) -> WrLatencyW<'_, LatencySpec> {
        WrLatencyW::new(self, 0)
    }
    #[doc = "Bits 7:9 - SRAM Read Latency"]
    #[inline(always)]
    pub fn rd_latency(&mut self) -> RdLatencyW<'_, LatencySpec> {
        RdLatencyW::new(self, 7)
    }
}
#[doc = "SRAM Read and Write Latency register\n\nYou can [`read`](crate::Reg::read) this register and get [`latency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`latency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LatencySpec;
impl crate::RegisterSpec for LatencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`latency::R`](R) reader structure"]
impl crate::Readable for LatencySpec {}
#[doc = "`write(|w| ..)` method takes [`latency::W`](W) writer structure"]
impl crate::Writable for LatencySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LATENCY to value 0"]
impl crate::Resettable for LatencySpec {}
