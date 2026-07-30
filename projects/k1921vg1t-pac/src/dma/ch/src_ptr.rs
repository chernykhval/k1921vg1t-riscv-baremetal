#[doc = "Register `SRC_PTR` reader"]
pub type R = crate::R<SrcPtrSpec>;
#[doc = "Register `SRC_PTR` writer"]
pub type W = crate::W<SrcPtrSpec>;
#[doc = "Field `RD_ADDR` reader - Start address of read buffer"]
pub type RdAddrR = crate::FieldReader<u32>;
#[doc = "Field `RD_ADDR` writer - Start address of read buffer"]
pub type RdAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of read buffer"]
    #[inline(always)]
    pub fn rd_addr(&self) -> RdAddrR {
        RdAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of read buffer"]
    #[inline(always)]
    pub fn rd_addr(&mut self) -> RdAddrW<'_, SrcPtrSpec> {
        RdAddrW::new(self, 0)
    }
}
#[doc = "Command register 0 (source pointer)\n\nYou can [`read`](crate::Reg::read) this register and get [`src_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcPtrSpec;
impl crate::RegisterSpec for SrcPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_ptr::R`](R) reader structure"]
impl crate::Readable for SrcPtrSpec {}
#[doc = "`write(|w| ..)` method takes [`src_ptr::W`](W) writer structure"]
impl crate::Writable for SrcPtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_PTR to value 0"]
impl crate::Resettable for SrcPtrSpec {}
