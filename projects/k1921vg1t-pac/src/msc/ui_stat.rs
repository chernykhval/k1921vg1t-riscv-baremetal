#[doc = "Register `UI_STAT` reader"]
pub type R = crate::R<UiStatSpec>;
#[doc = "Field `VALID_0` reader - Item 0 valid bit"]
pub type Valid0R = crate::BitReader;
#[doc = "Field `VALID_1` reader - Item 1 valid bit"]
pub type Valid1R = crate::BitReader;
#[doc = "Field `VALID_2` reader - Item 2 valid bit"]
pub type Valid2R = crate::BitReader;
#[doc = "Field `VALID_3` reader - Item 3 valid bit"]
pub type Valid3R = crate::BitReader;
#[doc = "Field `ERROR_0` reader - Item 0 combined error"]
pub type Error0R = crate::BitReader;
#[doc = "Field `ERROR_1` reader - Item 1 combined error"]
pub type Error1R = crate::BitReader;
#[doc = "Field `ERROR_2` reader - Item 2 combined error"]
pub type Error2R = crate::BitReader;
#[doc = "Field `ERROR_3` reader - Item 3 combined error"]
pub type Error3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Item 0 valid bit"]
    #[inline(always)]
    pub fn valid_0(&self) -> Valid0R {
        Valid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Item 1 valid bit"]
    #[inline(always)]
    pub fn valid_1(&self) -> Valid1R {
        Valid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Item 2 valid bit"]
    #[inline(always)]
    pub fn valid_2(&self) -> Valid2R {
        Valid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Item 3 valid bit"]
    #[inline(always)]
    pub fn valid_3(&self) -> Valid3R {
        Valid3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Item 0 combined error"]
    #[inline(always)]
    pub fn error_0(&self) -> Error0R {
        Error0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Item 1 combined error"]
    #[inline(always)]
    pub fn error_1(&self) -> Error1R {
        Error1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Item 2 combined error"]
    #[inline(always)]
    pub fn error_2(&self) -> Error2R {
        Error2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Item 3 combined error"]
    #[inline(always)]
    pub fn error_3(&self) -> Error3R {
        Error3R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Upstream items status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ui_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UiStatSpec;
impl crate::RegisterSpec for UiStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ui_stat::R`](R) reader structure"]
impl crate::Readable for UiStatSpec {}
#[doc = "`reset()` method sets UI_STAT to value 0"]
impl crate::Resettable for UiStatSpec {}
