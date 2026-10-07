#[doc = "Register `CORE1_CONF` reader"]
pub type R = crate::R<Core1ConfSpec>;
#[doc = "Register `CORE1_CONF` writer"]
pub type W = crate::W<Core1ConfSpec>;
#[doc = "If set, second core work independent from first core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dualcore {
    #[doc = "0: Second core work in lockstep mode"]
    Lockstep = 0,
    #[doc = "1: Second core work independently"]
    Dualcore = 1,
}
impl From<Dualcore> for bool {
    #[inline(always)]
    fn from(variant: Dualcore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUALCORE` reader - If set, second core work independent from first core"]
pub type DualcoreR = crate::BitReader<Dualcore>;
impl DualcoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dualcore {
        match self.bits {
            false => Dualcore::Lockstep,
            true => Dualcore::Dualcore,
        }
    }
    #[doc = "Second core work in lockstep mode"]
    #[inline(always)]
    pub fn is_lockstep(&self) -> bool {
        *self == Dualcore::Lockstep
    }
    #[doc = "Second core work independently"]
    #[inline(always)]
    pub fn is_dualcore(&self) -> bool {
        *self == Dualcore::Dualcore
    }
}
#[doc = "Field `DUALCORE` writer - If set, second core work independent from first core"]
pub type DualcoreW<'a, REG> = crate::BitWriter<'a, REG, Dualcore>;
impl<'a, REG> DualcoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Second core work in lockstep mode"]
    #[inline(always)]
    pub fn lockstep(self) -> &'a mut crate::W<REG> {
        self.variant(Dualcore::Lockstep)
    }
    #[doc = "Second core work independently"]
    #[inline(always)]
    pub fn dualcore(self) -> &'a mut crate::W<REG> {
        self.variant(Dualcore::Dualcore)
    }
}
#[doc = "Field `CODE` writer - Write key 0xACCE55 to change LOCK bit value"]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - If set, second core work independent from first core"]
    #[inline(always)]
    pub fn dualcore(&self) -> DualcoreR {
        DualcoreR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set, second core work independent from first core"]
    #[inline(always)]
    pub fn dualcore(&mut self) -> DualcoreW<'_, Core1ConfSpec> {
        DualcoreW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write key 0xACCE55 to change LOCK bit value"]
    #[inline(always)]
    pub fn code(&mut self) -> CodeW<'_, Core1ConfSpec> {
        CodeW::new(self, 8)
    }
}
#[doc = "Core 1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1ConfSpec;
impl crate::RegisterSpec for Core1ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core1_conf::R`](R) reader structure"]
impl crate::Readable for Core1ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`core1_conf::W`](W) writer structure"]
impl crate::Writable for Core1ConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE1_CONF to value 0"]
impl crate::Resettable for Core1ConfSpec {}
