#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Emulation mode bits - select the behavior of the time-base counter during emulation events\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freesoft {
    #[doc = "0: stop timer at next CLK tact"]
    StopAtClk = 0,
    #[doc = "1: stop timer when overload"]
    StopAtOverload = 1,
    #[doc = "2: endless count"]
    NoStop = 2,
}
impl From<Freesoft> for u8 {
    #[inline(always)]
    fn from(variant: Freesoft) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freesoft {
    type Ux = u8;
}
impl crate::IsEnum for Freesoft {}
#[doc = "Field `FREESOFT` reader - Emulation mode bits - select the behavior of the time-base counter during emulation events"]
pub type FreesoftR = crate::FieldReader<Freesoft>;
impl FreesoftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freesoft> {
        match self.bits {
            0 => Some(Freesoft::StopAtClk),
            1 => Some(Freesoft::StopAtOverload),
            2 => Some(Freesoft::NoStop),
            _ => None,
        }
    }
    #[doc = "stop timer at next CLK tact"]
    #[inline(always)]
    pub fn is_stop_at_clk(&self) -> bool {
        *self == Freesoft::StopAtClk
    }
    #[doc = "stop timer when overload"]
    #[inline(always)]
    pub fn is_stop_at_overload(&self) -> bool {
        *self == Freesoft::StopAtOverload
    }
    #[doc = "endless count"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == Freesoft::NoStop
    }
}
#[doc = "Field `FREESOFT` writer - Emulation mode bits - select the behavior of the time-base counter during emulation events"]
pub type FreesoftW<'a, REG> = crate::FieldWriter<'a, REG, 2, Freesoft>;
impl<'a, REG> FreesoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "stop timer at next CLK tact"]
    #[inline(always)]
    pub fn stop_at_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::StopAtClk)
    }
    #[doc = "stop timer when overload"]
    #[inline(always)]
    pub fn stop_at_overload(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::StopAtOverload)
    }
    #[doc = "endless count"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Freesoft::NoStop)
    }
}
#[doc = "Field `CLR` writer - Timer Clear"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Count mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Timer is Stopped"]
    Stop = 0,
    #[doc = "1: Timer count Up"]
    Up = 1,
    #[doc = "2: Timer count is multiple Up"]
    Multiple = 2,
    #[doc = "3: Timer count is multiple Up - Down"]
    UpDown = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Count mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Stop,
            1 => Mode::Up,
            2 => Mode::Multiple,
            3 => Mode::UpDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer is Stopped"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Mode::Stop
    }
    #[doc = "Timer count Up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Mode::Up
    }
    #[doc = "Timer count is multiple Up"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == Mode::Multiple
    }
    #[doc = "Timer count is multiple Up - Down"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == Mode::UpDown
    }
}
#[doc = "Field `MODE` writer - Count mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer is Stopped"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Stop)
    }
    #[doc = "Timer count Up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Up)
    }
    #[doc = "Timer count is multiple Up"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Multiple)
    }
    #[doc = "Timer count is multiple Up - Down"]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UpDown)
    }
}
#[doc = "Field `DMATXSTOP` reader - Stop Timer when TX DMA channel ends all transactions"]
pub type DmatxstopR = crate::BitReader;
#[doc = "Field `DMATXSTOP` writer - Stop Timer when TX DMA channel ends all transactions"]
pub type DmatxstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARXSTOP` reader - Stop Timer when RX DMA channel ends all transactions"]
pub type DmarxstopR = crate::BitReader;
#[doc = "Field `DMARXSTOP` writer - Stop Timer when RX DMA channel ends all transactions"]
pub type DmarxstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksel {
    #[doc = "0: Select system Clk"]
    SysClk = 0,
    #[doc = "1: Select External Clk"]
    ExtClk = 1,
}
impl From<Clksel> for bool {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSEL` reader - Clock Source Select"]
pub type ClkselR = crate::BitReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            false => Clksel::SysClk,
            true => Clksel::ExtClk,
        }
    }
    #[doc = "Select system Clk"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == Clksel::SysClk
    }
    #[doc = "Select External Clk"]
    #[inline(always)]
    pub fn is_ext_clk(&self) -> bool {
        *self == Clksel::ExtClk
    }
}
#[doc = "Field `CLKSEL` writer - Clock Source Select"]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select system Clk"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::SysClk)
    }
    #[doc = "Select External Clk"]
    #[inline(always)]
    pub fn ext_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::ExtClk)
    }
}
#[doc = "Field `OUTEN` reader - CAPCOM3 block output enable"]
pub type OutenR = crate::FieldReader;
#[doc = "Field `OUTEN` writer - CAPCOM3 block output enable"]
pub type OutenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Emulation mode bits - select the behavior of the time-base counter during emulation events"]
    #[inline(always)]
    pub fn freesoft(&self) -> FreesoftR {
        FreesoftR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Count mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Stop Timer when TX DMA channel ends all transactions"]
    #[inline(always)]
    pub fn dmatxstop(&self) -> DmatxstopR {
        DmatxstopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stop Timer when RX DMA channel ends all transactions"]
    #[inline(always)]
    pub fn dmarxstop(&self) -> DmarxstopR {
        DmarxstopR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - CAPCOM3 block output enable"]
    #[inline(always)]
    pub fn outen(&self) -> OutenR {
        OutenR::new(((self.bits >> 9) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Emulation mode bits - select the behavior of the time-base counter during emulation events"]
    #[inline(always)]
    pub fn freesoft(&mut self) -> FreesoftW<'_, CtrlSpec> {
        FreesoftW::new(self, 0)
    }
    #[doc = "Bit 2 - Timer Clear"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, CtrlSpec> {
        ClrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Count mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlSpec> {
        ModeW::new(self, 4)
    }
    #[doc = "Bit 6 - Stop Timer when TX DMA channel ends all transactions"]
    #[inline(always)]
    pub fn dmatxstop(&mut self) -> DmatxstopW<'_, CtrlSpec> {
        DmatxstopW::new(self, 6)
    }
    #[doc = "Bit 7 - Stop Timer when RX DMA channel ends all transactions"]
    #[inline(always)]
    pub fn dmarxstop(&mut self) -> DmarxstopW<'_, CtrlSpec> {
        DmarxstopW::new(self, 7)
    }
    #[doc = "Bit 8 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, CtrlSpec> {
        ClkselW::new(self, 8)
    }
    #[doc = "Bits 9:12 - CAPCOM3 block output enable"]
    #[inline(always)]
    pub fn outen(&mut self) -> OutenW<'_, CtrlSpec> {
        OutenW::new(self, 9)
    }
}
#[doc = "Timer control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
