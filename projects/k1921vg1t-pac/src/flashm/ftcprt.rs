#[doc = "Register `FTCPRT` reader"]
pub type R = crate::R<FtcprtSpec>;
#[doc = "Register `FTCPRT` writer"]
pub type W = crate::W<FtcprtSpec>;
#[doc = "Field `TPGS` reader - Web to PROG2 high time value in cycles"]
pub type TpgsR = crate::FieldReader;
#[doc = "Field `TPGS` writer - Web to PROG2 high time value in cycles"]
pub type TpgsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPREPROG` reader - Pre-program time value in cycles"]
pub type TpreprogR = crate::FieldReader;
#[doc = "Field `TPREPROG` writer - Pre-program time value in cycles"]
pub type TpreprogW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPREPGS` reader - PREPG to PROG2 time value in cycles"]
pub type TprepgsR = crate::FieldReader;
#[doc = "Field `TPREPGS` writer - PREPG to PROG2 time value in cycles"]
pub type TprepgsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPROG` reader - Program time value in cycles"]
pub type TprogR = crate::FieldReader;
#[doc = "Field `TPROG` writer - Program time value in cycles"]
pub type TprogW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Web to PROG2 high time value in cycles"]
    #[inline(always)]
    pub fn tpgs(&self) -> TpgsR {
        TpgsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pre-program time value in cycles"]
    #[inline(always)]
    pub fn tpreprog(&self) -> TpreprogR {
        TpreprogR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PREPG to PROG2 time value in cycles"]
    #[inline(always)]
    pub fn tprepgs(&self) -> TprepgsR {
        TprepgsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Program time value in cycles"]
    #[inline(always)]
    pub fn tprog(&self) -> TprogR {
        TprogR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Web to PROG2 high time value in cycles"]
    #[inline(always)]
    pub fn tpgs(&mut self) -> TpgsW<'_, FtcprtSpec> {
        TpgsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Pre-program time value in cycles"]
    #[inline(always)]
    pub fn tpreprog(&mut self) -> TpreprogW<'_, FtcprtSpec> {
        TpreprogW::new(self, 8)
    }
    #[doc = "Bits 16:23 - PREPG to PROG2 time value in cycles"]
    #[inline(always)]
    pub fn tprepgs(&mut self) -> TprepgsW<'_, FtcprtSpec> {
        TprepgsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Program time value in cycles"]
    #[inline(always)]
    pub fn tprog(&mut self) -> TprogW<'_, FtcprtSpec> {
        TprogW::new(self, 24)
    }
}
#[doc = "Flash Timing Control register (Program time)\n\nYou can [`read`](crate::Reg::read) this register and get [`ftcprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftcprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtcprtSpec;
impl crate::RegisterSpec for FtcprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftcprt::R`](R) reader structure"]
impl crate::Readable for FtcprtSpec {}
#[doc = "`write(|w| ..)` method takes [`ftcprt::W`](W) writer structure"]
impl crate::Writable for FtcprtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTCPRT to value 0x4820_1030"]
impl crate::Resettable for FtcprtSpec {
    const RESET_VALUE: u32 = 0x4820_1030;
}
