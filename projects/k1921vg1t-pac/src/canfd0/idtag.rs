#[doc = "Register `IDTAG` reader"]
pub type R = crate::R<IdtagSpec>;
#[doc = "Register `IDTAG` writer"]
pub type W = crate::W<IdtagSpec>;
#[doc = "Field `RBTAG` reader - RB Tag of Remote Device"]
pub type RbtagR = crate::FieldReader;
#[doc = "Field `RBTAG` writer - RB Tag of Remote Device"]
pub type RbtagW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTRTAG` reader - Remote Transmit Tag"]
pub type RtrtagR = crate::BitReader;
#[doc = "Field `RTRTAG` writer - Remote Transmit Tag"]
pub type RtrtagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDT` reader - Identification Tag"]
pub type IdtR = crate::FieldReader<u32>;
#[doc = "Field `IDT` writer - Identification Tag"]
pub type IdtW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:1 - RB Tag of Remote Device"]
    #[inline(always)]
    pub fn rbtag(&self) -> RbtagR {
        RbtagR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Remote Transmit Tag"]
    #[inline(always)]
    pub fn rtrtag(&self) -> RtrtagR {
        RtrtagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Identification Tag"]
    #[inline(always)]
    pub fn idt(&self) -> IdtR {
        IdtR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - RB Tag of Remote Device"]
    #[inline(always)]
    pub fn rbtag(&mut self) -> RbtagW<'_, IdtagSpec> {
        RbtagW::new(self, 0)
    }
    #[doc = "Bit 2 - Remote Transmit Tag"]
    #[inline(always)]
    pub fn rtrtag(&mut self) -> RtrtagW<'_, IdtagSpec> {
        RtrtagW::new(self, 2)
    }
    #[doc = "Bits 3:31 - Identification Tag"]
    #[inline(always)]
    pub fn idt(&mut self) -> IdtW<'_, IdtagSpec> {
        IdtW::new(self, 3)
    }
}
#[doc = "CAN Identifier Tag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idtag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idtag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdtagSpec;
impl crate::RegisterSpec for IdtagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idtag::R`](R) reader structure"]
impl crate::Readable for IdtagSpec {}
#[doc = "`write(|w| ..)` method takes [`idtag::W`](W) writer structure"]
impl crate::Writable for IdtagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDTAG to value 0"]
impl crate::Resettable for IdtagSpec {}
