#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `TMR` reader - Timer Interrupt Status"]
pub type TmrR = crate::BitReader;
#[doc = "Field `CAPCOM0_0` reader - Capcom0 Interrupt Status"]
pub type Capcom0_0R = crate::BitReader;
#[doc = "Field `CAPCOM0_1` reader - Capcom1 Interrupt Status"]
pub type Capcom0_1R = crate::BitReader;
#[doc = "Field `CAPCOM1_0` reader - Capcom2 Interrupt Status"]
pub type Capcom1_0R = crate::BitReader;
#[doc = "Field `CAPCOM1_1` reader - Capcom3 Interrupt Status"]
pub type Capcom1_1R = crate::BitReader;
#[doc = "Field `CAPCOM2_0` reader - Capcom4 Interrupt Status"]
pub type Capcom2_0R = crate::BitReader;
#[doc = "Field `CAPCOM2_1` reader - Capcom5 Interrupt Status"]
pub type Capcom2_1R = crate::BitReader;
#[doc = "Field `CAPCOM3_0` reader - Capcom6 Interrupt Status"]
pub type Capcom3_0R = crate::BitReader;
#[doc = "Field `CAPCOM3_1` reader - Capcom7 Interrupt Status"]
pub type Capcom3_1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer Interrupt Status"]
    #[inline(always)]
    pub fn tmr(&self) -> TmrR {
        TmrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capcom0 Interrupt Status"]
    #[inline(always)]
    pub fn capcom0_0(&self) -> Capcom0_0R {
        Capcom0_0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capcom1 Interrupt Status"]
    #[inline(always)]
    pub fn capcom0_1(&self) -> Capcom0_1R {
        Capcom0_1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capcom2 Interrupt Status"]
    #[inline(always)]
    pub fn capcom1_0(&self) -> Capcom1_0R {
        Capcom1_0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capcom3 Interrupt Status"]
    #[inline(always)]
    pub fn capcom1_1(&self) -> Capcom1_1R {
        Capcom1_1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capcom4 Interrupt Status"]
    #[inline(always)]
    pub fn capcom2_0(&self) -> Capcom2_0R {
        Capcom2_0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capcom5 Interrupt Status"]
    #[inline(always)]
    pub fn capcom2_1(&self) -> Capcom2_1R {
        Capcom2_1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capcom6 Interrupt Status"]
    #[inline(always)]
    pub fn capcom3_0(&self) -> Capcom3_0R {
        Capcom3_0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capcom7 Interrupt Status"]
    #[inline(always)]
    pub fn capcom3_1(&self) -> Capcom3_1R {
        Capcom3_1R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
