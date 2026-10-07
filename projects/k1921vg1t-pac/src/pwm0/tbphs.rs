#[doc = "Register `TBPHS` reader"]
pub type R = crate::R<TbphsSpec>;
#[doc = "Register `TBPHS` writer"]
pub type W = crate::W<TbphsSpec>;
#[doc = "Field `TBPHSHR` reader - Time-base counter phase in High-Resolution mode"]
pub type TbphshrR = crate::FieldReader;
#[doc = "Field `TBPHSHR` writer - Time-base counter phase in High-Resolution mode"]
pub type TbphshrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TBPHS` reader - Time-base counter phase"]
pub type TbphsR = crate::FieldReader<u16>;
#[doc = "Field `TBPHS` writer - Time-base counter phase"]
pub type TbphsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:15 - Time-base counter phase in High-Resolution mode"]
    #[inline(always)]
    pub fn tbphshr(&self) -> TbphshrR {
        TbphshrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Time-base counter phase"]
    #[inline(always)]
    pub fn tbphs(&self) -> TbphsR {
        TbphsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - Time-base counter phase in High-Resolution mode"]
    #[inline(always)]
    pub fn tbphshr(&mut self) -> TbphshrW<'_, TbphsSpec> {
        TbphshrW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Time-base counter phase"]
    #[inline(always)]
    pub fn tbphs(&mut self) -> TbphsW<'_, TbphsSpec> {
        TbphsW::new(self, 16)
    }
}
#[doc = "Time-Base Phase Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbphs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbphs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbphsSpec;
impl crate::RegisterSpec for TbphsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbphs::R`](R) reader structure"]
impl crate::Readable for TbphsSpec {}
#[doc = "`write(|w| ..)` method takes [`tbphs::W`](W) writer structure"]
impl crate::Writable for TbphsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBPHS to value 0"]
impl crate::Resettable for TbphsSpec {}
