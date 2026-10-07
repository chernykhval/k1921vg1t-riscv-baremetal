#[doc = "Register `DMACTRL` reader"]
pub type R = crate::R<DmactrlSpec>;
#[doc = "Register `DMACTRL` writer"]
pub type W = crate::W<DmactrlSpec>;
#[doc = "Field `DA` reader - Descriptors available"]
pub type DaR = crate::BitReader;
#[doc = "Field `DA` writer - Descriptors available"]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Interrupt enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AI` reader - AHB error interrupt"]
pub type AiR = crate::BitReader;
#[doc = "Field `AI` writer - AHB error interrupt"]
pub type AiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD` reader - Abort DMA"]
pub type AdR = crate::BitReader;
#[doc = "Field `AD` writer - Abort DMA"]
pub type AdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - AHB error"]
pub type AeR = crate::BitReader;
#[doc = "Field `AE` writer - AHB error"]
pub type AeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Descriptors available"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB error interrupt"]
    #[inline(always)]
    pub fn ai(&self) -> AiR {
        AiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Abort DMA"]
    #[inline(always)]
    pub fn ad(&self) -> AdR {
        AdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - AHB error"]
    #[inline(always)]
    pub fn ae(&self) -> AeR {
        AeR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptors available"]
    #[inline(always)]
    pub fn da(&mut self) -> DaW<'_, DmactrlSpec> {
        DaW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, DmactrlSpec> {
        IeW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB error interrupt"]
    #[inline(always)]
    pub fn ai(&mut self) -> AiW<'_, DmactrlSpec> {
        AiW::new(self, 2)
    }
    #[doc = "Bit 3 - Abort DMA"]
    #[inline(always)]
    pub fn ad(&mut self) -> AdW<'_, DmactrlSpec> {
        AdW::new(self, 3)
    }
    #[doc = "Bit 10 - AHB error"]
    #[inline(always)]
    pub fn ae(&mut self) -> AeW<'_, DmactrlSpec> {
        AeW::new(self, 10)
    }
}
#[doc = "OUT DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactrlSpec;
impl crate::RegisterSpec for DmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl::R`](R) reader structure"]
impl crate::Readable for DmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure"]
impl crate::Writable for DmactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTRL to value 0"]
impl crate::Resettable for DmactrlSpec {}
