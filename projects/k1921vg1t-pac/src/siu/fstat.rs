#[doc = "Register `FSTAT` reader"]
pub type R = crate::R<FstatSpec>;
#[doc = "Register `FSTAT` writer"]
pub type W = crate::W<FstatSpec>;
#[doc = "Field `NODE0FAULT` reader - Fault reset due node 0 fault"]
pub type Node0faultR = crate::BitReader;
#[doc = "Field `NODE0FAULT` writer - Fault reset due node 0 fault"]
pub type Node0faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE1FAULT` reader - Fault reset due node 1 fault"]
pub type Node1faultR = crate::BitReader;
#[doc = "Field `NODE1FAULT` writer - Fault reset due node 1 fault"]
pub type Node1faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE2FAULT` reader - Fault reset due node 2 fault"]
pub type Node2faultR = crate::BitReader;
#[doc = "Field `NODE2FAULT` writer - Fault reset due node 2 fault"]
pub type Node2faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE3FAULT` reader - Fault reset due node 3 fault"]
pub type Node3faultR = crate::BitReader;
#[doc = "Field `NODE3FAULT` writer - Fault reset due node 3 fault"]
pub type Node3faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODE4FAULT` reader - Fault reset due node 4 fault"]
pub type Node4faultR = crate::BitReader;
#[doc = "Field `NODE4FAULT` writer - Fault reset due node 4 fault"]
pub type Node4faultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVD11` reader - Fault reset due Minimal Voltage Detect on DVDD supply line"]
pub type Mvd11R = crate::BitReader;
#[doc = "Field `MVD11` writer - Fault reset due Minimal Voltage Detect on DVDD supply line"]
pub type Mvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVD11` reader - Fault reset due Low Voltage Detect on DVDD supply line"]
pub type Lvd11R = crate::BitReader;
#[doc = "Field `LVD11` writer - Fault reset due Low Voltage Detect on DVDD supply line"]
pub type Lvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVD11` reader - Fault reset due High Voltage Detect on DVDD supply line"]
pub type Hvd11R = crate::BitReader;
#[doc = "Field `HVD11` writer - Fault reset due High Voltage Detect on DVDD supply line"]
pub type Hvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVD33` reader - Fault reset due Minimal Voltage Detect on AVDD supply line"]
pub type Mvd33R = crate::BitReader;
#[doc = "Field `MVD33` writer - Fault reset due Minimal Voltage Detect on AVDD supply line"]
pub type Mvd33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVD33` reader - Fault reset due Low Voltage Detect on AVDD supply line"]
pub type Lvd33R = crate::BitReader;
#[doc = "Field `LVD33` writer - Fault reset due Low Voltage Detect on AVDD supply line"]
pub type Lvd33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVD33` reader - Fault reset due High Voltage Detect on AVDD supply line"]
pub type Hvd33R = crate::BitReader;
#[doc = "Field `HVD33` writer - Fault reset due High Voltage Detect on AVDD supply line"]
pub type Hvd33W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Fault reset due node 0 fault"]
    #[inline(always)]
    pub fn node0fault(&self) -> Node0faultR {
        Node0faultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault reset due node 1 fault"]
    #[inline(always)]
    pub fn node1fault(&self) -> Node1faultR {
        Node1faultR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault reset due node 2 fault"]
    #[inline(always)]
    pub fn node2fault(&self) -> Node2faultR {
        Node2faultR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault reset due node 3 fault"]
    #[inline(always)]
    pub fn node3fault(&self) -> Node3faultR {
        Node3faultR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault reset due node 4 fault"]
    #[inline(always)]
    pub fn node4fault(&self) -> Node4faultR {
        Node4faultR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault reset due Minimal Voltage Detect on DVDD supply line"]
    #[inline(always)]
    pub fn mvd11(&self) -> Mvd11R {
        Mvd11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault reset due Low Voltage Detect on DVDD supply line"]
    #[inline(always)]
    pub fn lvd11(&self) -> Lvd11R {
        Lvd11R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fault reset due High Voltage Detect on DVDD supply line"]
    #[inline(always)]
    pub fn hvd11(&self) -> Hvd11R {
        Hvd11R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fault reset due Minimal Voltage Detect on AVDD supply line"]
    #[inline(always)]
    pub fn mvd33(&self) -> Mvd33R {
        Mvd33R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fault reset due Low Voltage Detect on AVDD supply line"]
    #[inline(always)]
    pub fn lvd33(&self) -> Lvd33R {
        Lvd33R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fault reset due High Voltage Detect on AVDD supply line"]
    #[inline(always)]
    pub fn hvd33(&self) -> Hvd33R {
        Hvd33R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Fault reset due node 0 fault"]
    #[inline(always)]
    pub fn node0fault(&mut self) -> Node0faultW<'_, FstatSpec> {
        Node0faultW::new(self, 1)
    }
    #[doc = "Bit 2 - Fault reset due node 1 fault"]
    #[inline(always)]
    pub fn node1fault(&mut self) -> Node1faultW<'_, FstatSpec> {
        Node1faultW::new(self, 2)
    }
    #[doc = "Bit 3 - Fault reset due node 2 fault"]
    #[inline(always)]
    pub fn node2fault(&mut self) -> Node2faultW<'_, FstatSpec> {
        Node2faultW::new(self, 3)
    }
    #[doc = "Bit 4 - Fault reset due node 3 fault"]
    #[inline(always)]
    pub fn node3fault(&mut self) -> Node3faultW<'_, FstatSpec> {
        Node3faultW::new(self, 4)
    }
    #[doc = "Bit 5 - Fault reset due node 4 fault"]
    #[inline(always)]
    pub fn node4fault(&mut self) -> Node4faultW<'_, FstatSpec> {
        Node4faultW::new(self, 5)
    }
    #[doc = "Bit 6 - Fault reset due Minimal Voltage Detect on DVDD supply line"]
    #[inline(always)]
    pub fn mvd11(&mut self) -> Mvd11W<'_, FstatSpec> {
        Mvd11W::new(self, 6)
    }
    #[doc = "Bit 7 - Fault reset due Low Voltage Detect on DVDD supply line"]
    #[inline(always)]
    pub fn lvd11(&mut self) -> Lvd11W<'_, FstatSpec> {
        Lvd11W::new(self, 7)
    }
    #[doc = "Bit 8 - Fault reset due High Voltage Detect on DVDD supply line"]
    #[inline(always)]
    pub fn hvd11(&mut self) -> Hvd11W<'_, FstatSpec> {
        Hvd11W::new(self, 8)
    }
    #[doc = "Bit 9 - Fault reset due Minimal Voltage Detect on AVDD supply line"]
    #[inline(always)]
    pub fn mvd33(&mut self) -> Mvd33W<'_, FstatSpec> {
        Mvd33W::new(self, 9)
    }
    #[doc = "Bit 10 - Fault reset due Low Voltage Detect on AVDD supply line"]
    #[inline(always)]
    pub fn lvd33(&mut self) -> Lvd33W<'_, FstatSpec> {
        Lvd33W::new(self, 10)
    }
    #[doc = "Bit 11 - Fault reset due High Voltage Detect on AVDD supply line"]
    #[inline(always)]
    pub fn hvd33(&mut self) -> Hvd33W<'_, FstatSpec> {
        Hvd33W::new(self, 11)
    }
}
#[doc = "Fault status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FstatSpec;
impl crate::RegisterSpec for FstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstat::R`](R) reader structure"]
impl crate::Readable for FstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fstat::W`](W) writer structure"]
impl crate::Writable for FstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSTAT to value 0"]
impl crate::Resettable for FstatSpec {}
