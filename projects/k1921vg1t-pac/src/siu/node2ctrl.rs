#[doc = "Register `NODE2CTRL` reader"]
pub type R = crate::R<Node2ctrlSpec>;
#[doc = "Register `NODE2CTRL` writer"]
pub type W = crate::W<Node2ctrlSpec>;
#[doc = "Field `FAULT_STATUS` reader - Node fault status"]
pub type FaultStatusR = crate::BitReader;
#[doc = "Field `FAULT_STATUS` writer - Node fault status"]
pub type FaultStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCNT` writer - Clear error counter"]
pub type ClrcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP_STATUS` reader - Node comparator status"]
pub type CmpStatusR = crate::FieldReader<u16>;
#[doc = "Field `CMP_STATUS` writer - Node comparator status"]
pub type CmpStatusW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `MASK` reader - Node maximum errors selector"]
pub type MaskR = crate::FieldReader<u16>;
#[doc = "Field `MASK` writer - Node maximum errors selector"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Node fault status"]
    #[inline(always)]
    pub fn fault_status(&self) -> FaultStatusR {
        FaultStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:14 - Node comparator status"]
    #[inline(always)]
    pub fn cmp_status(&self) -> CmpStatusR {
        CmpStatusR::new(((self.bits >> 2) & 0x1fff) as u16)
    }
    #[doc = "Bits 20:31 - Node maximum errors selector"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Node fault status"]
    #[inline(always)]
    pub fn fault_status(&mut self) -> FaultStatusW<'_, Node2ctrlSpec> {
        FaultStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear error counter"]
    #[inline(always)]
    pub fn clrcnt(&mut self) -> ClrcntW<'_, Node2ctrlSpec> {
        ClrcntW::new(self, 1)
    }
    #[doc = "Bits 2:14 - Node comparator status"]
    #[inline(always)]
    pub fn cmp_status(&mut self) -> CmpStatusW<'_, Node2ctrlSpec> {
        CmpStatusW::new(self, 2)
    }
    #[doc = "Bits 20:31 - Node maximum errors selector"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, Node2ctrlSpec> {
        MaskW::new(self, 20)
    }
}
#[doc = "Node 2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`node2ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`node2ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Node2ctrlSpec;
impl crate::RegisterSpec for Node2ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`node2ctrl::R`](R) reader structure"]
impl crate::Readable for Node2ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`node2ctrl::W`](W) writer structure"]
impl crate::Writable for Node2ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NODE2CTRL to value 0xfff0_0000"]
impl crate::Resettable for Node2ctrlSpec {
    const RESET_VALUE: u32 = 0xfff0_0000;
}
