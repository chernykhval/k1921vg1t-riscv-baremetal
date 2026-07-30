#[doc = "Register `EXPAND_ADDR` reader"]
pub type R = crate::R<ExpandAddrSpec>;
#[doc = "Register `EXPAND_ADDR` writer"]
pub type W = crate::W<ExpandAddrSpec>;
#[doc = "Field `BYTESEL` reader - Address value for each byte"]
pub type ByteselR = crate::FieldReader;
#[doc = "Field `BYTESEL` writer - Address value for each byte"]
pub type ByteselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDRESS` reader - Address value within current sector to selected flash IP"]
pub type AddressR = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - Address value within current sector to selected flash IP"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECTOR_ADDR` reader - Sector address value to selected flash IP"]
pub type SectorAddrR = crate::FieldReader<u16>;
#[doc = "Field `SECTOR_ADDR` writer - Sector address value to selected flash IP"]
pub type SectorAddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:2 - Address value for each byte"]
    #[inline(always)]
    pub fn bytesel(&self) -> ByteselR {
        ByteselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:9 - Address value within current sector to selected flash IP"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 10:18 - Sector address value to selected flash IP"]
    #[inline(always)]
    pub fn sector_addr(&self) -> SectorAddrR {
        SectorAddrR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address value for each byte"]
    #[inline(always)]
    pub fn bytesel(&mut self) -> ByteselW<'_, ExpandAddrSpec> {
        ByteselW::new(self, 0)
    }
    #[doc = "Bits 3:9 - Address value within current sector to selected flash IP"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, ExpandAddrSpec> {
        AddressW::new(self, 3)
    }
    #[doc = "Bits 10:18 - Sector address value to selected flash IP"]
    #[inline(always)]
    pub fn sector_addr(&mut self) -> SectorAddrW<'_, ExpandAddrSpec> {
        SectorAddrW::new(self, 10)
    }
}
#[doc = "Address Register with expanded fields\n\nYou can [`read`](crate::Reg::read) this register and get [`expand_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`expand_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpandAddrSpec;
impl crate::RegisterSpec for ExpandAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`expand_addr::R`](R) reader structure"]
impl crate::Readable for ExpandAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`expand_addr::W`](W) writer structure"]
impl crate::Writable for ExpandAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXPAND_ADDR to value 0"]
impl crate::Resettable for ExpandAddrSpec {}
