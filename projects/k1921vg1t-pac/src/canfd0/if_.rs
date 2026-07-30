#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `AERG` reader - Acknowledge Error Interrupt"]
pub type AergR = crate::BitReader;
#[doc = "Field `AERG` writer - Acknowledge Error Interrupt"]
pub type AergW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERG` reader - Frame Error Interrupt"]
pub type FergR = crate::BitReader;
#[doc = "Field `FERG` writer - Frame Error Interrupt"]
pub type FergW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERG` reader - CRC Error Interrupt"]
pub type CergR = crate::BitReader;
#[doc = "Field `CERG` writer - CRC Error Interrupt"]
pub type CergW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERG` reader - Common Error Interrupt"]
pub type SergR = crate::BitReader;
#[doc = "Field `SERG` writer - Common Error Interrupt"]
pub type SergW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BXOK` reader - Frame Buffer Receive Interrupt"]
pub type BxokR = crate::BitReader;
#[doc = "Field `BXOK` writer - Frame Buffer Receive Interrupt"]
pub type BxokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRTIM` reader - CAN Timer Overrun Interrupt"]
pub type OvrtimR = crate::BitReader;
#[doc = "Field `OVRTIM` writer - CAN Timer Overrun Interrupt"]
pub type OvrtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFFIT` reader - Interrupt due to Bus Disable"]
pub type BoffitR = crate::BitReader;
#[doc = "Field `BOFFIT` writer - Interrupt due to Bus Disable"]
pub type BoffitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANIT` reader - Common Interrupt Flag"]
pub type CanitR = crate::BitReader;
#[doc = "Field `UNCORR_ERR` reader - Double Error while reading a memory"]
pub type UncorrErrR = crate::BitReader;
#[doc = "Field `UNCORR_ERR` writer - Double Error while reading a memory"]
pub type UncorrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_MEM` reader - Single Error when reading a memory"]
pub type ErrMemR = crate::BitReader;
#[doc = "Field `ERR_MEM` writer - Single Error when reading a memory"]
pub type ErrMemW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Acknowledge Error Interrupt"]
    #[inline(always)]
    pub fn aerg(&self) -> AergR {
        AergR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Error Interrupt"]
    #[inline(always)]
    pub fn ferg(&self) -> FergR {
        FergR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn cerg(&self) -> CergR {
        CergR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Common Error Interrupt"]
    #[inline(always)]
    pub fn serg(&self) -> SergR {
        SergR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame Buffer Receive Interrupt"]
    #[inline(always)]
    pub fn bxok(&self) -> BxokR {
        BxokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CAN Timer Overrun Interrupt"]
    #[inline(always)]
    pub fn ovrtim(&self) -> OvrtimR {
        OvrtimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt due to Bus Disable"]
    #[inline(always)]
    pub fn boffit(&self) -> BoffitR {
        BoffitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Common Interrupt Flag"]
    #[inline(always)]
    pub fn canit(&self) -> CanitR {
        CanitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Double Error while reading a memory"]
    #[inline(always)]
    pub fn uncorr_err(&self) -> UncorrErrR {
        UncorrErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Single Error when reading a memory"]
    #[inline(always)]
    pub fn err_mem(&self) -> ErrMemR {
        ErrMemR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledge Error Interrupt"]
    #[inline(always)]
    pub fn aerg(&mut self) -> AergW<'_, IfSpec> {
        AergW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Error Interrupt"]
    #[inline(always)]
    pub fn ferg(&mut self) -> FergW<'_, IfSpec> {
        FergW::new(self, 1)
    }
    #[doc = "Bit 2 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn cerg(&mut self) -> CergW<'_, IfSpec> {
        CergW::new(self, 2)
    }
    #[doc = "Bit 3 - Common Error Interrupt"]
    #[inline(always)]
    pub fn serg(&mut self) -> SergW<'_, IfSpec> {
        SergW::new(self, 3)
    }
    #[doc = "Bit 4 - Frame Buffer Receive Interrupt"]
    #[inline(always)]
    pub fn bxok(&mut self) -> BxokW<'_, IfSpec> {
        BxokW::new(self, 4)
    }
    #[doc = "Bit 5 - CAN Timer Overrun Interrupt"]
    #[inline(always)]
    pub fn ovrtim(&mut self) -> OvrtimW<'_, IfSpec> {
        OvrtimW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt due to Bus Disable"]
    #[inline(always)]
    pub fn boffit(&mut self) -> BoffitW<'_, IfSpec> {
        BoffitW::new(self, 6)
    }
    #[doc = "Bit 8 - Double Error while reading a memory"]
    #[inline(always)]
    pub fn uncorr_err(&mut self) -> UncorrErrW<'_, IfSpec> {
        UncorrErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Single Error when reading a memory"]
    #[inline(always)]
    pub fn err_mem(&mut self) -> ErrMemW<'_, IfSpec> {
        ErrMemW::new(self, 9)
    }
}
#[doc = "CAN General Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
