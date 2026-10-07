#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `CPHA` reader - Serial clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Serial clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Serial clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Serial clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDIV` reader - Serial clock DIV value"]
pub type CdivR = crate::FieldReader;
#[doc = "Field `CDIV` writer - Serial clock DIV value"]
pub type CdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Functional mode ( \\[00\\] - Std.SPI ; \\[01\\] - Reserved ; \\[10\\] - QSPI Memory Interface ; \\[11\\] - Test LoopBack )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fmod {
    #[doc = "0: Standart SPI"]
    Spi = 0,
    #[doc = "2: QSPI Memory Interface"]
    Qspi = 2,
    #[doc = "3: Test LoopBack"]
    Loopback = 3,
}
impl From<Fmod> for u8 {
    #[inline(always)]
    fn from(variant: Fmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fmod {
    type Ux = u8;
}
impl crate::IsEnum for Fmod {}
#[doc = "Field `FMOD` reader - Functional mode ( \\[00\\] - Std.SPI ; \\[01\\] - Reserved ; \\[10\\] - QSPI Memory Interface ; \\[11\\] - Test LoopBack )"]
pub type FmodR = crate::FieldReader<Fmod>;
impl FmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fmod> {
        match self.bits {
            0 => Some(Fmod::Spi),
            2 => Some(Fmod::Qspi),
            3 => Some(Fmod::Loopback),
            _ => None,
        }
    }
    #[doc = "Standart SPI"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Fmod::Spi
    }
    #[doc = "QSPI Memory Interface"]
    #[inline(always)]
    pub fn is_qspi(&self) -> bool {
        *self == Fmod::Qspi
    }
    #[doc = "Test LoopBack"]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == Fmod::Loopback
    }
}
#[doc = "Field `FMOD` writer - Functional mode ( \\[00\\] - Std.SPI ; \\[01\\] - Reserved ; \\[10\\] - QSPI Memory Interface ; \\[11\\] - Test LoopBack )"]
pub type FmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fmod>;
impl<'a, REG> FmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standart SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Fmod::Spi)
    }
    #[doc = "QSPI Memory Interface"]
    #[inline(always)]
    pub fn qspi(self) -> &'a mut crate::W<REG> {
        self.variant(Fmod::Qspi)
    }
    #[doc = "Test LoopBack"]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Fmod::Loopback)
    }
}
impl R {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial clock DIV value"]
    #[inline(always)]
    pub fn cdiv(&self) -> CdivR {
        CdivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Functional mode ( \\[00\\] - Std.SPI ; \\[01\\] - Reserved ; \\[10\\] - QSPI Memory Interface ; \\[11\\] - Test LoopBack )"]
    #[inline(always)]
    pub fn fmod(&self) -> FmodR {
        FmodR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, DcrSpec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, DcrSpec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Serial clock DIV value"]
    #[inline(always)]
    pub fn cdiv(&mut self) -> CdivW<'_, DcrSpec> {
        CdivW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Functional mode ( \\[00\\] - Std.SPI ; \\[01\\] - Reserved ; \\[10\\] - QSPI Memory Interface ; \\[11\\] - Test LoopBack )"]
    #[inline(always)]
    pub fn fmod(&mut self) -> FmodW<'_, DcrSpec> {
        FmodW::new(self, 16)
    }
}
#[doc = "Device Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCR to value 0x0100"]
impl crate::Resettable for DcrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
