#[doc = "Register `MOBCTRL` reader"]
pub type R = crate::R<MobctrlSpec>;
#[doc = "Register `MOBCTRL` writer"]
pub type W = crate::W<MobctrlSpec>;
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDE` reader - Identificator Extension"]
pub type IdeR = crate::BitReader;
#[doc = "Field `IDE` writer - Identificator Extension"]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPLV` reader - Reply Active"]
pub type RplvR = crate::BitReader;
#[doc = "Field `RPLV` writer - Reply Active"]
pub type RplvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONMOB` reader - MOb Configuration"]
pub type ConmobR = crate::FieldReader;
#[doc = "Field `CONMOB` writer - MOb Configuration"]
pub type ConmobW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Identificator Extension"]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reply Active"]
    #[inline(always)]
    pub fn rplv(&self) -> RplvR {
        RplvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - MOb Configuration"]
    #[inline(always)]
    pub fn conmob(&self) -> ConmobR {
        ConmobR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<'_, MobctrlSpec> {
        DlcW::new(self, 0)
    }
    #[doc = "Bit 4 - Identificator Extension"]
    #[inline(always)]
    pub fn ide(&mut self) -> IdeW<'_, MobctrlSpec> {
        IdeW::new(self, 4)
    }
    #[doc = "Bit 5 - Reply Active"]
    #[inline(always)]
    pub fn rplv(&mut self) -> RplvW<'_, MobctrlSpec> {
        RplvW::new(self, 5)
    }
    #[doc = "Bits 6:7 - MOb Configuration"]
    #[inline(always)]
    pub fn conmob(&mut self) -> ConmobW<'_, MobctrlSpec> {
        ConmobW::new(self, 6)
    }
}
#[doc = "CAN MOb Control and DLC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MobctrlSpec;
impl crate::RegisterSpec for MobctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mobctrl::R`](R) reader structure"]
impl crate::Readable for MobctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mobctrl::W`](W) writer structure"]
impl crate::Writable for MobctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOBCTRL to value 0"]
impl crate::Resettable for MobctrlSpec {}
