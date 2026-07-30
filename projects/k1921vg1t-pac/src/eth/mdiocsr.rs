#[doc = "Register `MDIOCSR` reader"]
pub type R = crate::R<MdiocsrSpec>;
#[doc = "Register `MDIOCSR` writer"]
pub type W = crate::W<MdiocsrSpec>;
#[doc = "Field `WR` reader - Wirte"]
pub type WrR = crate::BitReader;
#[doc = "Field `WR` writer - Wirte"]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Read"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Read"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINKFAIL` reader - Linkfail"]
pub type LinkfailR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `RADDR` reader - Register address"]
pub type RaddrR = crate::FieldReader;
#[doc = "Field `RADDR` writer - Register address"]
pub type RaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PADDR` reader - PHY address"]
pub type PaddrR = crate::FieldReader;
#[doc = "Field `PADDR` writer - PHY address"]
pub type PaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DATA` reader - MDIO Data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - MDIO Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Wirte"]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Linkfail"]
    #[inline(always)]
    pub fn linkfail(&self) -> LinkfailR {
        LinkfailR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:10 - Register address"]
    #[inline(always)]
    pub fn raddr(&self) -> RaddrR {
        RaddrR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn paddr(&self) -> PaddrR {
        PaddrR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - MDIO Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Wirte"]
    #[inline(always)]
    pub fn wr(&mut self) -> WrW<'_, MdiocsrSpec> {
        WrW::new(self, 0)
    }
    #[doc = "Bit 1 - Read"]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<'_, MdiocsrSpec> {
        RdW::new(self, 1)
    }
    #[doc = "Bits 6:10 - Register address"]
    #[inline(always)]
    pub fn raddr(&mut self) -> RaddrW<'_, MdiocsrSpec> {
        RaddrW::new(self, 6)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PaddrW<'_, MdiocsrSpec> {
        PaddrW::new(self, 11)
    }
    #[doc = "Bits 16:31 - MDIO Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, MdiocsrSpec> {
        DataW::new(self, 16)
    }
}
#[doc = "MDIO control & status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdiocsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdiocsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiocsrSpec;
impl crate::RegisterSpec for MdiocsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdiocsr::R`](R) reader structure"]
impl crate::Readable for MdiocsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdiocsr::W`](W) writer structure"]
impl crate::Writable for MdiocsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOCSR to value 0"]
impl crate::Resettable for MdiocsrSpec {}
