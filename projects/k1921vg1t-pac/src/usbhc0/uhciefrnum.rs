#[doc = "Register `UHCIEFRNUM` reader"]
pub type R = crate::R<UhciefrnumSpec>;
#[doc = "Register `UHCIEFRNUM` writer"]
pub type W = crate::W<UhciefrnumSpec>;
#[doc = "Field `TIMCRCIE` reader - Timeout/CRC Interrupt Enable"]
pub type TimcrcieR = crate::BitReader;
#[doc = "Field `TIMCRCIE` writer - Timeout/CRC Interrupt Enable"]
pub type TimcrcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUMEIE` reader - Resume Interrupt Enable"]
pub type ResumeieR = crate::BitReader;
#[doc = "Field `RESUMEIE` writer - Resume Interrupt Enable"]
pub type ResumeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCE` reader - Interrupt on Complete Enable"]
pub type IoceR = crate::BitReader;
#[doc = "Field `IOCE` writer - Interrupt on Complete Enable"]
pub type IoceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIE` reader - Short Packet Interrupt Enable"]
pub type SpieR = crate::BitReader;
#[doc = "Field `SPIE` writer - Short Packet Interrupt Enable"]
pub type SpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRNUM` reader - Frame List Current Index/Frame Number"]
pub type FrnumR = crate::FieldReader<u16>;
#[doc = "Field `FRNUM` writer - Frame List Current Index/Frame Number"]
pub type FrnumW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Timeout/CRC Interrupt Enable"]
    #[inline(always)]
    pub fn timcrcie(&self) -> TimcrcieR {
        TimcrcieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resumeie(&self) -> ResumeieR {
        ResumeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on Complete Enable"]
    #[inline(always)]
    pub fn ioce(&self) -> IoceR {
        IoceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn spie(&self) -> SpieR {
        SpieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Frame List Current Index/Frame Number"]
    #[inline(always)]
    pub fn frnum(&self) -> FrnumR {
        FrnumR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout/CRC Interrupt Enable"]
    #[inline(always)]
    pub fn timcrcie(&mut self) -> TimcrcieW<'_, UhciefrnumSpec> {
        TimcrcieW::new(self, 0)
    }
    #[doc = "Bit 1 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resumeie(&mut self) -> ResumeieW<'_, UhciefrnumSpec> {
        ResumeieW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt on Complete Enable"]
    #[inline(always)]
    pub fn ioce(&mut self) -> IoceW<'_, UhciefrnumSpec> {
        IoceW::new(self, 2)
    }
    #[doc = "Bit 3 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn spie(&mut self) -> SpieW<'_, UhciefrnumSpec> {
        SpieW::new(self, 3)
    }
    #[doc = "Bits 16:26 - Frame List Current Index/Frame Number"]
    #[inline(always)]
    pub fn frnum(&mut self) -> FrnumW<'_, UhciefrnumSpec> {
        FrnumW::new(self, 16)
    }
}
#[doc = "USBHC UHC Interrupt Enable and Frame Number register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhciefrnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhciefrnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhciefrnumSpec;
impl crate::RegisterSpec for UhciefrnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhciefrnum::R`](R) reader structure"]
impl crate::Readable for UhciefrnumSpec {}
#[doc = "`write(|w| ..)` method takes [`uhciefrnum::W`](W) writer structure"]
impl crate::Writable for UhciefrnumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCIEFRNUM to value 0"]
impl crate::Resettable for UhciefrnumSpec {}
