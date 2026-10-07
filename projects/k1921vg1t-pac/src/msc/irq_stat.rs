#[doc = "Register `IRQ_STAT` reader"]
pub type R = crate::R<IrqStatSpec>;
#[doc = "Field `DF` reader - Data Frame flag"]
pub type DfR = crate::BitReader;
#[doc = "Field `CF` reader - Command Frame flag"]
pub type CfR = crate::BitReader;
#[doc = "Field `TF` reader - passive Time Frame flag"]
pub type TfR = crate::BitReader;
#[doc = "Field `RD` reader - Receive upstream Data flag"]
pub type RdR = crate::BitReader;
#[doc = "Field `UTO` reader - Upstream Time-Out flag"]
pub type UtoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Frame flag"]
    #[inline(always)]
    pub fn df(&self) -> DfR {
        DfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Frame flag"]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - passive Time Frame flag"]
    #[inline(always)]
    pub fn tf(&self) -> TfR {
        TfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive upstream Data flag"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Upstream Time-Out flag"]
    #[inline(always)]
    pub fn uto(&self) -> UtoR {
        UtoR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Interrupt requests status register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqStatSpec;
impl crate::RegisterSpec for IrqStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_stat::R`](R) reader structure"]
impl crate::Readable for IrqStatSpec {}
#[doc = "`reset()` method sets IRQ_STAT to value 0"]
impl crate::Resettable for IrqStatSpec {}
