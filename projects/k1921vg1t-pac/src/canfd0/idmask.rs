#[doc = "Register `IDMASK` reader"]
pub type R = crate::R<IdmaskSpec>;
#[doc = "Register `IDMASK` writer"]
pub type W = crate::W<IdmaskSpec>;
#[doc = "Field `IDEMSK` reader - Extended Identificator Mask"]
pub type IdemskR = crate::BitReader;
#[doc = "Field `IDEMSK` writer - Extended Identificator Mask"]
pub type IdemskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRMSK` reader - Remote Transmite Mask"]
pub type RtrmskR = crate::BitReader;
#[doc = "Field `RTRMSK` writer - Remote Transmite Mask"]
pub type RtrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMSK` reader - Identificator Mask"]
pub type IdmskR = crate::FieldReader<u32>;
#[doc = "Field `IDMSK` writer - Identificator Mask"]
pub type IdmskW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - Extended Identificator Mask"]
    #[inline(always)]
    pub fn idemsk(&self) -> IdemskR {
        IdemskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Remote Transmite Mask"]
    #[inline(always)]
    pub fn rtrmsk(&self) -> RtrmskR {
        RtrmskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Identificator Mask"]
    #[inline(always)]
    pub fn idmsk(&self) -> IdmskR {
        IdmskR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Extended Identificator Mask"]
    #[inline(always)]
    pub fn idemsk(&mut self) -> IdemskW<'_, IdmaskSpec> {
        IdemskW::new(self, 0)
    }
    #[doc = "Bit 2 - Remote Transmite Mask"]
    #[inline(always)]
    pub fn rtrmsk(&mut self) -> RtrmskW<'_, IdmaskSpec> {
        RtrmskW::new(self, 2)
    }
    #[doc = "Bits 3:31 - Identificator Mask"]
    #[inline(always)]
    pub fn idmsk(&mut self) -> IdmskW<'_, IdmaskSpec> {
        IdmskW::new(self, 3)
    }
}
#[doc = "CAN Identifier Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdmaskSpec;
impl crate::RegisterSpec for IdmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmask::R`](R) reader structure"]
impl crate::Readable for IdmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`idmask::W`](W) writer structure"]
impl crate::Writable for IdmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDMASK to value 0"]
impl crate::Resettable for IdmaskSpec {}
