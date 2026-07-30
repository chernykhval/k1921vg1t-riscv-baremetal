#[doc = "Register `DST_PTR` reader"]
pub type R = crate::R<DstPtrSpec>;
#[doc = "Register `DST_PTR` writer"]
pub type W = crate::W<DstPtrSpec>;
#[doc = "Field `WR_ADDR` reader - Start address of write buffer"]
pub type WrAddrR = crate::FieldReader<u32>;
#[doc = "Field `WR_ADDR` writer - Start address of write buffer"]
pub type WrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of write buffer"]
    #[inline(always)]
    pub fn wr_addr(&self) -> WrAddrR {
        WrAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of write buffer"]
    #[inline(always)]
    pub fn wr_addr(&mut self) -> WrAddrW<'_, DstPtrSpec> {
        WrAddrW::new(self, 0)
    }
}
#[doc = "Command register 1 (destination pointer)\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstPtrSpec;
impl crate::RegisterSpec for DstPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_ptr::R`](R) reader structure"]
impl crate::Readable for DstPtrSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_ptr::W`](W) writer structure"]
impl crate::Writable for DstPtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_PTR to value 0"]
impl crate::Resettable for DstPtrSpec {}
