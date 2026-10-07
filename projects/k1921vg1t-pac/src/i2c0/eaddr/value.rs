#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `ADDR` reader - Additional adress"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Additional adress"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SAEN` reader - Enable bit address recognition"]
pub type SaenR = crate::BitReader;
#[doc = "Field `SAEN` writer - Enable bit address recognition"]
pub type SaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Additional adress"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Enable bit address recognition"]
    #[inline(always)]
    pub fn saen(&self) -> SaenR {
        SaenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Additional adress"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, ValueSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 15 - Enable bit address recognition"]
    #[inline(always)]
    pub fn saen(&mut self) -> SaenW<'_, ValueSpec> {
        SaenW::new(self, 15)
    }
}
#[doc = "Extended address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
