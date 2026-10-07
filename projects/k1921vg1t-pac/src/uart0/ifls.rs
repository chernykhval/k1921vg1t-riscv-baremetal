#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "Transmit interrupt FIFO level select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txiflsel {
    #[doc = "4: interrupt on 1/8"]
    Lvl18 = 4,
    #[doc = "8: interrupt on 1/4"]
    Lvl14 = 8,
    #[doc = "16: interrupt on 1/2"]
    Lvl12 = 16,
    #[doc = "24: interrupt on 3/4"]
    Lvl34 = 24,
    #[doc = "28: interrupt on 7/8"]
    Lvl78 = 28,
    #[doc = "32: interrupt on full fifo"]
    Lvl88 = 32,
}
impl From<Txiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Txiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Txiflsel {}
#[doc = "Field `TXIFLSEL` reader - Transmit interrupt FIFO level select"]
pub type TxiflselR = crate::FieldReader<Txiflsel>;
impl TxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txiflsel> {
        match self.bits {
            4 => Some(Txiflsel::Lvl18),
            8 => Some(Txiflsel::Lvl14),
            16 => Some(Txiflsel::Lvl12),
            24 => Some(Txiflsel::Lvl34),
            28 => Some(Txiflsel::Lvl78),
            32 => Some(Txiflsel::Lvl88),
            _ => None,
        }
    }
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn is_lvl18(&self) -> bool {
        *self == Txiflsel::Lvl18
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn is_lvl14(&self) -> bool {
        *self == Txiflsel::Lvl14
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn is_lvl12(&self) -> bool {
        *self == Txiflsel::Lvl12
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn is_lvl34(&self) -> bool {
        *self == Txiflsel::Lvl34
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn is_lvl78(&self) -> bool {
        *self == Txiflsel::Lvl78
    }
    #[doc = "interrupt on full fifo"]
    #[inline(always)]
    pub fn is_lvl88(&self) -> bool {
        *self == Txiflsel::Lvl88
    }
}
#[doc = "Field `TXIFLSEL` writer - Transmit interrupt FIFO level select"]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Txiflsel>;
impl<'a, REG> TxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn lvl18(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl18)
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn lvl14(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl14)
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn lvl12(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl12)
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn lvl34(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl34)
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn lvl78(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl78)
    }
    #[doc = "interrupt on full fifo"]
    #[inline(always)]
    pub fn lvl88(self) -> &'a mut crate::W<REG> {
        self.variant(Txiflsel::Lvl88)
    }
}
#[doc = "Receive interrupt FIFO level select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxiflsel {
    #[doc = "4: interrupt on 1/8"]
    Lvl18 = 4,
    #[doc = "8: interrupt on 1/4"]
    Lvl14 = 8,
    #[doc = "16: interrupt on 1/2"]
    Lvl12 = 16,
    #[doc = "24: interrupt on 3/4"]
    Lvl34 = 24,
    #[doc = "28: interrupt on 7/8"]
    Lvl78 = 28,
    #[doc = "32: interrupt on full fifo"]
    Lvl88 = 32,
}
impl From<Rxiflsel> for u8 {
    #[inline(always)]
    fn from(variant: Rxiflsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxiflsel {
    type Ux = u8;
}
impl crate::IsEnum for Rxiflsel {}
#[doc = "Field `RXIFLSEL` reader - Receive interrupt FIFO level select"]
pub type RxiflselR = crate::FieldReader<Rxiflsel>;
impl RxiflselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxiflsel> {
        match self.bits {
            4 => Some(Rxiflsel::Lvl18),
            8 => Some(Rxiflsel::Lvl14),
            16 => Some(Rxiflsel::Lvl12),
            24 => Some(Rxiflsel::Lvl34),
            28 => Some(Rxiflsel::Lvl78),
            32 => Some(Rxiflsel::Lvl88),
            _ => None,
        }
    }
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn is_lvl18(&self) -> bool {
        *self == Rxiflsel::Lvl18
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn is_lvl14(&self) -> bool {
        *self == Rxiflsel::Lvl14
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn is_lvl12(&self) -> bool {
        *self == Rxiflsel::Lvl12
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn is_lvl34(&self) -> bool {
        *self == Rxiflsel::Lvl34
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn is_lvl78(&self) -> bool {
        *self == Rxiflsel::Lvl78
    }
    #[doc = "interrupt on full fifo"]
    #[inline(always)]
    pub fn is_lvl88(&self) -> bool {
        *self == Rxiflsel::Lvl88
    }
}
#[doc = "Field `RXIFLSEL` writer - Receive interrupt FIFO level select"]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Rxiflsel>;
impl<'a, REG> RxiflselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "interrupt on 1/8"]
    #[inline(always)]
    pub fn lvl18(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl18)
    }
    #[doc = "interrupt on 1/4"]
    #[inline(always)]
    pub fn lvl14(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl14)
    }
    #[doc = "interrupt on 1/2"]
    #[inline(always)]
    pub fn lvl12(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl12)
    }
    #[doc = "interrupt on 3/4"]
    #[inline(always)]
    pub fn lvl34(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl34)
    }
    #[doc = "interrupt on 7/8"]
    #[inline(always)]
    pub fn lvl78(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl78)
    }
    #[doc = "interrupt on full fifo"]
    #[inline(always)]
    pub fn lvl88(self) -> &'a mut crate::W<REG> {
        self.variant(Rxiflsel::Lvl88)
    }
}
impl R {
    #[doc = "Bits 0:5 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Transmit interrupt FIFO level select"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TxiflselW<'_, IflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Receive interrupt FIFO level select"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RxiflselW<'_, IflsSpec> {
        RxiflselW::new(self, 6)
    }
}
#[doc = "Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFLS to value 0x0104"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0x0104;
}
