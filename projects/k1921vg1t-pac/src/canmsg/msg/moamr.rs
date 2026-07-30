#[doc = "Register `MOAMR` reader"]
pub type R = crate::R<MoamrSpec>;
#[doc = "Register `MOAMR` writer"]
pub type W = crate::W<MoamrSpec>;
#[doc = "Field `AM` reader - Mask ID"]
pub type AmR = crate::FieldReader<u32>;
#[doc = "Field `AM` writer - Mask ID"]
pub type AmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `MIDE` reader - Mask bit IDE message"]
pub type MideR = crate::BitReader;
#[doc = "Field `MIDE` writer - Mask bit IDE message"]
pub type MideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - Mask ID"]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Mask bit IDE message"]
    #[inline(always)]
    pub fn mide(&self) -> MideR {
        MideR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Mask ID"]
    #[inline(always)]
    pub fn am(&mut self) -> AmW<'_, MoamrSpec> {
        AmW::new(self, 0)
    }
    #[doc = "Bit 29 - Mask bit IDE message"]
    #[inline(always)]
    pub fn mide(&mut self) -> MideW<'_, MoamrSpec> {
        MideW::new(self, 29)
    }
}
#[doc = "Mask register message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`moamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoamrSpec;
impl crate::RegisterSpec for MoamrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moamr::R`](R) reader structure"]
impl crate::Readable for MoamrSpec {}
#[doc = "`write(|w| ..)` method takes [`moamr::W`](W) writer structure"]
impl crate::Writable for MoamrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOAMR to value 0"]
impl crate::Resettable for MoamrSpec {}
