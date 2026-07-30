#[doc = "Register `U_DATA` reader"]
pub type R = crate::R<UDataSpec>;
#[doc = "Field `BYTE0` reader - U_ITEM\\[0\\].DATA byte value"]
pub type Byte0R = crate::FieldReader;
#[doc = "Field `BYTE1` reader - U_ITEM\\[1\\].DATA byte value"]
pub type Byte1R = crate::FieldReader;
#[doc = "Field `BYTE2` reader - U_ITEM\\[2\\].DATA byte value"]
pub type Byte2R = crate::FieldReader;
#[doc = "Field `BYTE3` reader - U_ITEM\\[3\\].DATA byte value"]
pub type Byte3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - U_ITEM\\[0\\].DATA byte value"]
    #[inline(always)]
    pub fn byte0(&self) -> Byte0R {
        Byte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U_ITEM\\[1\\].DATA byte value"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U_ITEM\\[2\\].DATA byte value"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        Byte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - U_ITEM\\[3\\].DATA byte value"]
    #[inline(always)]
    pub fn byte3(&self) -> Byte3R {
        Byte3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Aggregated upstream items data\n\nYou can [`read`](crate::Reg::read) this register and get [`u_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDataSpec;
impl crate::RegisterSpec for UDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_data::R`](R) reader structure"]
impl crate::Readable for UDataSpec {}
#[doc = "`reset()` method sets U_DATA to value 0"]
impl crate::Resettable for UDataSpec {}
