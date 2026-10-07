#[doc = "Register `I2SSHIFTDATA` reader"]
pub type R = crate::R<I2sshiftdataSpec>;
#[doc = "Register `I2SSHIFTDATA` writer"]
pub type W = crate::W<I2sshiftdataSpec>;
#[doc = "Field `LSHIFT_VAL` reader - Number of counts to be shifted left"]
pub type LshiftValR = crate::FieldReader;
#[doc = "Field `LSHIFT_VAL` writer - Number of counts to be shifted left"]
pub type LshiftValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RSHIFT_VAL` reader - Number of counts to be shifted right"]
pub type RshiftValR = crate::FieldReader;
#[doc = "Field `RSHIFT_VAL` writer - Number of counts to be shifted right"]
pub type RshiftValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of counts to be shifted left"]
    #[inline(always)]
    pub fn lshift_val(&self) -> LshiftValR {
        LshiftValR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Number of counts to be shifted right"]
    #[inline(always)]
    pub fn rshift_val(&self) -> RshiftValR {
        RshiftValR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of counts to be shifted left"]
    #[inline(always)]
    pub fn lshift_val(&mut self) -> LshiftValW<'_, I2sshiftdataSpec> {
        LshiftValW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Number of counts to be shifted right"]
    #[inline(always)]
    pub fn rshift_val(&mut self) -> RshiftValW<'_, I2sshiftdataSpec> {
        RshiftValW::new(self, 8)
    }
}
#[doc = "I2S Shift Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sshiftdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sshiftdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sshiftdataSpec;
impl crate::RegisterSpec for I2sshiftdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sshiftdata::R`](R) reader structure"]
impl crate::Readable for I2sshiftdataSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sshiftdata::W`](W) writer structure"]
impl crate::Writable for I2sshiftdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SSHIFTDATA to value 0"]
impl crate::Resettable for I2sshiftdataSpec {}
