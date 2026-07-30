#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `BS` reader - Buffer select"]
pub type BsR = crate::BitReader;
#[doc = "Field `B0` reader - Buffer 0 data valid"]
pub type B0R = crate::BitReader;
#[doc = "Field `B1` reader - Buffer 1 data valid"]
pub type B1R = crate::BitReader;
#[doc = "Field `B0CNT` reader - Buffer 0 byte count"]
pub type B0cntR = crate::FieldReader<u16>;
#[doc = "Field `B1CNT` reader - Buffer 1 byte count"]
pub type B1cntR = crate::FieldReader<u16>;
#[doc = "Field `PT` reader - Packet transmitted"]
pub type PtR = crate::BitReader;
#[doc = "Field `PT` writer - Packet transmitted"]
pub type PtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Buffer select"]
    #[inline(always)]
    pub fn bs(&self) -> BsR {
        BsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer 0 data valid"]
    #[inline(always)]
    pub fn b0(&self) -> B0R {
        B0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer 1 data valid"]
    #[inline(always)]
    pub fn b1(&self) -> B1R {
        B1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15 - Buffer 0 byte count"]
    #[inline(always)]
    pub fn b0cnt(&self) -> B0cntR {
        B0cntR::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Buffer 1 byte count"]
    #[inline(always)]
    pub fn b1cnt(&self) -> B1cntR {
        B1cntR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 29 - Packet transmitted"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Packet transmitted"]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<'_, StatSpec> {
        PtW::new(self, 29)
    }
}
#[doc = "IN Endpoint status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
