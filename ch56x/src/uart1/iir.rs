#[doc = "Register `IIR` reader"]
pub struct R(crate::R<IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NO_INT` reader - UART no interrupt flag"]
pub struct NO_INT_R(crate::FieldReader<bool, bool>);
impl NO_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_MASK` reader - UART interrupt flag bit mask"]
pub struct INT_MASK_R(crate::FieldReader<u8, u8>);
impl INT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_ID` reader - UART FIFO enabled flag"]
pub struct FIFO_ID_R(crate::FieldReader<u8, u8>);
impl FIFO_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - UART no interrupt flag"]
    #[inline(always)]
    pub fn no_int(&self) -> NO_INT_R {
        NO_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - UART interrupt flag bit mask"]
    #[inline(always)]
    pub fn int_mask(&self) -> INT_MASK_R {
        INT_MASK_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - UART FIFO enabled flag"]
    #[inline(always)]
    pub fn fifo_id(&self) -> FIFO_ID_R {
        FIFO_ID_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "UART1 interrupt identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir](index.html) module"]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iir::R](R) reader structure"]
impl crate::Readable for IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
