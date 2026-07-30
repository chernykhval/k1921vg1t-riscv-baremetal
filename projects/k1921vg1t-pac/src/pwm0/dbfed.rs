#[doc = "Register `DBFED` reader"]
pub type R = crate::R<DbfedSpec>;
#[doc = "Register `DBFED` writer"]
pub type W = crate::W<DbfedSpec>;
#[doc = "Field `DEL` reader - Falling edge delay count"]
pub type DelR = crate::FieldReader<u16>;
#[doc = "Field `DEL` writer - Falling edge delay count"]
pub type DelW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge delay count"]
    #[inline(always)]
    pub fn del(&self) -> DelR {
        DelR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge delay count"]
    #[inline(always)]
    pub fn del(&mut self) -> DelW<'_, DbfedSpec> {
        DelW::new(self, 0)
    }
}
#[doc = "Dead-Band Generator Falling Edge Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbfed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbfed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbfedSpec;
impl crate::RegisterSpec for DbfedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbfed::R`](R) reader structure"]
impl crate::Readable for DbfedSpec {}
#[doc = "`write(|w| ..)` method takes [`dbfed::W`](W) writer structure"]
impl crate::Writable for DbfedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBFED to value 0"]
impl crate::Resettable for DbfedSpec {}
