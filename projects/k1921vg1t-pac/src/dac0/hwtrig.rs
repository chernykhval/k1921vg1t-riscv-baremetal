#[doc = "Register `HWTRIG` reader"]
pub type R = crate::R<HwtrigSpec>;
#[doc = "Register `HWTRIG` writer"]
pub type W = crate::W<HwtrigSpec>;
#[doc = "Field `ENABLE` reader - Start internal trigger generation"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Start internal trigger generation"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVIDER` reader - Frequency divider value"]
pub type DividerR = crate::FieldReader<u16>;
#[doc = "Field `DIVIDER` writer - Frequency divider value"]
pub type DividerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Start internal trigger generation"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - Frequency divider value"]
    #[inline(always)]
    pub fn divider(&self) -> DividerR {
        DividerR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Start internal trigger generation"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, HwtrigSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Frequency divider value"]
    #[inline(always)]
    pub fn divider(&mut self) -> DividerW<'_, HwtrigSpec> {
        DividerW::new(self, 16)
    }
}
#[doc = "Internal Hardware trigger generator control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwtrig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwtrig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwtrigSpec;
impl crate::RegisterSpec for HwtrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwtrig::R`](R) reader structure"]
impl crate::Readable for HwtrigSpec {}
#[doc = "`write(|w| ..)` method takes [`hwtrig::W`](W) writer structure"]
impl crate::Writable for HwtrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HWTRIG to value 0x0010_0000"]
impl crate::Resettable for HwtrigSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
