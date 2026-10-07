#[doc = "Register `IRQ_EN` reader"]
pub type R = crate::R<IrqEnSpec>;
#[doc = "Register `IRQ_EN` writer"]
pub type W = crate::W<IrqEnSpec>;
#[doc = "Field `DF` reader - enable Data Frame interrupt"]
pub type DfR = crate::BitReader;
#[doc = "Field `DF` writer - enable Data Frame interrupt"]
pub type DfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CF` reader - enable Command Frame interrupt"]
pub type CfR = crate::BitReader;
#[doc = "Field `CF` writer - enable Command Frame interrupt"]
pub type CfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TF` reader - enable passive Time Frame interrupt"]
pub type TfR = crate::BitReader;
#[doc = "Field `TF` writer - enable passive Time Frame interrupt"]
pub type TfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - enable Receive upstream Data interrupt"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - enable Receive upstream Data interrupt"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTO` reader - enable Upstream Time-Out interrupt"]
pub type UtoR = crate::BitReader;
#[doc = "Field `UTO` writer - enable Upstream Time-Out interrupt"]
pub type UtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFEI` reader - Data Frame Early Interrupt (after first data bit)"]
pub type DfeiR = crate::BitReader;
#[doc = "Field `DFEI` writer - Data Frame Early Interrupt (after first data bit)"]
pub type DfeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRI0` reader - Ignore Received Item 0"]
pub type Iri0R = crate::BitReader;
#[doc = "Field `IRI0` writer - Ignore Received Item 0"]
pub type Iri0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRI1` reader - Ignore Received Item 1"]
pub type Iri1R = crate::BitReader;
#[doc = "Field `IRI1` writer - Ignore Received Item 1"]
pub type Iri1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRI2` reader - Ignore Received Item 2"]
pub type Iri2R = crate::BitReader;
#[doc = "Field `IRI2` writer - Ignore Received Item 2"]
pub type Iri2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRI3` reader - Ignore Received Item 3"]
pub type Iri3R = crate::BitReader;
#[doc = "Field `IRI3` writer - Ignore Received Item 3"]
pub type Iri3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable Data Frame interrupt"]
    #[inline(always)]
    pub fn df(&self) -> DfR {
        DfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable Command Frame interrupt"]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable passive Time Frame interrupt"]
    #[inline(always)]
    pub fn tf(&self) -> TfR {
        TfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable Receive upstream Data interrupt"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable Upstream Time-Out interrupt"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Data Frame Early Interrupt (after first data bit)"]
    #[inline(always)]
    pub fn dfei(&self) -> DfeiR {
        DfeiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Ignore Received Item 0"]
    #[inline(always)]
    pub fn iri0(&self) -> Iri0R {
        Iri0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ignore Received Item 1"]
    #[inline(always)]
    pub fn iri1(&self) -> Iri1R {
        Iri1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ignore Received Item 2"]
    #[inline(always)]
    pub fn iri2(&self) -> Iri2R {
        Iri2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Ignore Received Item 3"]
    #[inline(always)]
    pub fn iri3(&self) -> Iri3R {
        Iri3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable Data Frame interrupt"]
    #[inline(always)]
    pub fn df(&mut self) -> DfW<'_, IrqEnSpec> {
        DfW::new(self, 0)
    }
    #[doc = "Bit 1 - enable Command Frame interrupt"]
    #[inline(always)]
    pub fn cf(&mut self) -> CfW<'_, IrqEnSpec> {
        CfW::new(self, 1)
    }
    #[doc = "Bit 2 - enable passive Time Frame interrupt"]
    #[inline(always)]
    pub fn tf(&mut self) -> TfW<'_, IrqEnSpec> {
        TfW::new(self, 2)
    }
    #[doc = "Bit 3 - enable Receive upstream Data interrupt"]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<'_, IrqEnSpec> {
        RdW::new(self, 3)
    }
    #[doc = "Bit 4 - enable Upstream Time-Out interrupt"]
    #[inline(always)]
    pub fn uto(&mut self) -> UtoW<'_, IrqEnSpec> {
        UtoW::new(self, 4)
    }
    #[doc = "Bit 8 - Data Frame Early Interrupt (after first data bit)"]
    #[inline(always)]
    pub fn dfei(&mut self) -> DfeiW<'_, IrqEnSpec> {
        DfeiW::new(self, 8)
    }
    #[doc = "Bit 16 - Ignore Received Item 0"]
    #[inline(always)]
    pub fn iri0(&mut self) -> Iri0W<'_, IrqEnSpec> {
        Iri0W::new(self, 16)
    }
    #[doc = "Bit 17 - Ignore Received Item 1"]
    #[inline(always)]
    pub fn iri1(&mut self) -> Iri1W<'_, IrqEnSpec> {
        Iri1W::new(self, 17)
    }
    #[doc = "Bit 18 - Ignore Received Item 2"]
    #[inline(always)]
    pub fn iri2(&mut self) -> Iri2W<'_, IrqEnSpec> {
        Iri2W::new(self, 18)
    }
    #[doc = "Bit 19 - Ignore Received Item 3"]
    #[inline(always)]
    pub fn iri3(&mut self) -> Iri3W<'_, IrqEnSpec> {
        Iri3W::new(self, 19)
    }
}
#[doc = "Interrupt requests enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnSpec;
impl crate::RegisterSpec for IrqEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_en::R`](R) reader structure"]
impl crate::Readable for IrqEnSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_en::W`](W) writer structure"]
impl crate::Writable for IrqEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_EN to value 0"]
impl crate::Resettable for IrqEnSpec {}
