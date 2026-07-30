#[doc = "Register `SSRAMMCR` reader"]
pub type R = crate::R<SsrammcrSpec>;
#[doc = "Register `SSRAMMCR` writer"]
pub type W = crate::W<SsrammcrSpec>;
#[doc = "Field `RMEN` reader - Enable to use Read-Write Margin"]
pub type RmenR = crate::BitReader;
#[doc = "Field `RMEN` writer - Enable to use Read-Write Margin"]
pub type RmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RM` reader - Read-Write Margin value"]
pub type RmR = crate::FieldReader;
#[doc = "Field `RM` writer - Read-Write Margin value"]
pub type RmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable to use Read-Write Margin"]
    #[inline(always)]
    pub fn rmen(&self) -> RmenR {
        RmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Read-Write Margin value"]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable to use Read-Write Margin"]
    #[inline(always)]
    pub fn rmen(&mut self) -> RmenW<'_, SsrammcrSpec> {
        RmenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Read-Write Margin value"]
    #[inline(always)]
    pub fn rm(&mut self) -> RmW<'_, SsrammcrSpec> {
        RmW::new(self, 1)
    }
}
#[doc = "System SRAM margin control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrammcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrammcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrammcrSpec;
impl crate::RegisterSpec for SsrammcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssrammcr::R`](R) reader structure"]
impl crate::Readable for SsrammcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssrammcr::W`](W) writer structure"]
impl crate::Writable for SsrammcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSRAMMCR to value 0"]
impl crate::Resettable for SsrammcrSpec {}
