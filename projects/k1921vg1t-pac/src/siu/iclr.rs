#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `BUSTIMOUT` writer - Clear bus timeout interrupt"]
pub type BustimoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE0FAULT` writer - Clear node 0 fault interrupt"]
pub type Node0faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE1FAULT` writer - Clear node 1 fault interrupt"]
pub type Node1faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE2FAULT` writer - Clear node 2 fault interrupt"]
pub type Node2faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE3FAULT` writer - Clear node 3 fault interrupt"]
pub type Node3faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE4FAULT` writer - Clear node 4 fault interrupt"]
pub type Node4faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVD11` writer - Clear Minimal Voltage Detect on DVDD supply line interrupt"]
pub type Mvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVD11` writer - Clear Low Voltage Detect on DVDD supply line interrupt"]
pub type Lvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVD11` writer - Clear High Voltage Detect on DVDD supply line interrupt"]
pub type Hvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVD33` writer - Clear Minimal Voltage Detect on AVDD supply line interrupt"]
pub type Mvd33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVD33` writer - Clear Low Voltage Detect on AVDD supply line interrupt"]
pub type Lvd33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVD33` writer - Clear High Voltage Detect on AVDD supply line interrupt"]
pub type Hvd33W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear bus timeout interrupt"]
    #[inline(always)]
    pub fn bustimout(&mut self) -> BustimoutW<'_, IclrSpec> {
        BustimoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear node 0 fault interrupt"]
    #[inline(always)]
    pub fn node0fault(&mut self) -> Node0faultW<'_, IclrSpec> {
        Node0faultW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear node 1 fault interrupt"]
    #[inline(always)]
    pub fn node1fault(&mut self) -> Node1faultW<'_, IclrSpec> {
        Node1faultW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear node 2 fault interrupt"]
    #[inline(always)]
    pub fn node2fault(&mut self) -> Node2faultW<'_, IclrSpec> {
        Node2faultW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear node 3 fault interrupt"]
    #[inline(always)]
    pub fn node3fault(&mut self) -> Node3faultW<'_, IclrSpec> {
        Node3faultW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear node 4 fault interrupt"]
    #[inline(always)]
    pub fn node4fault(&mut self) -> Node4faultW<'_, IclrSpec> {
        Node4faultW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Minimal Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn mvd11(&mut self) -> Mvd11W<'_, IclrSpec> {
        Mvd11W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Low Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn lvd11(&mut self) -> Lvd11W<'_, IclrSpec> {
        Lvd11W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear High Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn hvd11(&mut self) -> Hvd11W<'_, IclrSpec> {
        Hvd11W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Minimal Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn mvd33(&mut self) -> Mvd33W<'_, IclrSpec> {
        Mvd33W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Low Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn lvd33(&mut self) -> Lvd33W<'_, IclrSpec> {
        Lvd33W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear High Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn hvd33(&mut self) -> Hvd33W<'_, IclrSpec> {
        Hvd33W::new(self, 11)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {}
