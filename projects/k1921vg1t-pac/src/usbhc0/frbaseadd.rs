#[doc = "Register `FRBASEADD` reader"]
pub type R = crate::R<FrbaseaddSpec>;
#[doc = "Register `FRBASEADD` writer"]
pub type W = crate::W<FrbaseaddSpec>;
#[doc = "Field `BASEADDRESS` reader - Beginning address of the Frame List in the system memory"]
pub type BaseaddressR = crate::FieldReader<u32>;
#[doc = "Field `BASEADDRESS` writer - Beginning address of the Frame List in the system memory"]
pub type BaseaddressW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - Beginning address of the Frame List in the system memory"]
    #[inline(always)]
    pub fn baseaddress(&self) -> BaseaddressR {
        BaseaddressR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - Beginning address of the Frame List in the system memory"]
    #[inline(always)]
    pub fn baseaddress(&mut self) -> BaseaddressW<'_, FrbaseaddSpec> {
        BaseaddressW::new(self, 12)
    }
}
#[doc = "USBHC UHC Frame List Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`frbaseadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frbaseadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrbaseaddSpec;
impl crate::RegisterSpec for FrbaseaddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frbaseadd::R`](R) reader structure"]
impl crate::Readable for FrbaseaddSpec {}
#[doc = "`write(|w| ..)` method takes [`frbaseadd::W`](W) writer structure"]
impl crate::Writable for FrbaseaddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRBASEADD to value 0"]
impl crate::Resettable for FrbaseaddSpec {}
