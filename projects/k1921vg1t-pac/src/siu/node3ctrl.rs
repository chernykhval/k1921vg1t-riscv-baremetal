#[doc = "Register `NODE3CTRL` reader"]
pub type R = crate::R<Node3ctrlSpec>;
#[doc = "Register `NODE3CTRL` writer"]
pub type W = crate::W<Node3ctrlSpec>;
#[doc = "Field `FAULT_STATUS` reader - Node fault status"]
pub type FaultStatusR = crate::BitReader;
#[doc = "Field `FAULT_STATUS` writer - Node fault status"]
pub type FaultStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCNT` writer - Clear error counter"]
pub type ClrcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP_STATUS` reader - Node comparator status"]
pub type CmpStatusR = crate::FieldReader<u16>;
#[doc = "Field `CMP_STATUS` writer - Node comparator status"]
pub type CmpStatusW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
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
    #[doc = "Bits 2:10 - Node comparator status"]
    #[inline(always)]
    pub fn cmp_status(&self) -> CmpStatusR {
        CmpStatusR::new(((self.bits >> 2) & 0x01ff) as u16)
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
    pub fn fault_status(&mut self) -> FaultStatusW<'_, Node3ctrlSpec> {
        FaultStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear error counter"]
    #[inline(always)]
    pub fn clrcnt(&mut self) -> ClrcntW<'_, Node3ctrlSpec> {
        ClrcntW::new(self, 1)
    }
    #[doc = "Bits 2:10 - Node comparator status"]
    #[inline(always)]
    pub fn cmp_status(&mut self) -> CmpStatusW<'_, Node3ctrlSpec> {
        CmpStatusW::new(self, 2)
    }
    #[doc = "Bits 20:31 - Node maximum errors selector"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, Node3ctrlSpec> {
        MaskW::new(self, 20)
    }
}
#[doc = "Node 3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`node3ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`node3ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Node3ctrlSpec;
impl crate::RegisterSpec for Node3ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`node3ctrl::R`](R) reader structure"]
impl crate::Readable for Node3ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`node3ctrl::W`](W) writer structure"]
impl crate::Writable for Node3ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NODE3CTRL to value 0xfff0_0000"]
impl crate::Resettable for Node3ctrlSpec {
    const RESET_VALUE: u32 = 0xfff0_0000;
}
