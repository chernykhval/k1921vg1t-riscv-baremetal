#[doc = "Register `GSTAT` reader"]
pub type R = crate::R<GstatSpec>;
#[doc = "Register `GSTAT` writer"]
pub type W = crate::W<GstatSpec>;
#[doc = "Field `FN` reader - Frame number"]
pub type FnR = crate::FieldReader<u16>;
#[doc = "Field `AF` reader - Additional frames"]
pub type AfR = crate::FieldReader;
#[doc = "Field `SP` reader - Speed"]
pub type SpR = crate::BitReader;
#[doc = "Field `VB` reader - VBUS valid"]
pub type VbR = crate::BitReader;
#[doc = "Field `UR` reader - USB reset"]
pub type UrR = crate::BitReader;
#[doc = "Field `UR` writer - USB reset"]
pub type UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SU` reader - Suspend"]
pub type SuR = crate::BitReader;
#[doc = "Data mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dm {
    #[doc = "0: core uses slave mode for data transfers"]
    SlaveDataMode = 0,
    #[doc = "1: core uses master mode (DMA) for data transfers"]
    MasterDataMode = 1,
}
impl From<Dm> for bool {
    #[inline(always)]
    fn from(variant: Dm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DM` reader - Data mode"]
pub type DmR = crate::BitReader<Dm>;
impl DmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dm {
        match self.bits {
            false => Dm::SlaveDataMode,
            true => Dm::MasterDataMode,
        }
    }
    #[doc = "core uses slave mode for data transfers"]
    #[inline(always)]
    pub fn is_slave_data_mode(&self) -> bool {
        *self == Dm::SlaveDataMode
    }
    #[doc = "core uses master mode (DMA) for data transfers"]
    #[inline(always)]
    pub fn is_master_data_mode(&self) -> bool {
        *self == Dm::MasterDataMode
    }
}
#[doc = "Field `NEPO` reader - Number of implemented OUT endpoints"]
pub type NepoR = crate::FieldReader;
#[doc = "Field `NEPI` reader - Number of implemented IN endpoints"]
pub type NepiR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - Frame number"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:13 - Additional frames"]
    #[inline(always)]
    pub fn af(&self) -> AfR {
        AfR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VBUS valid"]
    #[inline(always)]
    pub fn vb(&self) -> VbR {
        VbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USB reset"]
    #[inline(always)]
    pub fn ur(&self) -> UrR {
        UrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Suspend"]
    #[inline(always)]
    pub fn su(&self) -> SuR {
        SuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Data mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Number of implemented OUT endpoints"]
    #[inline(always)]
    pub fn nepo(&self) -> NepoR {
        NepoR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Number of implemented IN endpoints"]
    #[inline(always)]
    pub fn nepi(&self) -> NepiR {
        NepiR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - USB reset"]
    #[inline(always)]
    pub fn ur(&mut self) -> UrW<'_, GstatSpec> {
        UrW::new(self, 16)
    }
}
#[doc = "USBDC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GstatSpec;
impl crate::RegisterSpec for GstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gstat::R`](R) reader structure"]
impl crate::Readable for GstatSpec {}
#[doc = "`write(|w| ..)` method takes [`gstat::W`](W) writer structure"]
impl crate::Writable for GstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSTAT to value 0"]
impl crate::Resettable for GstatSpec {}
