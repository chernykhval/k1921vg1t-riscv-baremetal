#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `BUSTIMOUT` reader - Bus timeout interrupt status"]
pub type BustimoutR = crate::BitReader;
#[doc = "Field `NODE0FAULT` reader - Node 0 fault status"]
pub type Node0faultR = crate::BitReader;
#[doc = "Field `NODE1FAULT` reader - Node 1 fault status"]
pub type Node1faultR = crate::BitReader;
#[doc = "Field `NODE2FAULT` reader - Node 2 fault status"]
pub type Node2faultR = crate::BitReader;
#[doc = "Field `NODE3FAULT` reader - Node 3 fault status"]
pub type Node3faultR = crate::BitReader;
#[doc = "Field `NODE4FAULT` reader - Node 4 fault status"]
pub type Node4faultR = crate::BitReader;
#[doc = "Field `MVD11` reader - Minimal Voltage Detect on DVDD supply line status"]
pub type Mvd11R = crate::BitReader;
#[doc = "Field `LVD11` reader - Low Voltage Detect on DVDD supply line status"]
pub type Lvd11R = crate::BitReader;
#[doc = "Field `HVD11` reader - High Voltage Detect on DVDD supply line status"]
pub type Hvd11R = crate::BitReader;
#[doc = "Field `MVD33` reader - Minimal Voltage Detect on AVDD supply line status"]
pub type Mvd33R = crate::BitReader;
#[doc = "Field `LVD33` reader - Low Voltage Detect on AVDD supply line status"]
pub type Lvd33R = crate::BitReader;
#[doc = "Field `HVD33` reader - High Voltage Detect on AVDD supply line status"]
pub type Hvd33R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Bus timeout interrupt status"]
    #[inline(always)]
    pub fn bustimout(&self) -> BustimoutR {
        BustimoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Node 0 fault status"]
    #[inline(always)]
    pub fn node0fault(&self) -> Node0faultR {
        Node0faultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Node 1 fault status"]
    #[inline(always)]
    pub fn node1fault(&self) -> Node1faultR {
        Node1faultR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Node 2 fault status"]
    #[inline(always)]
    pub fn node2fault(&self) -> Node2faultR {
        Node2faultR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Node 3 fault status"]
    #[inline(always)]
    pub fn node3fault(&self) -> Node3faultR {
        Node3faultR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Node 4 fault status"]
    #[inline(always)]
    pub fn node4fault(&self) -> Node4faultR {
        Node4faultR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Minimal Voltage Detect on DVDD supply line status"]
    #[inline(always)]
    pub fn mvd11(&self) -> Mvd11R {
        Mvd11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low Voltage Detect on DVDD supply line status"]
    #[inline(always)]
    pub fn lvd11(&self) -> Lvd11R {
        Lvd11R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Voltage Detect on DVDD supply line status"]
    #[inline(always)]
    pub fn hvd11(&self) -> Hvd11R {
        Hvd11R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Minimal Voltage Detect on AVDD supply line status"]
    #[inline(always)]
    pub fn mvd33(&self) -> Mvd33R {
        Mvd33R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low Voltage Detect on AVDD supply line status"]
    #[inline(always)]
    pub fn lvd33(&self) -> Lvd33R {
        Lvd33R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - High Voltage Detect on AVDD supply line status"]
    #[inline(always)]
    pub fn hvd33(&self) -> Hvd33R {
        Hvd33R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {}
