#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `CMD_SET_INT` reader - Set interrupt after the entire buffer has been transferred"]
pub type CmdSetIntR = crate::BitReader;
#[doc = "Field `CMD_SET_INT` writer - Set interrupt after the entire buffer has been transferred"]
pub type CmdSetIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_LAST` reader - This is a last DMA command"]
pub type CmdLastR = crate::BitReader;
#[doc = "Field `CMD_LAST` writer - This is a last DMA command"]
pub type CmdLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEXT_ADDR` reader - Address of next command"]
pub type NextAddrR = crate::FieldReader<u32>;
#[doc = "Field `NEXT_ADDR` writer - Address of next command"]
pub type NextAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Set interrupt after the entire buffer has been transferred"]
    #[inline(always)]
    pub fn cmd_set_int(&self) -> CmdSetIntR {
        CmdSetIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is a last DMA command"]
    #[inline(always)]
    pub fn cmd_last(&self) -> CmdLastR {
        CmdLastR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Address of next command"]
    #[inline(always)]
    pub fn next_addr(&self) -> NextAddrR {
        NextAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Set interrupt after the entire buffer has been transferred"]
    #[inline(always)]
    pub fn cmd_set_int(&mut self) -> CmdSetIntW<'_, ConfigSpec> {
        CmdSetIntW::new(self, 0)
    }
    #[doc = "Bit 1 - This is a last DMA command"]
    #[inline(always)]
    pub fn cmd_last(&mut self) -> CmdLastW<'_, ConfigSpec> {
        CmdLastW::new(self, 1)
    }
    #[doc = "Bits 2:31 - Address of next command"]
    #[inline(always)]
    pub fn next_addr(&mut self) -> NextAddrW<'_, ConfigSpec> {
        NextAddrW::new(self, 2)
    }
}
#[doc = "Command register 3 (config)\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
