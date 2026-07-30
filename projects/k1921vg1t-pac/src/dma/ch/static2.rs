#[doc = "Register `STATIC2` reader"]
pub type R = crate::R<Static2Spec>;
#[doc = "Register `STATIC2` writer"]
pub type W = crate::W<Static2Spec>;
#[doc = "Field `FRAME_WIDTH` reader - Frame width for block mode - Not Implemented"]
pub type FrameWidthR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK` reader - Work in block mode - Not Implemented"]
pub type BlockR = crate::BitReader;
#[doc = "Field `JOINT` reader - Channel works in joint mode"]
pub type JointR = crate::BitReader;
#[doc = "Field `JOINT` writer - Channel works in joint mode"]
pub type JointW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY` reader - If set the channel will automatically retry transfer if overflow or underflow occurs - Not implemented"]
pub type RetryR = crate::BitReader;
#[doc = "Endiannes byte swapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EndSwap {
    #[doc = "0: No swapping"]
    NoSwap = 0,
    #[doc = "1: Swap bytes within 16 bits"]
    SwapWithin16bits = 1,
    #[doc = "2: Swap bytes within 32 bits"]
    SwapWithin32bits = 2,
}
impl From<EndSwap> for u8 {
    #[inline(always)]
    fn from(variant: EndSwap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EndSwap {
    type Ux = u8;
}
impl crate::IsEnum for EndSwap {}
#[doc = "Field `END_SWAP` reader - Endiannes byte swapping"]
pub type EndSwapR = crate::FieldReader<EndSwap>;
impl EndSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndSwap {
        match self.bits {
            0 => EndSwap::NoSwap,
            1 => EndSwap::SwapWithin16bits,
            2 => EndSwap::SwapWithin32bits,
            _ => unreachable!(),
        }
    }
    #[doc = "No swapping"]
    #[inline(always)]
    pub fn is_no_swap(&self) -> bool {
        *self == EndSwap::NoSwap
    }
    #[doc = "Swap bytes within 16 bits"]
    #[inline(always)]
    pub fn is_swap_within16bits(&self) -> bool {
        *self == EndSwap::SwapWithin16bits
    }
    #[doc = "Swap bytes within 32 bits"]
    #[inline(always)]
    pub fn is_swap_within32bits(&self) -> bool {
        *self == EndSwap::SwapWithin32bits
    }
}
#[doc = "Field `END_SWAP` writer - Endiannes byte swapping"]
pub type EndSwapW<'a, REG> = crate::FieldWriter<'a, REG, 2, EndSwap>;
impl<'a, REG> EndSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No swapping"]
    #[inline(always)]
    pub fn no_swap(self) -> &'a mut crate::W<REG> {
        self.variant(EndSwap::NoSwap)
    }
    #[doc = "Swap bytes within 16 bits"]
    #[inline(always)]
    pub fn swap_within16bits(self) -> &'a mut crate::W<REG> {
        self.variant(EndSwap::SwapWithin16bits)
    }
    #[doc = "Swap bytes within 32 bits"]
    #[inline(always)]
    pub fn swap_within32bits(self) -> &'a mut crate::W<REG> {
        self.variant(EndSwap::SwapWithin32bits)
    }
}
impl R {
    #[doc = "Bits 0:11 - Frame width for block mode - Not Implemented"]
    #[inline(always)]
    pub fn frame_width(&self) -> FrameWidthR {
        FrameWidthR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - Work in block mode - Not Implemented"]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel works in joint mode"]
    #[inline(always)]
    pub fn joint(&self) -> JointR {
        JointR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If set the channel will automatically retry transfer if overflow or underflow occurs - Not implemented"]
    #[inline(always)]
    pub fn retry(&self) -> RetryR {
        RetryR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Endiannes byte swapping"]
    #[inline(always)]
    pub fn end_swap(&self) -> EndSwapR {
        EndSwapR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Channel works in joint mode"]
    #[inline(always)]
    pub fn joint(&mut self) -> JointW<'_, Static2Spec> {
        JointW::new(self, 16)
    }
    #[doc = "Bits 28:29 - Endiannes byte swapping"]
    #[inline(always)]
    pub fn end_swap(&mut self) -> EndSwapW<'_, Static2Spec> {
        EndSwapW::new(self, 28)
    }
}
#[doc = "Static register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`static2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Static2Spec;
impl crate::RegisterSpec for Static2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`static2::R`](R) reader structure"]
impl crate::Readable for Static2Spec {}
#[doc = "`write(|w| ..)` method takes [`static2::W`](W) writer structure"]
impl crate::Writable for Static2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATIC2 to value 0"]
impl crate::Resettable for Static2Spec {}
