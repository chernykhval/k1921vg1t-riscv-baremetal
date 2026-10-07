#[doc = "Register `TSENSCTRL` reader"]
pub type R = crate::R<TsensctrlSpec>;
#[doc = "Register `TSENSCTRL` writer"]
pub type W = crate::W<TsensctrlSpec>;
#[doc = "Field `DIS` reader - Disable tsensor"]
pub type DisR = crate::BitReader;
#[doc = "Field `DIS` writer - Disable tsensor"]
pub type DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALE` reader - Scale out control"]
pub type ScaleR = crate::BitReader;
#[doc = "Field `SCALE` writer - Scale out control"]
pub type ScaleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable tsensor"]
    #[inline(always)]
    pub fn dis(&self) -> DisR {
        DisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scale out control"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable tsensor"]
    #[inline(always)]
    pub fn dis(&mut self) -> DisW<'_, TsensctrlSpec> {
        DisW::new(self, 0)
    }
    #[doc = "Bit 1 - Scale out control"]
    #[inline(always)]
    pub fn scale(&mut self) -> ScaleW<'_, TsensctrlSpec> {
        ScaleW::new(self, 1)
    }
}
#[doc = "Tempreture sensor control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsensctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsensctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsensctrlSpec;
impl crate::RegisterSpec for TsensctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsensctrl::R`](R) reader structure"]
impl crate::Readable for TsensctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tsensctrl::W`](W) writer structure"]
impl crate::Writable for TsensctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSENSCTRL to value 0x01"]
impl crate::Resettable for TsensctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
