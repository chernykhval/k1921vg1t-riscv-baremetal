#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `START` reader - Start bit"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start bit"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop bit"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop bit"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - Interrupt enable bit"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Interrupt enable bit"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNADDR` reader - Unaddress mode, slave response to any address"]
pub type UnaddrR = crate::BitReader;
#[doc = "Field `UNADDR` writer - Unaddress mode, slave response to any address"]
pub type UnaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Acknowledgment bit reception"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledgment bit reception"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCMEN` reader - Control bit part a response to the general call address"]
pub type GcmenR = crate::BitReader;
#[doc = "Field `GCMEN` writer - Control bit part a response to the general call address"]
pub type GcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBARE` reader - Control bit part a response to the response address"]
pub type SmbareR = crate::BitReader;
#[doc = "Field `SMBARE` writer - Control bit part a response to the response address"]
pub type SmbareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRST` reader - Bit reset flag interrupt"]
pub type ClrstR = crate::BitReader;
#[doc = "Field `CLRST` writer - Bit reset flag interrupt"]
pub type ClrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start bit"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop bit"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable bit"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unaddress mode, slave response to any address"]
    #[inline(always)]
    pub fn unaddr(&self) -> UnaddrR {
        UnaddrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledgment bit reception"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control bit part a response to the general call address"]
    #[inline(always)]
    pub fn gcmen(&self) -> GcmenR {
        GcmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control bit part a response to the response address"]
    #[inline(always)]
    pub fn smbare(&self) -> SmbareR {
        SmbareR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit reset flag interrupt"]
    #[inline(always)]
    pub fn clrst(&self) -> ClrstR {
        ClrstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start bit"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Ctl0Spec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop bit"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Ctl0Spec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable bit"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, Ctl0Spec> {
        IntenW::new(self, 2)
    }
    #[doc = "Bit 3 - Unaddress mode, slave response to any address"]
    #[inline(always)]
    pub fn unaddr(&mut self) -> UnaddrW<'_, Ctl0Spec> {
        UnaddrW::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledgment bit reception"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, Ctl0Spec> {
        AckW::new(self, 4)
    }
    #[doc = "Bit 5 - Control bit part a response to the general call address"]
    #[inline(always)]
    pub fn gcmen(&mut self) -> GcmenW<'_, Ctl0Spec> {
        GcmenW::new(self, 5)
    }
    #[doc = "Bit 6 - Control bit part a response to the response address"]
    #[inline(always)]
    pub fn smbare(&mut self) -> SmbareW<'_, Ctl0Spec> {
        SmbareW::new(self, 6)
    }
    #[doc = "Bit 7 - Bit reset flag interrupt"]
    #[inline(always)]
    pub fn clrst(&mut self) -> ClrstW<'_, Ctl0Spec> {
        ClrstW::new(self, 7)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
