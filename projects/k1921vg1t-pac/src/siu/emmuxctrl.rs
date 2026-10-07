#[doc = "Register `EMMUXCTRL` reader"]
pub type R = crate::R<EmmuxctrlSpec>;
#[doc = "Register `EMMUXCTRL` writer"]
pub type W = crate::W<EmmuxctrlSpec>;
#[doc = "Connect outputs of SDRAM IP to external memory mapped GPIO pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdramen {
    #[doc = "0: EMC connected to external memory mapped GPIO pins"]
    Emc = 0,
    #[doc = "1: SDRAM connected to external memory mapped GPIO pins"]
    Sdram = 1,
}
impl From<Sdramen> for bool {
    #[inline(always)]
    fn from(variant: Sdramen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDRAMEN` reader - Connect outputs of SDRAM IP to external memory mapped GPIO pins"]
pub type SdramenR = crate::BitReader<Sdramen>;
impl SdramenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdramen {
        match self.bits {
            false => Sdramen::Emc,
            true => Sdramen::Sdram,
        }
    }
    #[doc = "EMC connected to external memory mapped GPIO pins"]
    #[inline(always)]
    pub fn is_emc(&self) -> bool {
        *self == Sdramen::Emc
    }
    #[doc = "SDRAM connected to external memory mapped GPIO pins"]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == Sdramen::Sdram
    }
}
#[doc = "Field `SDRAMEN` writer - Connect outputs of SDRAM IP to external memory mapped GPIO pins"]
pub type SdramenW<'a, REG> = crate::BitWriter<'a, REG, Sdramen>;
impl<'a, REG> SdramenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EMC connected to external memory mapped GPIO pins"]
    #[inline(always)]
    pub fn emc(self) -> &'a mut crate::W<REG> {
        self.variant(Sdramen::Emc)
    }
    #[doc = "SDRAM connected to external memory mapped GPIO pins"]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut crate::W<REG> {
        self.variant(Sdramen::Sdram)
    }
}
impl R {
    #[doc = "Bit 0 - Connect outputs of SDRAM IP to external memory mapped GPIO pins"]
    #[inline(always)]
    pub fn sdramen(&self) -> SdramenR {
        SdramenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Connect outputs of SDRAM IP to external memory mapped GPIO pins"]
    #[inline(always)]
    pub fn sdramen(&mut self) -> SdramenW<'_, EmmuxctrlSpec> {
        SdramenW::new(self, 0)
    }
}
#[doc = "Extrernal memory multiplexor control register\n\nYou can [`read`](crate::Reg::read) this register and get [`emmuxctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emmuxctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmuxctrlSpec;
impl crate::RegisterSpec for EmmuxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmuxctrl::R`](R) reader structure"]
impl crate::Readable for EmmuxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`emmuxctrl::W`](W) writer structure"]
impl crate::Writable for EmmuxctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMMUXCTRL to value 0"]
impl crate::Resettable for EmmuxctrlSpec {}
