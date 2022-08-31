#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_RDY` reader - UART receiver fifo data ready status"]
pub struct DATA_RDY_R(crate::FieldReader<bool, bool>);
impl DATA_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER_ERR` reader - UART receiver overrun error"]
pub struct OVER_ERR_R(crate::FieldReader<bool, bool>);
impl OVER_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVER_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVER_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAR_ERR` reader - UART receiver frame error"]
pub struct PAR_ERR_R(crate::FieldReader<bool, bool>);
impl PAR_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_ERR` reader - UART receiver frame error"]
pub struct FRAME_ERR_R(crate::FieldReader<bool, bool>);
impl FRAME_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREAK_ERR` reader - UART receiver break error"]
pub struct BREAK_ERR_R(crate::FieldReader<bool, bool>);
impl BREAK_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BREAK_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAK_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_EMP` reader - UART transmitter fifo empty status"]
pub struct TX_FIFO_EMP_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_EMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_EMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_EMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ALL_EMP` reader - UART transmitter all empty status"]
pub struct TX_ALL_EMP_R(crate::FieldReader<bool, bool>);
impl TX_ALL_EMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_ALL_EMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ALL_EMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_RX_FIFO` reader - indicate error in UART receiver fifo"]
pub struct ERR_RX_FIFO_R(crate::FieldReader<bool, bool>);
impl ERR_RX_FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_RX_FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_RX_FIFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - UART receiver fifo data ready status"]
    #[inline(always)]
    pub fn data_rdy(&self) -> DATA_RDY_R {
        DATA_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART receiver overrun error"]
    #[inline(always)]
    pub fn over_err(&self) -> OVER_ERR_R {
        OVER_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART receiver frame error"]
    #[inline(always)]
    pub fn par_err(&self) -> PAR_ERR_R {
        PAR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART receiver frame error"]
    #[inline(always)]
    pub fn frame_err(&self) -> FRAME_ERR_R {
        FRAME_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART receiver break error"]
    #[inline(always)]
    pub fn break_err(&self) -> BREAK_ERR_R {
        BREAK_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmitter fifo empty status"]
    #[inline(always)]
    pub fn tx_fifo_emp(&self) -> TX_FIFO_EMP_R {
        TX_FIFO_EMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART transmitter all empty status"]
    #[inline(always)]
    pub fn tx_all_emp(&self) -> TX_ALL_EMP_R {
        TX_ALL_EMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - indicate error in UART receiver fifo"]
    #[inline(always)]
    pub fn err_rx_fifo(&self) -> ERR_RX_FIFO_R {
        ERR_RX_FIFO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART2 line status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSR to value 0xc0"]
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
