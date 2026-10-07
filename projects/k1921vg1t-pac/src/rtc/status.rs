#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `A1F` reader - Alarm 1 flag"]
pub type A1fR = crate::BitReader;
#[doc = "Field `A1F` writer - Alarm 1 flag"]
pub type A1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A2F` reader - Alarm 2 flag"]
pub type A2fR = crate::BitReader;
#[doc = "Field `A2F` writer - Alarm 2 flag"]
pub type A2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF` reader - Power switched to battery flag"]
pub type BfR = crate::BitReader;
#[doc = "Field `BF` writer - Power switched to battery flag"]
pub type BfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPF` reader - Switch to normal power supply was detected"]
pub type NpfR = crate::BitReader;
#[doc = "Field `NPF` writer - Switch to normal power supply was detected"]
pub type NpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` reader - Timer overflow was detected"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - Timer overflow was detected"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCOK` reader - RC oscillator state is ok"]
pub type OscokR = crate::BitReader;
#[doc = "Field `OSCOK` writer - RC oscillator state is ok"]
pub type OscokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BU` reader - Battery supply is used status"]
pub type BuR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm 1 flag"]
    #[inline(always)]
    pub fn a1f(&self) -> A1fR {
        A1fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm 2 flag"]
    #[inline(always)]
    pub fn a2f(&self) -> A2fR {
        A2fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power switched to battery flag"]
    #[inline(always)]
    pub fn bf(&self) -> BfR {
        BfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Switch to normal power supply was detected"]
    #[inline(always)]
    pub fn npf(&self) -> NpfR {
        NpfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer overflow was detected"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - RC oscillator state is ok"]
    #[inline(always)]
    pub fn oscok(&self) -> OscokR {
        OscokR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - Battery supply is used status"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 1 flag"]
    #[inline(always)]
    pub fn a1f(&mut self) -> A1fW<'_, StatusSpec> {
        A1fW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm 2 flag"]
    #[inline(always)]
    pub fn a2f(&mut self) -> A2fW<'_, StatusSpec> {
        A2fW::new(self, 1)
    }
    #[doc = "Bit 2 - Power switched to battery flag"]
    #[inline(always)]
    pub fn bf(&mut self) -> BfW<'_, StatusSpec> {
        BfW::new(self, 2)
    }
    #[doc = "Bit 3 - Switch to normal power supply was detected"]
    #[inline(always)]
    pub fn npf(&mut self) -> NpfW<'_, StatusSpec> {
        NpfW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer overflow was detected"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, StatusSpec> {
        OfW::new(self, 4)
    }
    #[doc = "Bit 8 - RC oscillator state is ok"]
    #[inline(always)]
    pub fn oscok(&mut self) -> OscokW<'_, StatusSpec> {
        OscokW::new(self, 8)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
