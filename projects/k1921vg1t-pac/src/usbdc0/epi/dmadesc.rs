#[doc = "Register `DMADESC` reader"]
pub type R = crate::R<DmadescSpec>;
#[doc = "Register `DMADESC` writer"]
pub type W = crate::W<DmadescSpec>;
#[doc = "Field `DESCADDR` reader - Descriptor table address"]
pub type DescaddrR = crate::FieldReader<u32>;
#[doc = "Field `DESCADDR` writer - Descriptor table address"]
pub type DescaddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Descriptor table address"]
    #[inline(always)]
    pub fn descaddr(&self) -> DescaddrR {
        DescaddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Descriptor table address"]
    #[inline(always)]
    pub fn descaddr(&mut self) -> DescaddrW<'_, DmadescSpec> {
        DescaddrW::new(self, 2)
    }
}
#[doc = "IN Descriptor address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmadesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmadesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmadescSpec;
impl crate::RegisterSpec for DmadescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmadesc::R`](R) reader structure"]
impl crate::Readable for DmadescSpec {}
#[doc = "`write(|w| ..)` method takes [`dmadesc::W`](W) writer structure"]
impl crate::Writable for DmadescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMADESC to value 0"]
impl crate::Resettable for DmadescSpec {}
