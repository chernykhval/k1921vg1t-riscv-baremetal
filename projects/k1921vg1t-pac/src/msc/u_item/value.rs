#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `DATA` reader - Data field"]
pub type DataR = crate::FieldReader;
#[doc = "Field `ADDR` reader - Address field"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `VALID` reader - item Valid bit"]
pub type ValidR = crate::BitReader;
#[doc = "Field `RP` reader - Received Parity bit"]
pub type RpR = crate::BitReader;
#[doc = "Field `CLEAR` writer - Clear item valit bit"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_HS` reader - Erorr High Start bit"]
pub type ErrHsR = crate::BitReader;
#[doc = "Field `ERR_P` reader - Error Parity bit"]
pub type ErrPR = crate::BitReader;
#[doc = "Field `ERR_LS` reader - Error Low Stop bit(s)"]
pub type ErrLsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Data field"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Address field"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - item Valid bit"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Received Parity bit"]
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Erorr High Start bit"]
    #[inline(always)]
    pub fn err_hs(&self) -> ErrHsR {
        ErrHsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Error Parity bit"]
    #[inline(always)]
    pub fn err_p(&self) -> ErrPR {
        ErrPR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Error Low Stop bit(s)"]
    #[inline(always)]
    pub fn err_ls(&self) -> ErrLsR {
        ErrLsR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Clear item valit bit"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, ValueSpec> {
        ClearW::new(self, 18)
    }
}
#[doc = "Upstream item index 0\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
