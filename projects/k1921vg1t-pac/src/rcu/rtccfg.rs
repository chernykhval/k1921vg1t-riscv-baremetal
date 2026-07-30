#[doc = "Register `RTCCFG` reader"]
pub type R = crate::R<RtccfgSpec>;
#[doc = "Register `RTCCFG` writer"]
pub type W = crate::W<RtccfgSpec>;
#[doc = "Field `RST` reader - Reset active High"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Reset active High"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVEN` reader - Enable divider"]
pub type DivenR = crate::BitReader;
#[doc = "Field `DIVEN` writer - Enable divider"]
pub type DivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVN` reader - Divider coefficient"]
pub type DivnR = crate::FieldReader;
#[doc = "Field `DIVN` writer - Divider coefficient"]
pub type DivnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 4 - Reset active High"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable divider"]
    #[inline(always)]
    pub fn diven(&self) -> DivenR {
        DivenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&self) -> DivnR {
        DivnR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Reset active High"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<'_, RtccfgSpec> {
        RstW::new(self, 4)
    }
    #[doc = "Bit 12 - Enable divider"]
    #[inline(always)]
    pub fn diven(&mut self) -> DivenW<'_, RtccfgSpec> {
        DivenW::new(self, 12)
    }
    #[doc = "Bits 16:21 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&mut self) -> DivnW<'_, RtccfgSpec> {
        DivnW::new(self, 16)
    }
}
#[doc = "RTC APB clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtccfgSpec;
impl crate::RegisterSpec for RtccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccfg::R`](R) reader structure"]
impl crate::Readable for RtccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rtccfg::W`](W) writer structure"]
impl crate::Writable for RtccfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCFG to value 0x0003_1000"]
impl crate::Resettable for RtccfgSpec {
    const RESET_VALUE: u32 = 0x0003_1000;
}
