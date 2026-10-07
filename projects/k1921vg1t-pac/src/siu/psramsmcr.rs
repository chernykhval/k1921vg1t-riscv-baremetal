#[doc = "Register `PSRAMSMCR` reader"]
pub type R = crate::R<PsramsmcrSpec>;
#[doc = "Register `PSRAMSMCR` writer"]
pub type W = crate::W<PsramsmcrSpec>;
#[doc = "Field `CANFD_RMEN` reader - Enable to use Read-Write Margin on CANFD RAM"]
pub type CanfdRmenR = crate::BitReader;
#[doc = "Field `CANFD_RMEN` writer - Enable to use Read-Write Margin on CANFD RAM"]
pub type CanfdRmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANFD_RM` reader - Read-Write CANFD RAM Margin value"]
pub type CanfdRmR = crate::FieldReader;
#[doc = "Field `CANFD_RM` writer - Read-Write CANFD RAM Margin value"]
pub type CanfdRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CANFD_LS` reader - Light Sleep CANFD RAM"]
pub type CanfdLsR = crate::BitReader;
#[doc = "Field `CANFD_LS` writer - Light Sleep CANFD RAM"]
pub type CanfdLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN_RMEN` reader - Enable to use Read-Write Margin on CAN RAM"]
pub type CanRmenR = crate::BitReader;
#[doc = "Field `CAN_RMEN` writer - Enable to use Read-Write Margin on CAN RAM"]
pub type CanRmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN_RM` reader - Read-Write CAN RAM Margin value"]
pub type CanRmR = crate::FieldReader;
#[doc = "Field `CAN_RM` writer - Read-Write CAN RAM Margin value"]
pub type CanRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAN_LS` reader - Light Sleep CAN RAM"]
pub type CanLsR = crate::BitReader;
#[doc = "Field `CAN_LS` writer - Light Sleep CAN RAM"]
pub type CanLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH_RMEN` reader - Enable to use Read-Write Margin on Ethernet RAM"]
pub type EthRmenR = crate::BitReader;
#[doc = "Field `ETH_RMEN` writer - Enable to use Read-Write Margin on Ethernet RAM"]
pub type EthRmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH_RM` reader - Read-Write Ethernet RAM Margin value"]
pub type EthRmR = crate::FieldReader;
#[doc = "Field `ETH_RM` writer - Read-Write Ethernet RAM Margin value"]
pub type EthRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETH_LS` reader - Light Sleep Ethernet RAM"]
pub type EthLsR = crate::BitReader;
#[doc = "Field `ETH_LS` writer - Light Sleep Ethernet RAM"]
pub type EthLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RMEN` reader - Enable to use Read-Write Margin on ADC RAM"]
pub type AdcRmenR = crate::BitReader;
#[doc = "Field `ADC_RMEN` writer - Enable to use Read-Write Margin on ADC RAM"]
pub type AdcRmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RM` reader - Read-Write ADC RAM Margin value"]
pub type AdcRmR = crate::FieldReader;
#[doc = "Field `ADC_RM` writer - Read-Write ADC RAM Margin value"]
pub type AdcRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_LS` reader - Light Sleep ADC RAM"]
pub type AdcLsR = crate::BitReader;
#[doc = "Field `ADC_LS` writer - Light Sleep ADC RAM"]
pub type AdcLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0DC_RMEN` reader - Enable to use Read-Write Margin on USB0 Device RAM"]
pub type Usb0dcRmenR = crate::BitReader;
#[doc = "Field `USB0DC_RMEN` writer - Enable to use Read-Write Margin on USB0 Device RAM"]
pub type Usb0dcRmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0DC_RM` reader - Read-Write USB0 Device RAM Margin value"]
pub type Usb0dcRmR = crate::FieldReader;
#[doc = "Field `USB0DC_RM` writer - Read-Write USB0 Device RAM Margin value"]
pub type Usb0dcRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB0DC_LS` reader - Light Sleep USB0 Device RAM"]
pub type Usb0dcLsR = crate::BitReader;
#[doc = "Field `USB0DC_LS` writer - Light Sleep USB0 Device RAM"]
pub type Usb0dcLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0HC_RMEN` reader - Enable to use Read-Write Margin on USB0 Host RAM"]
pub type Usb0hcRmenR = crate::BitReader;
#[doc = "Field `USB0HC_RMEN` writer - Enable to use Read-Write Margin on USB0 Host RAM"]
pub type Usb0hcRmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0HC_RM` reader - Read-Write USB0 Host RAM Margin value"]
pub type Usb0hcRmR = crate::FieldReader;
#[doc = "Field `USB0HC_RM` writer - Read-Write USB0 Host RAM Margin value"]
pub type Usb0hcRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB0HC_LS` reader - Light Sleep USB0 Host RAM"]
pub type Usb0hcLsR = crate::BitReader;
#[doc = "Field `USB0HC_LS` writer - Light Sleep USB0 Host RAM"]
pub type Usb0hcLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1DC_RMEN` reader - Enable to use Read-Write Margin on USB1 Device RAM (currently unsupported)"]
pub type Usb1dcRmenR = crate::BitReader;
#[doc = "Field `USB1DC_RMEN` writer - Enable to use Read-Write Margin on USB1 Device RAM (currently unsupported)"]
pub type Usb1dcRmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1DC_RM` reader - Read-Write USB1 Device RAM Margin value (currently unsupported)"]
pub type Usb1dcRmR = crate::FieldReader;
#[doc = "Field `USB1DC_RM` writer - Read-Write USB1 Device RAM Margin value (currently unsupported)"]
pub type Usb1dcRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB1DC_LS` reader - Light Sleep USB1 Device RAM (currently unsupported)"]
pub type Usb1dcLsR = crate::BitReader;
#[doc = "Field `USB1DC_LS` writer - Light Sleep USB1 Device RAM (currently unsupported)"]
pub type Usb1dcLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1HC_RMEN` reader - Enable to use Read-Write Margin on USB1 Host RAM (currently unsupported)"]
pub type Usb1hcRmenR = crate::BitReader;
#[doc = "Field `USB1HC_RMEN` writer - Enable to use Read-Write Margin on USB1 Host RAM (currently unsupported)"]
pub type Usb1hcRmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1HC_RM` reader - Read-Write USB1 Host RAM Margin value (currently unsupported)"]
pub type Usb1hcRmR = crate::FieldReader;
#[doc = "Field `USB1HC_RM` writer - Read-Write USB1 Host RAM Margin value (currently unsupported)"]
pub type Usb1hcRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB1HC_LS` reader - Light Sleep USB1 Host RAM (currently unsupported)"]
pub type Usb1hcLsR = crate::BitReader;
#[doc = "Field `USB1HC_LS` writer - Light Sleep USB1 Host RAM (currently unsupported)"]
pub type Usb1hcLsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable to use Read-Write Margin on CANFD RAM"]
    #[inline(always)]
    pub fn canfd_rmen(&self) -> CanfdRmenR {
        CanfdRmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Read-Write CANFD RAM Margin value"]
    #[inline(always)]
    pub fn canfd_rm(&self) -> CanfdRmR {
        CanfdRmR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Light Sleep CANFD RAM"]
    #[inline(always)]
    pub fn canfd_ls(&self) -> CanfdLsR {
        CanfdLsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable to use Read-Write Margin on CAN RAM"]
    #[inline(always)]
    pub fn can_rmen(&self) -> CanRmenR {
        CanRmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Read-Write CAN RAM Margin value"]
    #[inline(always)]
    pub fn can_rm(&self) -> CanRmR {
        CanRmR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Light Sleep CAN RAM"]
    #[inline(always)]
    pub fn can_ls(&self) -> CanLsR {
        CanLsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable to use Read-Write Margin on Ethernet RAM"]
    #[inline(always)]
    pub fn eth_rmen(&self) -> EthRmenR {
        EthRmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Read-Write Ethernet RAM Margin value"]
    #[inline(always)]
    pub fn eth_rm(&self) -> EthRmR {
        EthRmR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Light Sleep Ethernet RAM"]
    #[inline(always)]
    pub fn eth_ls(&self) -> EthLsR {
        EthLsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable to use Read-Write Margin on ADC RAM"]
    #[inline(always)]
    pub fn adc_rmen(&self) -> AdcRmenR {
        AdcRmenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Read-Write ADC RAM Margin value"]
    #[inline(always)]
    pub fn adc_rm(&self) -> AdcRmR {
        AdcRmR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Light Sleep ADC RAM"]
    #[inline(always)]
    pub fn adc_ls(&self) -> AdcLsR {
        AdcLsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable to use Read-Write Margin on USB0 Device RAM"]
    #[inline(always)]
    pub fn usb0dc_rmen(&self) -> Usb0dcRmenR {
        Usb0dcRmenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Read-Write USB0 Device RAM Margin value"]
    #[inline(always)]
    pub fn usb0dc_rm(&self) -> Usb0dcRmR {
        Usb0dcRmR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Light Sleep USB0 Device RAM"]
    #[inline(always)]
    pub fn usb0dc_ls(&self) -> Usb0dcLsR {
        Usb0dcLsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable to use Read-Write Margin on USB0 Host RAM"]
    #[inline(always)]
    pub fn usb0hc_rmen(&self) -> Usb0hcRmenR {
        Usb0hcRmenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Read-Write USB0 Host RAM Margin value"]
    #[inline(always)]
    pub fn usb0hc_rm(&self) -> Usb0hcRmR {
        Usb0hcRmR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Light Sleep USB0 Host RAM"]
    #[inline(always)]
    pub fn usb0hc_ls(&self) -> Usb0hcLsR {
        Usb0hcLsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable to use Read-Write Margin on USB1 Device RAM (currently unsupported)"]
    #[inline(always)]
    pub fn usb1dc_rmen(&self) -> Usb1dcRmenR {
        Usb1dcRmenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Read-Write USB1 Device RAM Margin value (currently unsupported)"]
    #[inline(always)]
    pub fn usb1dc_rm(&self) -> Usb1dcRmR {
        Usb1dcRmR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Light Sleep USB1 Device RAM (currently unsupported)"]
    #[inline(always)]
    pub fn usb1dc_ls(&self) -> Usb1dcLsR {
        Usb1dcLsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable to use Read-Write Margin on USB1 Host RAM (currently unsupported)"]
    #[inline(always)]
    pub fn usb1hc_rmen(&self) -> Usb1hcRmenR {
        Usb1hcRmenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Read-Write USB1 Host RAM Margin value (currently unsupported)"]
    #[inline(always)]
    pub fn usb1hc_rm(&self) -> Usb1hcRmR {
        Usb1hcRmR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Light Sleep USB1 Host RAM (currently unsupported)"]
    #[inline(always)]
    pub fn usb1hc_ls(&self) -> Usb1hcLsR {
        Usb1hcLsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable to use Read-Write Margin on CANFD RAM"]
    #[inline(always)]
    pub fn canfd_rmen(&mut self) -> CanfdRmenW<'_, PsramsmcrSpec> {
        CanfdRmenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Read-Write CANFD RAM Margin value"]
    #[inline(always)]
    pub fn canfd_rm(&mut self) -> CanfdRmW<'_, PsramsmcrSpec> {
        CanfdRmW::new(self, 1)
    }
    #[doc = "Bit 3 - Light Sleep CANFD RAM"]
    #[inline(always)]
    pub fn canfd_ls(&mut self) -> CanfdLsW<'_, PsramsmcrSpec> {
        CanfdLsW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable to use Read-Write Margin on CAN RAM"]
    #[inline(always)]
    pub fn can_rmen(&mut self) -> CanRmenW<'_, PsramsmcrSpec> {
        CanRmenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Read-Write CAN RAM Margin value"]
    #[inline(always)]
    pub fn can_rm(&mut self) -> CanRmW<'_, PsramsmcrSpec> {
        CanRmW::new(self, 5)
    }
    #[doc = "Bit 7 - Light Sleep CAN RAM"]
    #[inline(always)]
    pub fn can_ls(&mut self) -> CanLsW<'_, PsramsmcrSpec> {
        CanLsW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable to use Read-Write Margin on Ethernet RAM"]
    #[inline(always)]
    pub fn eth_rmen(&mut self) -> EthRmenW<'_, PsramsmcrSpec> {
        EthRmenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Read-Write Ethernet RAM Margin value"]
    #[inline(always)]
    pub fn eth_rm(&mut self) -> EthRmW<'_, PsramsmcrSpec> {
        EthRmW::new(self, 9)
    }
    #[doc = "Bit 11 - Light Sleep Ethernet RAM"]
    #[inline(always)]
    pub fn eth_ls(&mut self) -> EthLsW<'_, PsramsmcrSpec> {
        EthLsW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable to use Read-Write Margin on ADC RAM"]
    #[inline(always)]
    pub fn adc_rmen(&mut self) -> AdcRmenW<'_, PsramsmcrSpec> {
        AdcRmenW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Read-Write ADC RAM Margin value"]
    #[inline(always)]
    pub fn adc_rm(&mut self) -> AdcRmW<'_, PsramsmcrSpec> {
        AdcRmW::new(self, 13)
    }
    #[doc = "Bit 15 - Light Sleep ADC RAM"]
    #[inline(always)]
    pub fn adc_ls(&mut self) -> AdcLsW<'_, PsramsmcrSpec> {
        AdcLsW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable to use Read-Write Margin on USB0 Device RAM"]
    #[inline(always)]
    pub fn usb0dc_rmen(&mut self) -> Usb0dcRmenW<'_, PsramsmcrSpec> {
        Usb0dcRmenW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Read-Write USB0 Device RAM Margin value"]
    #[inline(always)]
    pub fn usb0dc_rm(&mut self) -> Usb0dcRmW<'_, PsramsmcrSpec> {
        Usb0dcRmW::new(self, 17)
    }
    #[doc = "Bit 19 - Light Sleep USB0 Device RAM"]
    #[inline(always)]
    pub fn usb0dc_ls(&mut self) -> Usb0dcLsW<'_, PsramsmcrSpec> {
        Usb0dcLsW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable to use Read-Write Margin on USB0 Host RAM"]
    #[inline(always)]
    pub fn usb0hc_rmen(&mut self) -> Usb0hcRmenW<'_, PsramsmcrSpec> {
        Usb0hcRmenW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Read-Write USB0 Host RAM Margin value"]
    #[inline(always)]
    pub fn usb0hc_rm(&mut self) -> Usb0hcRmW<'_, PsramsmcrSpec> {
        Usb0hcRmW::new(self, 21)
    }
    #[doc = "Bit 23 - Light Sleep USB0 Host RAM"]
    #[inline(always)]
    pub fn usb0hc_ls(&mut self) -> Usb0hcLsW<'_, PsramsmcrSpec> {
        Usb0hcLsW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable to use Read-Write Margin on USB1 Device RAM (currently unsupported)"]
    #[inline(always)]
    pub fn usb1dc_rmen(&mut self) -> Usb1dcRmenW<'_, PsramsmcrSpec> {
        Usb1dcRmenW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Read-Write USB1 Device RAM Margin value (currently unsupported)"]
    #[inline(always)]
    pub fn usb1dc_rm(&mut self) -> Usb1dcRmW<'_, PsramsmcrSpec> {
        Usb1dcRmW::new(self, 25)
    }
    #[doc = "Bit 27 - Light Sleep USB1 Device RAM (currently unsupported)"]
    #[inline(always)]
    pub fn usb1dc_ls(&mut self) -> Usb1dcLsW<'_, PsramsmcrSpec> {
        Usb1dcLsW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable to use Read-Write Margin on USB1 Host RAM (currently unsupported)"]
    #[inline(always)]
    pub fn usb1hc_rmen(&mut self) -> Usb1hcRmenW<'_, PsramsmcrSpec> {
        Usb1hcRmenW::new(self, 28)
    }
    #[doc = "Bits 29:30 - Read-Write USB1 Host RAM Margin value (currently unsupported)"]
    #[inline(always)]
    pub fn usb1hc_rm(&mut self) -> Usb1hcRmW<'_, PsramsmcrSpec> {
        Usb1hcRmW::new(self, 29)
    }
    #[doc = "Bit 31 - Light Sleep USB1 Host RAM (currently unsupported)"]
    #[inline(always)]
    pub fn usb1hc_ls(&mut self) -> Usb1hcLsW<'_, PsramsmcrSpec> {
        Usb1hcLsW::new(self, 31)
    }
}
#[doc = "Periperals SRAM sleep and margin control register\n\nYou can [`read`](crate::Reg::read) this register and get [`psramsmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psramsmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramsmcrSpec;
impl crate::RegisterSpec for PsramsmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psramsmcr::R`](R) reader structure"]
impl crate::Readable for PsramsmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`psramsmcr::W`](W) writer structure"]
impl crate::Writable for PsramsmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAMSMCR to value 0"]
impl crate::Resettable for PsramsmcrSpec {}
