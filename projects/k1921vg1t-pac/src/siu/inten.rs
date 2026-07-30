#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `BUSTIMOUT` reader - Enable bus timeout interrupt"]
pub type BustimoutR = crate::BitReader;
#[doc = "Field `BUSTIMOUT` writer - Enable bus timeout interrupt"]
pub type BustimoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE0FAULT` reader - Enable node 0 fault interrupt"]
pub type Node0faultR = crate::BitReader;
#[doc = "Field `NODE0FAULT` writer - Enable node 0 fault interrupt"]
pub type Node0faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE1FAULT` reader - Enable node 1 fault interrupt"]
pub type Node1faultR = crate::BitReader;
#[doc = "Field `NODE1FAULT` writer - Enable node 1 fault interrupt"]
pub type Node1faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE2FAULT` reader - Enable node 2 fault interrupt"]
pub type Node2faultR = crate::BitReader;
#[doc = "Field `NODE2FAULT` writer - Enable node 2 fault interrupt"]
pub type Node2faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE3FAULT` reader - Enable node 3 fault interrupt"]
pub type Node3faultR = crate::BitReader;
#[doc = "Field `NODE3FAULT` writer - Enable node 3 fault interrupt"]
pub type Node3faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE4FAULT` reader - Enable node 4 fault interrupt"]
pub type Node4faultR = crate::BitReader;
#[doc = "Field `NODE4FAULT` writer - Enable node 4 fault interrupt"]
pub type Node4faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVD11` reader - Enable Minimal Voltage Detect on DVDD supply line interrupt"]
pub type Mvd11R = crate::BitReader;
#[doc = "Field `MVD11` writer - Enable Minimal Voltage Detect on DVDD supply line interrupt"]
pub type Mvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVD11` reader - Enable Low Voltage Detect on DVDD supply line interrupt"]
pub type Lvd11R = crate::BitReader;
#[doc = "Field `LVD11` writer - Enable Low Voltage Detect on DVDD supply line interrupt"]
pub type Lvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVD11` reader - Enable High Voltage Detect on DVDD supply line interrupt"]
pub type Hvd11R = crate::BitReader;
#[doc = "Field `HVD11` writer - Enable High Voltage Detect on DVDD supply line interrupt"]
pub type Hvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVD33` reader - Enable Minimal Voltage Detect on AVDD supply line interrupt"]
pub type Mvd33R = crate::BitReader;
#[doc = "Field `MVD33` writer - Enable Minimal Voltage Detect on AVDD supply line interrupt"]
pub type Mvd33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVD33` reader - Enable Low Voltage Detect on AVDD supply line interrupt"]
pub type Lvd33R = crate::BitReader;
#[doc = "Field `LVD33` writer - Enable Low Voltage Detect on AVDD supply line interrupt"]
pub type Lvd33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVD33` reader - Enable High Voltage Detect on AVDD supply line interrupt"]
pub type Hvd33R = crate::BitReader;
#[doc = "Field `HVD33` writer - Enable High Voltage Detect on AVDD supply line interrupt"]
pub type Hvd33W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable bus timeout interrupt"]
    #[inline(always)]
    pub fn bustimout(&self) -> BustimoutR {
        BustimoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable node 0 fault interrupt"]
    #[inline(always)]
    pub fn node0fault(&self) -> Node0faultR {
        Node0faultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable node 1 fault interrupt"]
    #[inline(always)]
    pub fn node1fault(&self) -> Node1faultR {
        Node1faultR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable node 2 fault interrupt"]
    #[inline(always)]
    pub fn node2fault(&self) -> Node2faultR {
        Node2faultR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable node 3 fault interrupt"]
    #[inline(always)]
    pub fn node3fault(&self) -> Node3faultR {
        Node3faultR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable node 4 fault interrupt"]
    #[inline(always)]
    pub fn node4fault(&self) -> Node4faultR {
        Node4faultR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Minimal Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn mvd11(&self) -> Mvd11R {
        Mvd11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Low Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn lvd11(&self) -> Lvd11R {
        Lvd11R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable High Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn hvd11(&self) -> Hvd11R {
        Hvd11R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Minimal Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn mvd33(&self) -> Mvd33R {
        Mvd33R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Low Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn lvd33(&self) -> Lvd33R {
        Lvd33R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable High Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn hvd33(&self) -> Hvd33R {
        Hvd33R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bus timeout interrupt"]
    #[inline(always)]
    pub fn bustimout(&mut self) -> BustimoutW<'_, IntenSpec> {
        BustimoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable node 0 fault interrupt"]
    #[inline(always)]
    pub fn node0fault(&mut self) -> Node0faultW<'_, IntenSpec> {
        Node0faultW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable node 1 fault interrupt"]
    #[inline(always)]
    pub fn node1fault(&mut self) -> Node1faultW<'_, IntenSpec> {
        Node1faultW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable node 2 fault interrupt"]
    #[inline(always)]
    pub fn node2fault(&mut self) -> Node2faultW<'_, IntenSpec> {
        Node2faultW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable node 3 fault interrupt"]
    #[inline(always)]
    pub fn node3fault(&mut self) -> Node3faultW<'_, IntenSpec> {
        Node3faultW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable node 4 fault interrupt"]
    #[inline(always)]
    pub fn node4fault(&mut self) -> Node4faultW<'_, IntenSpec> {
        Node4faultW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Minimal Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn mvd11(&mut self) -> Mvd11W<'_, IntenSpec> {
        Mvd11W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Low Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn lvd11(&mut self) -> Lvd11W<'_, IntenSpec> {
        Lvd11W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable High Voltage Detect on DVDD supply line interrupt"]
    #[inline(always)]
    pub fn hvd11(&mut self) -> Hvd11W<'_, IntenSpec> {
        Hvd11W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Minimal Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn mvd33(&mut self) -> Mvd33W<'_, IntenSpec> {
        Mvd33W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Low Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn lvd33(&mut self) -> Lvd33W<'_, IntenSpec> {
        Lvd33W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable High Voltage Detect on AVDD supply line interrupt"]
    #[inline(always)]
    pub fn hvd33(&mut self) -> Hvd33W<'_, IntenSpec> {
        Hvd33W::new(self, 11)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
