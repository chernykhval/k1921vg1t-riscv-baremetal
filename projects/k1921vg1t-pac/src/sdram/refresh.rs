#[doc = "Register `REFRESH` reader"]
pub type R = crate::R<RefreshSpec>;
#[doc = "Register `REFRESH` writer"]
pub type W = crate::W<RefreshSpec>;
#[doc = "Field `RF_TIME` reader - Period between auto-refresh commands issued by the controller, specified in clocks"]
pub type RfTimeR = crate::FieldReader<u16>;
#[doc = "Field `RF_TIME` writer - Period between auto-refresh commands issued by the controller, specified in clocks"]
pub type RfTimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RF_MAX` reader - Maximum number of rows to be refreshed at a time"]
pub type RfMaxR = crate::FieldReader;
#[doc = "Field `RF_MAX` writer - Maximum number of rows to be refreshed at a time"]
pub type RfMaxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - Period between auto-refresh commands issued by the controller, specified in clocks"]
    #[inline(always)]
    pub fn rf_time(&self) -> RfTimeR {
        RfTimeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Maximum number of rows to be refreshed at a time"]
    #[inline(always)]
    pub fn rf_max(&self) -> RfMaxR {
        RfMaxR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Period between auto-refresh commands issued by the controller, specified in clocks"]
    #[inline(always)]
    pub fn rf_time(&mut self) -> RfTimeW<'_, RefreshSpec> {
        RfTimeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Maximum number of rows to be refreshed at a time"]
    #[inline(always)]
    pub fn rf_max(&mut self) -> RfMaxW<'_, RefreshSpec> {
        RfMaxW::new(self, 12)
    }
}
#[doc = "SDRAM Refresh register\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefreshSpec;
impl crate::RegisterSpec for RefreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refresh::R`](R) reader structure"]
impl crate::Readable for RefreshSpec {}
#[doc = "`write(|w| ..)` method takes [`refresh::W`](W) writer structure"]
impl crate::Writable for RefreshSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFRESH to value 0x6100"]
impl crate::Resettable for RefreshSpec {
    const RESET_VALUE: u32 = 0x6100;
}
