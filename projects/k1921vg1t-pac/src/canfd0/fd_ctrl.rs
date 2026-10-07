#[doc = "Register `FD_CTRL` reader"]
pub type R = crate::R<FdCtrlSpec>;
#[doc = "Register `FD_CTRL` writer"]
pub type W = crate::W<FdCtrlSpec>;
#[doc = "Enable CAN FD Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdmod {
    #[doc = "0: Can 2.0 Mode"]
    Can20 = 0,
    #[doc = "1: CAN FD Mode"]
    Canfd = 1,
}
impl From<Fdmod> for bool {
    #[inline(always)]
    fn from(variant: Fdmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDMOD` reader - Enable CAN FD Mode"]
pub type FdmodR = crate::BitReader<Fdmod>;
impl FdmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdmod {
        match self.bits {
            false => Fdmod::Can20,
            true => Fdmod::Canfd,
        }
    }
    #[doc = "Can 2.0 Mode"]
    #[inline(always)]
    pub fn is_can20(&self) -> bool {
        *self == Fdmod::Can20
    }
    #[doc = "CAN FD Mode"]
    #[inline(always)]
    pub fn is_canfd(&self) -> bool {
        *self == Fdmod::Canfd
    }
}
#[doc = "Field `FDMOD` writer - Enable CAN FD Mode"]
pub type FdmodW<'a, REG> = crate::BitWriter<'a, REG, Fdmod>;
impl<'a, REG> FdmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Can 2.0 Mode"]
    #[inline(always)]
    pub fn can20(self) -> &'a mut crate::W<REG> {
        self.variant(Fdmod::Can20)
    }
    #[doc = "CAN FD Mode"]
    #[inline(always)]
    pub fn canfd(self) -> &'a mut crate::W<REG> {
        self.variant(Fdmod::Canfd)
    }
}
#[doc = "CAN FD ISO Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exmod {
    #[doc = "0: In CAN FD mode create packet in non-ISO format"]
    Noniso = 0,
    #[doc = "1: In CAN FD mode create packet in ISO format"]
    Iso = 1,
}
impl From<Exmod> for bool {
    #[inline(always)]
    fn from(variant: Exmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXMOD` reader - CAN FD ISO Select"]
pub type ExmodR = crate::BitReader<Exmod>;
impl ExmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exmod {
        match self.bits {
            false => Exmod::Noniso,
            true => Exmod::Iso,
        }
    }
    #[doc = "In CAN FD mode create packet in non-ISO format"]
    #[inline(always)]
    pub fn is_noniso(&self) -> bool {
        *self == Exmod::Noniso
    }
    #[doc = "In CAN FD mode create packet in ISO format"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Exmod::Iso
    }
}
#[doc = "Field `EXMOD` writer - CAN FD ISO Select"]
pub type ExmodW<'a, REG> = crate::BitWriter<'a, REG, Exmod>;
impl<'a, REG> ExmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In CAN FD mode create packet in non-ISO format"]
    #[inline(always)]
    pub fn noniso(self) -> &'a mut crate::W<REG> {
        self.variant(Exmod::Noniso)
    }
    #[doc = "In CAN FD mode create packet in ISO format"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Exmod::Iso)
    }
}
impl R {
    #[doc = "Bit 0 - Enable CAN FD Mode"]
    #[inline(always)]
    pub fn fdmod(&self) -> FdmodR {
        FdmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN FD ISO Select"]
    #[inline(always)]
    pub fn exmod(&self) -> ExmodR {
        ExmodR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CAN FD Mode"]
    #[inline(always)]
    pub fn fdmod(&mut self) -> FdmodW<'_, FdCtrlSpec> {
        FdmodW::new(self, 0)
    }
    #[doc = "Bit 1 - CAN FD ISO Select"]
    #[inline(always)]
    pub fn exmod(&mut self) -> ExmodW<'_, FdCtrlSpec> {
        ExmodW::new(self, 1)
    }
}
#[doc = "CAN FD Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdCtrlSpec;
impl crate::RegisterSpec for FdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fd_ctrl::R`](R) reader structure"]
impl crate::Readable for FdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fd_ctrl::W`](W) writer structure"]
impl crate::Writable for FdCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FD_CTRL to value 0"]
impl crate::Resettable for FdCtrlSpec {}
