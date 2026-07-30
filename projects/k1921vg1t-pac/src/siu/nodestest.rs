#[doc = "Register `NODESTEST` reader"]
pub type R = crate::R<NodestestSpec>;
#[doc = "Register `NODESTEST` writer"]
pub type W = crate::W<NodestestSpec>;
#[doc = "Field `START` writer - Start nodes selftest"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR` writer - Clear status bits"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STC0` reader - Self test for node 0 complete"]
pub type Stc0R = crate::BitReader;
#[doc = "Field `STC1` reader - Self test for node 1 complete"]
pub type Stc1R = crate::BitReader;
#[doc = "Field `STC2` reader - Self test for node 2 complete"]
pub type Stc2R = crate::BitReader;
#[doc = "Field `STC3` reader - Self test for node 3 complete"]
pub type Stc3R = crate::BitReader;
#[doc = "Field `STC4` reader - Self test for node 4 complete"]
pub type Stc4R = crate::BitReader;
#[doc = "Field `STE0` reader - Self test for node 0 ends with errors"]
pub type Ste0R = crate::BitReader;
#[doc = "Field `STE1` reader - Self test for node 1 ends with errors"]
pub type Ste1R = crate::BitReader;
#[doc = "Field `STE2` reader - Self test for node 2 ends with errors"]
pub type Ste2R = crate::BitReader;
#[doc = "Field `STE3` reader - Self test for node 3 ends with errors"]
pub type Ste3R = crate::BitReader;
#[doc = "Field `STE4` reader - Self test for node 4 ends with errors"]
pub type Ste4R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - Self test for node 0 complete"]
    #[inline(always)]
    pub fn stc0(&self) -> Stc0R {
        Stc0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Self test for node 1 complete"]
    #[inline(always)]
    pub fn stc1(&self) -> Stc1R {
        Stc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Self test for node 2 complete"]
    #[inline(always)]
    pub fn stc2(&self) -> Stc2R {
        Stc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Self test for node 3 complete"]
    #[inline(always)]
    pub fn stc3(&self) -> Stc3R {
        Stc3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Self test for node 4 complete"]
    #[inline(always)]
    pub fn stc4(&self) -> Stc4R {
        Stc4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Self test for node 0 ends with errors"]
    #[inline(always)]
    pub fn ste0(&self) -> Ste0R {
        Ste0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Self test for node 1 ends with errors"]
    #[inline(always)]
    pub fn ste1(&self) -> Ste1R {
        Ste1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Self test for node 2 ends with errors"]
    #[inline(always)]
    pub fn ste2(&self) -> Ste2R {
        Ste2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Self test for node 3 ends with errors"]
    #[inline(always)]
    pub fn ste3(&self) -> Ste3R {
        Ste3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Self test for node 4 ends with errors"]
    #[inline(always)]
    pub fn ste4(&self) -> Ste4R {
        Ste4R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start nodes selftest"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, NodestestSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear status bits"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, NodestestSpec> {
        ClrW::new(self, 1)
    }
}
#[doc = "Nodes selftest start and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`nodestest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nodestest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NodestestSpec;
impl crate::RegisterSpec for NodestestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nodestest::R`](R) reader structure"]
impl crate::Readable for NodestestSpec {}
#[doc = "`write(|w| ..)` method takes [`nodestest::W`](W) writer structure"]
impl crate::Writable for NodestestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NODESTEST to value 0"]
impl crate::Resettable for NodestestSpec {}
