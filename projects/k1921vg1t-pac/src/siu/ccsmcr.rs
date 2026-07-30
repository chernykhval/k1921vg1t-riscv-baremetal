#[doc = "Register `CCSMCR` reader"]
pub type R = crate::R<CcsmcrSpec>;
#[doc = "Register `CCSMCR` writer"]
pub type W = crate::W<CcsmcrSpec>;
#[doc = "Field `L1IRMEN` reader - Enable to use Read-Write Margin for L1I Cache"]
pub type L1irmenR = crate::BitReader;
#[doc = "Field `L1IRMEN` writer - Enable to use Read-Write Margin for L1I Cache"]
pub type L1irmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1IRM` reader - Read-Write Margin value for L1I Cache"]
pub type L1irmR = crate::FieldReader;
#[doc = "Field `L1IRM` writer - Read-Write Margin value for L1I Cache"]
pub type L1irmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `L1I_LS` reader - Light Sleep Control for L1I Cache"]
pub type L1iLsR = crate::BitReader;
#[doc = "Field `L1I_LS` writer - Light Sleep Control for L1I Cache"]
pub type L1iLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1DRMEN` reader - Enable to use Read-Write Margin for L1D Cache"]
pub type L1drmenR = crate::BitReader;
#[doc = "Field `L1DRMEN` writer - Enable to use Read-Write Margin for L1D Cache"]
pub type L1drmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1DRM` reader - Read-Write Margin value for L1D Cache"]
pub type L1drmR = crate::FieldReader;
#[doc = "Field `L1DRM` writer - Read-Write Margin value for L1D Cache"]
pub type L1drmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `L1D_LS` reader - Light Sleep Control for L1D Cache"]
pub type L1dLsR = crate::BitReader;
#[doc = "Field `L1D_LS` writer - Light Sleep Control for L1D Cache"]
pub type L1dLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2RMEN` reader - Enable to use Read-Write Margin for L2 Cache"]
pub type L2rmenR = crate::BitReader;
#[doc = "Field `L2RMEN` writer - Enable to use Read-Write Margin for L2 Cache"]
pub type L2rmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2RM` reader - Read-Write Margin value for L2 Cache"]
pub type L2rmR = crate::FieldReader;
#[doc = "Field `L2RM` writer - Read-Write Margin value for L2 Cache"]
pub type L2rmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `L2_LS` reader - Light Sleep Control for L2 Cache"]
pub type L2LsR = crate::BitReader;
#[doc = "Field `L2_LS` writer - Light Sleep Control for L2 Cache"]
pub type L2LsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable to use Read-Write Margin for L1I Cache"]
    #[inline(always)]
    pub fn l1irmen(&self) -> L1irmenR {
        L1irmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Read-Write Margin value for L1I Cache"]
    #[inline(always)]
    pub fn l1irm(&self) -> L1irmR {
        L1irmR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Light Sleep Control for L1I Cache"]
    #[inline(always)]
    pub fn l1i_ls(&self) -> L1iLsR {
        L1iLsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable to use Read-Write Margin for L1D Cache"]
    #[inline(always)]
    pub fn l1drmen(&self) -> L1drmenR {
        L1drmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Read-Write Margin value for L1D Cache"]
    #[inline(always)]
    pub fn l1drm(&self) -> L1drmR {
        L1drmR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Light Sleep Control for L1D Cache"]
    #[inline(always)]
    pub fn l1d_ls(&self) -> L1dLsR {
        L1dLsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable to use Read-Write Margin for L2 Cache"]
    #[inline(always)]
    pub fn l2rmen(&self) -> L2rmenR {
        L2rmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Read-Write Margin value for L2 Cache"]
    #[inline(always)]
    pub fn l2rm(&self) -> L2rmR {
        L2rmR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Light Sleep Control for L2 Cache"]
    #[inline(always)]
    pub fn l2_ls(&self) -> L2LsR {
        L2LsR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable to use Read-Write Margin for L1I Cache"]
    #[inline(always)]
    pub fn l1irmen(&mut self) -> L1irmenW<'_, CcsmcrSpec> {
        L1irmenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Read-Write Margin value for L1I Cache"]
    #[inline(always)]
    pub fn l1irm(&mut self) -> L1irmW<'_, CcsmcrSpec> {
        L1irmW::new(self, 1)
    }
    #[doc = "Bit 3 - Light Sleep Control for L1I Cache"]
    #[inline(always)]
    pub fn l1i_ls(&mut self) -> L1iLsW<'_, CcsmcrSpec> {
        L1iLsW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable to use Read-Write Margin for L1D Cache"]
    #[inline(always)]
    pub fn l1drmen(&mut self) -> L1drmenW<'_, CcsmcrSpec> {
        L1drmenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Read-Write Margin value for L1D Cache"]
    #[inline(always)]
    pub fn l1drm(&mut self) -> L1drmW<'_, CcsmcrSpec> {
        L1drmW::new(self, 5)
    }
    #[doc = "Bit 7 - Light Sleep Control for L1D Cache"]
    #[inline(always)]
    pub fn l1d_ls(&mut self) -> L1dLsW<'_, CcsmcrSpec> {
        L1dLsW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable to use Read-Write Margin for L2 Cache"]
    #[inline(always)]
    pub fn l2rmen(&mut self) -> L2rmenW<'_, CcsmcrSpec> {
        L2rmenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Read-Write Margin value for L2 Cache"]
    #[inline(always)]
    pub fn l2rm(&mut self) -> L2rmW<'_, CcsmcrSpec> {
        L2rmW::new(self, 9)
    }
    #[doc = "Bit 11 - Light Sleep Control for L2 Cache"]
    #[inline(always)]
    pub fn l2_ls(&mut self) -> L2LsW<'_, CcsmcrSpec> {
        L2LsW::new(self, 11)
    }
}
#[doc = "Core caches sleep and margin control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccsmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcsmcrSpec;
impl crate::RegisterSpec for CcsmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccsmcr::R`](R) reader structure"]
impl crate::Readable for CcsmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccsmcr::W`](W) writer structure"]
impl crate::Writable for CcsmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCSMCR to value 0"]
impl crate::Resettable for CcsmcrSpec {}
