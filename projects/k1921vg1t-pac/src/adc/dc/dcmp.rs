#[doc = "Register `DCMP` reader"]
pub type R = crate::R<DcmpSpec>;
#[doc = "Register `DCMP` writer"]
pub type W = crate::W<DcmpSpec>;
#[doc = "Field `CMPL` reader - Low threshold compare value"]
pub type CmplR = crate::FieldReader<u16>;
#[doc = "Field `CMPL` writer - Low threshold compare value"]
pub type CmplW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CMPH` reader - High threshold compare value"]
pub type CmphR = crate::FieldReader<u16>;
#[doc = "Field `CMPH` writer - High threshold compare value"]
pub type CmphW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Low threshold compare value"]
    #[inline(always)]
    pub fn cmpl(&self) -> CmplR {
        CmplR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - High threshold compare value"]
    #[inline(always)]
    pub fn cmph(&self) -> CmphR {
        CmphR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low threshold compare value"]
    #[inline(always)]
    pub fn cmpl(&mut self) -> CmplW<'_, DcmpSpec> {
        CmplW::new(self, 0)
    }
    #[doc = "Bits 16:27 - High threshold compare value"]
    #[inline(always)]
    pub fn cmph(&mut self) -> CmphW<'_, DcmpSpec> {
        CmphW::new(self, 16)
    }
}
#[doc = "Digital comparator range register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcmpSpec;
impl crate::RegisterSpec for DcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmp::R`](R) reader structure"]
impl crate::Readable for DcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`dcmp::W`](W) writer structure"]
impl crate::Writable for DcmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCMP to value 0"]
impl crate::Resettable for DcmpSpec {}
