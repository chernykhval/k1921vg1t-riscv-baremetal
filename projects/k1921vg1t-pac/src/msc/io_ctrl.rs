#[doc = "Register `IO_CTRL` reader"]
pub type R = crate::R<IoCtrlSpec>;
#[doc = "Register `IO_CTRL` writer"]
pub type W = crate::W<IoCtrlSpec>;
#[doc = "Field `IC` reader - Invert Clock output"]
pub type IcR = crate::BitReader;
#[doc = "Field `IC` writer - Invert Clock output"]
pub type IcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDTS` reader - Invert Data To Slave output"]
pub type IdtsR = crate::BitReader;
#[doc = "Field `IDTS` writer - Invert Data To Slave output"]
pub type IdtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISS` reader - Invert Slave Select output"]
pub type IssR = crate::BitReader;
#[doc = "Field `ISS` writer - Invert Slave Select output"]
pub type IssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IU` reader - Invert Upstream input"]
pub type IuR = crate::BitReader;
#[doc = "Field `IU` writer - Invert Upstream input"]
pub type IuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAT` reader - Clock All Time"]
pub type CatR = crate::BitReader;
#[doc = "Field `CAT` writer - Clock All Time"]
pub type CatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSI` reader - Low data part Slave Index"]
pub type LsiR = crate::FieldReader;
#[doc = "Field `LSI` writer - Low data part Slave Index"]
pub type LsiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSI` reader - High data part Slave Index"]
pub type HsiR = crate::FieldReader;
#[doc = "Field `HSI` writer - High data part Slave Index"]
pub type HsiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSI` reader - Command Slave Index"]
pub type CsiR = crate::FieldReader;
#[doc = "Field `CSI` writer - Command Slave Index"]
pub type CsiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UI` reader - Upstream input Index"]
pub type UiR = crate::BitReader;
#[doc = "Field `UI` writer - Upstream input Index"]
pub type UiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invert Clock output"]
    #[inline(always)]
    pub fn ic(&self) -> IcR {
        IcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invert Data To Slave output"]
    #[inline(always)]
    pub fn idts(&self) -> IdtsR {
        IdtsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invert Slave Select output"]
    #[inline(always)]
    pub fn iss(&self) -> IssR {
        IssR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert Upstream input"]
    #[inline(always)]
    pub fn iu(&self) -> IuR {
        IuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock All Time"]
    #[inline(always)]
    pub fn cat(&self) -> CatR {
        CatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Low data part Slave Index"]
    #[inline(always)]
    pub fn lsi(&self) -> LsiR {
        LsiR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - High data part Slave Index"]
    #[inline(always)]
    pub fn hsi(&self) -> HsiR {
        HsiR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Command Slave Index"]
    #[inline(always)]
    pub fn csi(&self) -> CsiR {
        CsiR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Upstream input Index"]
    #[inline(always)]
    pub fn ui(&self) -> UiR {
        UiR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert Clock output"]
    #[inline(always)]
    pub fn ic(&mut self) -> IcW<'_, IoCtrlSpec> {
        IcW::new(self, 0)
    }
    #[doc = "Bit 1 - Invert Data To Slave output"]
    #[inline(always)]
    pub fn idts(&mut self) -> IdtsW<'_, IoCtrlSpec> {
        IdtsW::new(self, 1)
    }
    #[doc = "Bit 2 - Invert Slave Select output"]
    #[inline(always)]
    pub fn iss(&mut self) -> IssW<'_, IoCtrlSpec> {
        IssW::new(self, 2)
    }
    #[doc = "Bit 3 - Invert Upstream input"]
    #[inline(always)]
    pub fn iu(&mut self) -> IuW<'_, IoCtrlSpec> {
        IuW::new(self, 3)
    }
    #[doc = "Bit 8 - Clock All Time"]
    #[inline(always)]
    pub fn cat(&mut self) -> CatW<'_, IoCtrlSpec> {
        CatW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Low data part Slave Index"]
    #[inline(always)]
    pub fn lsi(&mut self) -> LsiW<'_, IoCtrlSpec> {
        LsiW::new(self, 16)
    }
    #[doc = "Bits 20:21 - High data part Slave Index"]
    #[inline(always)]
    pub fn hsi(&mut self) -> HsiW<'_, IoCtrlSpec> {
        HsiW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Command Slave Index"]
    #[inline(always)]
    pub fn csi(&mut self) -> CsiW<'_, IoCtrlSpec> {
        CsiW::new(self, 24)
    }
    #[doc = "Bit 28 - Upstream input Index"]
    #[inline(always)]
    pub fn ui(&mut self) -> UiW<'_, IoCtrlSpec> {
        UiW::new(self, 28)
    }
}
#[doc = "Input/output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoCtrlSpec;
impl crate::RegisterSpec for IoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ctrl::R`](R) reader structure"]
impl crate::Readable for IoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`io_ctrl::W`](W) writer structure"]
impl crate::Writable for IoCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_CTRL to value 0"]
impl crate::Resettable for IoCtrlSpec {}
