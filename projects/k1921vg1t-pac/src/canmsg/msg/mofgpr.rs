#[doc = "Register `MOFGPR` reader"]
pub type R = crate::R<MofgprSpec>;
#[doc = "Register `MOFGPR` writer"]
pub type W = crate::W<MofgprSpec>;
#[doc = "Field `BOT` reader - FIFO pointer to the lower element"]
pub type BotR = crate::FieldReader;
#[doc = "Field `BOT` writer - FIFO pointer to the lower element"]
pub type BotW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOP` reader - FIFO pointer to the top element"]
pub type TopR = crate::FieldReader;
#[doc = "Field `TOP` writer - FIFO pointer to the top element"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CUR` reader - A pointer to the current object within the FIFO or gateway"]
pub type CurR = crate::FieldReader;
#[doc = "Field `CUR` writer - A pointer to the current object within the FIFO or gateway"]
pub type CurW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL` reader - Object pointer message"]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - Object pointer message"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FIFO pointer to the lower element"]
    #[inline(always)]
    pub fn bot(&self) -> BotR {
        BotR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - FIFO pointer to the top element"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - A pointer to the current object within the FIFO or gateway"]
    #[inline(always)]
    pub fn cur(&self) -> CurR {
        CurR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Object pointer message"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO pointer to the lower element"]
    #[inline(always)]
    pub fn bot(&mut self) -> BotW<'_, MofgprSpec> {
        BotW::new(self, 0)
    }
    #[doc = "Bits 8:15 - FIFO pointer to the top element"]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<'_, MofgprSpec> {
        TopW::new(self, 8)
    }
    #[doc = "Bits 16:23 - A pointer to the current object within the FIFO or gateway"]
    #[inline(always)]
    pub fn cur(&mut self) -> CurW<'_, MofgprSpec> {
        CurW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Object pointer message"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, MofgprSpec> {
        SelW::new(self, 24)
    }
}
#[doc = "Pointer register FIFO / gateway message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mofgpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mofgpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MofgprSpec;
impl crate::RegisterSpec for MofgprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mofgpr::R`](R) reader structure"]
impl crate::Readable for MofgprSpec {}
#[doc = "`write(|w| ..)` method takes [`mofgpr::W`](W) writer structure"]
impl crate::Writable for MofgprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOFGPR to value 0"]
impl crate::Resettable for MofgprSpec {}
