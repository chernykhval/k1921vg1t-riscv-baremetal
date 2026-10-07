#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `ENABLE` reader - SDRAM Controller enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - SDRAM Controller enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW_BUS` reader - Must be set if System clock frequency is less than SDRAM frequency"]
pub type SlowBusR = crate::BitReader;
#[doc = "Field `SLOW_BUS` writer - Must be set if System clock frequency is less than SDRAM frequency"]
pub type SlowBusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SDRAM Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataWidth {
    #[doc = "0: 32-bit SDRAM"]
    Data32 = 0,
    #[doc = "1: 16-bit SDRAM"]
    Data16 = 1,
    #[doc = "2: 8-bit SDRAM"]
    Data8 = 2,
}
impl From<DataWidth> for u8 {
    #[inline(always)]
    fn from(variant: DataWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DataWidth {
    type Ux = u8;
}
impl crate::IsEnum for DataWidth {}
#[doc = "Field `DATA_WIDTH` reader - SDRAM Data Width"]
pub type DataWidthR = crate::FieldReader<DataWidth>;
impl DataWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataWidth {
        match self.bits {
            0 => DataWidth::Data32,
            1 => DataWidth::Data16,
            2 => DataWidth::Data8,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit SDRAM"]
    #[inline(always)]
    pub fn is_data32(&self) -> bool {
        *self == DataWidth::Data32
    }
    #[doc = "16-bit SDRAM"]
    #[inline(always)]
    pub fn is_data16(&self) -> bool {
        *self == DataWidth::Data16
    }
    #[doc = "8-bit SDRAM"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == DataWidth::Data8
    }
}
#[doc = "Field `DATA_WIDTH` writer - SDRAM Data Width"]
pub type DataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, DataWidth>;
impl<'a, REG> DataWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit SDRAM"]
    #[inline(always)]
    pub fn data32(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::Data32)
    }
    #[doc = "16-bit SDRAM"]
    #[inline(always)]
    pub fn data16(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::Data16)
    }
    #[doc = "8-bit SDRAM"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::Data8)
    }
}
#[doc = "SDRAM Column Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ColumnWidth {
    #[doc = "0: 8 bit"]
    Bit8 = 0,
    #[doc = "1: 9 bit"]
    Bit9 = 1,
    #[doc = "2: 10 bits"]
    Bit10 = 2,
    #[doc = "3: 11 bits"]
    Bit11 = 3,
}
impl From<ColumnWidth> for u8 {
    #[inline(always)]
    fn from(variant: ColumnWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ColumnWidth {
    type Ux = u8;
}
impl crate::IsEnum for ColumnWidth {}
#[doc = "Field `COLUMN_WIDTH` reader - SDRAM Column Bit"]
pub type ColumnWidthR = crate::FieldReader<ColumnWidth>;
impl ColumnWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ColumnWidth {
        match self.bits {
            0 => ColumnWidth::Bit8,
            1 => ColumnWidth::Bit9,
            2 => ColumnWidth::Bit10,
            3 => ColumnWidth::Bit11,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == ColumnWidth::Bit8
    }
    #[doc = "9 bit"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == ColumnWidth::Bit9
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == ColumnWidth::Bit10
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == ColumnWidth::Bit11
    }
}
#[doc = "Field `COLUMN_WIDTH` writer - SDRAM Column Bit"]
pub type ColumnWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, ColumnWidth, crate::Safe>;
impl<'a, REG> ColumnWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(ColumnWidth::Bit8)
    }
    #[doc = "9 bit"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut crate::W<REG> {
        self.variant(ColumnWidth::Bit9)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(ColumnWidth::Bit10)
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(ColumnWidth::Bit11)
    }
}
#[doc = "Field `REQ_DEPTH` reader - How many requests internal buffer should hold"]
pub type ReqDepthR = crate::FieldReader;
#[doc = "Field `REQ_DEPTH` writer - How many requests internal buffer should hold"]
pub type ReqDepthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - SDRAM Controller enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Must be set if System clock frequency is less than SDRAM frequency"]
    #[inline(always)]
    pub fn slow_bus(&self) -> SlowBusR {
        SlowBusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SDRAM Data Width"]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SDRAM Column Bit"]
    #[inline(always)]
    pub fn column_width(&self) -> ColumnWidthR {
        ColumnWidthR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - How many requests internal buffer should hold"]
    #[inline(always)]
    pub fn req_depth(&self) -> ReqDepthR {
        ReqDepthR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SDRAM Controller enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Must be set if System clock frequency is less than SDRAM frequency"]
    #[inline(always)]
    pub fn slow_bus(&mut self) -> SlowBusW<'_, ControlSpec> {
        SlowBusW::new(self, 1)
    }
    #[doc = "Bits 8:9 - SDRAM Data Width"]
    #[inline(always)]
    pub fn data_width(&mut self) -> DataWidthW<'_, ControlSpec> {
        DataWidthW::new(self, 8)
    }
    #[doc = "Bits 10:11 - SDRAM Column Bit"]
    #[inline(always)]
    pub fn column_width(&mut self) -> ColumnWidthW<'_, ControlSpec> {
        ColumnWidthW::new(self, 10)
    }
    #[doc = "Bits 16:17 - How many requests internal buffer should hold"]
    #[inline(always)]
    pub fn req_depth(&mut self) -> ReqDepthW<'_, ControlSpec> {
        ReqDepthW::new(self, 16)
    }
}
#[doc = "SDRAM Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0x0003_0000"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x0003_0000;
}
