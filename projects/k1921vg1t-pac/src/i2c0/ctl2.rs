#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Field `S10ADR` reader - Upper bits of 10-bit slave address"]
pub type S10adrR = crate::FieldReader;
#[doc = "Field `S10ADR` writer - Upper bits of 10-bit slave address"]
pub type S10adrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `S10EN` reader - Bit enabled 10-bit addressing slave"]
pub type S10enR = crate::BitReader;
#[doc = "Field `S10EN` writer - Bit enabled 10-bit addressing slave"]
pub type S10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDIV` reader - Field frequency selection signal at pin SCL in HS master mode (bits \\[3:0\\])"]
pub type HsdivR = crate::FieldReader;
#[doc = "Field `HSDIV` writer - Field frequency selection signal at pin SCL in HS master mode (bits \\[3:0\\])"]
pub type HsdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Upper bits of 10-bit slave address"]
    #[inline(always)]
    pub fn s10adr(&self) -> S10adrR {
        S10adrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Bit enabled 10-bit addressing slave"]
    #[inline(always)]
    pub fn s10en(&self) -> S10enR {
        S10enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Field frequency selection signal at pin SCL in HS master mode (bits \\[3:0\\])"]
    #[inline(always)]
    pub fn hsdiv(&self) -> HsdivR {
        HsdivR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Upper bits of 10-bit slave address"]
    #[inline(always)]
    pub fn s10adr(&mut self) -> S10adrW<'_, Ctl2Spec> {
        S10adrW::new(self, 0)
    }
    #[doc = "Bit 3 - Bit enabled 10-bit addressing slave"]
    #[inline(always)]
    pub fn s10en(&mut self) -> S10enW<'_, Ctl2Spec> {
        S10enW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Field frequency selection signal at pin SCL in HS master mode (bits \\[3:0\\])"]
    #[inline(always)]
    pub fn hsdiv(&mut self) -> HsdivW<'_, Ctl2Spec> {
        HsdivW::new(self, 4)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {}
