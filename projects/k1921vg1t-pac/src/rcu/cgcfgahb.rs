#[doc = "Register `CGCFGAHB` reader"]
pub type R = crate::R<CgcfgahbSpec>;
#[doc = "Register `CGCFGAHB` writer"]
pub type W = crate::W<CgcfgahbSpec>;
#[doc = "Field `GPIOAEN` reader - "]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - "]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - "]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - "]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - "]
pub type GpiocenR = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - "]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - "]
pub type GpiodenR = crate::BitReader;
#[doc = "Field `GPIODEN` writer - "]
pub type GpiodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - "]
pub type GpioeenR = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - "]
pub type GpioeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - "]
pub type GpiofenR = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - "]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGEN` reader - "]
pub type GpiogenR = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - "]
pub type GpiogenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANFDEN` reader - "]
pub type CanfdenR = crate::BitReader;
#[doc = "Field `CANFDEN` writer - "]
pub type CanfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANEN` reader - "]
pub type CanenR = crate::BitReader;
#[doc = "Field `CANEN` writer - "]
pub type CanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - "]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - "]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI0EN` reader - "]
pub type Qspi0enR = crate::BitReader;
#[doc = "Field `QSPI0EN` writer - "]
pub type Qspi0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI1EN` reader - "]
pub type Qspi1enR = crate::BitReader;
#[doc = "Field `QSPI1EN` writer - "]
pub type Qspi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - "]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - "]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTOEN` reader - "]
pub type CryptoenR = crate::BitReader;
#[doc = "Field `CRYPTOEN` writer - "]
pub type CryptoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHEN` reader - "]
pub type HashenR = crate::BitReader;
#[doc = "Field `HASHEN` writer - "]
pub type HashenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0DEN` reader - "]
pub type Usb0denR = crate::BitReader;
#[doc = "Field `USB0DEN` writer - "]
pub type Usb0denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0HEN` reader - "]
pub type Usb0henR = crate::BitReader;
#[doc = "Field `USB0HEN` writer - "]
pub type Usb0henW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1DEN` reader - "]
pub type Usb1denR = crate::BitReader;
#[doc = "Field `USB1DEN` writer - "]
pub type Usb1denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1HEN` reader - "]
pub type Usb1henR = crate::BitReader;
#[doc = "Field `USB1HEN` writer - "]
pub type Usb1henW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpioden(&self) -> GpiodenR {
        GpiodenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GpioeenR {
        GpioeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GpiogenR {
        GpiogenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn canfden(&self) -> CanfdenR {
        CanfdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn canen(&self) -> CanenR {
        CanenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn qspi0en(&self) -> Qspi0enR {
        Qspi0enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn qspi1en(&self) -> Qspi1enR {
        Qspi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cryptoen(&self) -> CryptoenR {
        CryptoenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn hashen(&self) -> HashenR {
        HashenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn usb0den(&self) -> Usb0denR {
        Usb0denR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn usb0hen(&self) -> Usb0henR {
        Usb0henR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usb1den(&self) -> Usb1denR {
        Usb1denR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn usb1hen(&self) -> Usb1henR {
        Usb1henR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GpioaenW<'_, CgcfgahbSpec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GpiobenW<'_, CgcfgahbSpec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GpiocenW<'_, CgcfgahbSpec> {
        GpiocenW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GpiodenW<'_, CgcfgahbSpec> {
        GpiodenW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GpioeenW<'_, CgcfgahbSpec> {
        GpioeenW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GpiofenW<'_, CgcfgahbSpec> {
        GpiofenW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GpiogenW<'_, CgcfgahbSpec> {
        GpiogenW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn canfden(&mut self) -> CanfdenW<'_, CgcfgahbSpec> {
        CanfdenW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn canen(&mut self) -> CanenW<'_, CgcfgahbSpec> {
        CanenW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<'_, CgcfgahbSpec> {
        AdcenW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn qspi0en(&mut self) -> Qspi0enW<'_, CgcfgahbSpec> {
        Qspi0enW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn qspi1en(&mut self) -> Qspi1enW<'_, CgcfgahbSpec> {
        Qspi1enW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, CgcfgahbSpec> {
        CrcenW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cryptoen(&mut self) -> CryptoenW<'_, CgcfgahbSpec> {
        CryptoenW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn hashen(&mut self) -> HashenW<'_, CgcfgahbSpec> {
        HashenW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn usb0den(&mut self) -> Usb0denW<'_, CgcfgahbSpec> {
        Usb0denW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn usb0hen(&mut self) -> Usb0henW<'_, CgcfgahbSpec> {
        Usb0henW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usb1den(&mut self) -> Usb1denW<'_, CgcfgahbSpec> {
        Usb1denW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn usb1hen(&mut self) -> Usb1henW<'_, CgcfgahbSpec> {
        Usb1henW::new(self, 19)
    }
}
#[doc = "Clockgate Config AHB register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcfgahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcfgahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgcfgahbSpec;
impl crate::RegisterSpec for CgcfgahbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgcfgahb::R`](R) reader structure"]
impl crate::Readable for CgcfgahbSpec {}
#[doc = "`write(|w| ..)` method takes [`cgcfgahb::W`](W) writer structure"]
impl crate::Writable for CgcfgahbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGCFGAHB to value 0"]
impl crate::Resettable for CgcfgahbSpec {}
