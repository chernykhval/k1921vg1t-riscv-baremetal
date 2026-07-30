#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `BYTE0` reader - Message Value (byte 0)"]
pub type Byte0R = crate::FieldReader;
#[doc = "Field `BYTE0` writer - Message Value (byte 0)"]
pub type Byte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE1` reader - Message Value (byte 1)"]
pub type Byte1R = crate::FieldReader;
#[doc = "Field `BYTE1` writer - Message Value (byte 1)"]
pub type Byte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE2` reader - Message Value (byte 2)"]
pub type Byte2R = crate::FieldReader;
#[doc = "Field `BYTE2` writer - Message Value (byte 2)"]
pub type Byte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE3` reader - Message Value (byte 3)"]
pub type Byte3R = crate::FieldReader;
#[doc = "Field `BYTE3` writer - Message Value (byte 3)"]
pub type Byte3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Message Value (byte 0)"]
    #[inline(always)]
    pub fn byte0(&self) -> Byte0R {
        Byte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Message Value (byte 1)"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Message Value (byte 2)"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        Byte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Message Value (byte 3)"]
    #[inline(always)]
    pub fn byte3(&self) -> Byte3R {
        Byte3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Message Value (byte 0)"]
    #[inline(always)]
    pub fn byte0(&mut self) -> Byte0W<'_, ValueSpec> {
        Byte0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Message Value (byte 1)"]
    #[inline(always)]
    pub fn byte1(&mut self) -> Byte1W<'_, ValueSpec> {
        Byte1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Message Value (byte 2)"]
    #[inline(always)]
    pub fn byte2(&mut self) -> Byte2W<'_, ValueSpec> {
        Byte2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Message Value (byte 3)"]
    #[inline(always)]
    pub fn byte3(&mut self) -> Byte3W<'_, ValueSpec> {
        Byte3W::new(self, 24)
    }
}
#[doc = "CAN Data Message Register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
