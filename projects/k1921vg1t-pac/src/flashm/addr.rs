#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `BYTESEL` reader - Address value for each byte"]
pub type ByteselR = crate::FieldReader;
#[doc = "Field `BYTESEL` writer - Address value for each byte"]
pub type ByteselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRESS` reader - Address value to selected flash IP"]
pub type AddressR = crate::FieldReader<u16>;
#[doc = "Field `ADDRESS` writer - Address value to selected flash IP"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BANK` reader - Address value for flash operations (Bank number)"]
pub type BankR = crate::FieldReader;
#[doc = "Field `BANK` writer - Address value for flash operations (Bank number)"]
pub type BankW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address value for each byte"]
    #[inline(always)]
    pub fn bytesel(&self) -> ByteselR {
        ByteselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - Address value to selected flash IP"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bits 20:21 - Address value for flash operations (Bank number)"]
    #[inline(always)]
    pub fn bank(&self) -> BankR {
        BankR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address value for each byte"]
    #[inline(always)]
    pub fn bytesel(&mut self) -> ByteselW<'_, AddrSpec> {
        ByteselW::new(self, 0)
    }
    #[doc = "Bits 4:19 - Address value to selected flash IP"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, AddrSpec> {
        AddressW::new(self, 4)
    }
    #[doc = "Bits 20:21 - Address value for flash operations (Bank number)"]
    #[inline(always)]
    pub fn bank(&mut self) -> BankW<'_, AddrSpec> {
        BankW::new(self, 20)
    }
}
#[doc = "Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {}
