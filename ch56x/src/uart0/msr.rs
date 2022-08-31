#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTS_CHG` reader - UART0 CTS changed status, high action"]
pub struct CTS_CHG_R(crate::FieldReader<bool, bool>);
impl CTS_CHG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_CHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_CHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR_CHG` reader - UART0 DSR changed status, high action"]
pub struct DSR_CHG_R(crate::FieldReader<bool, bool>);
impl DSR_CHG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSR_CHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_CHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RI_CHG` reader - UART0 RI changed status, high action"]
pub struct RI_CHG_R(crate::FieldReader<bool, bool>);
impl RI_CHG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RI_CHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RI_CHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCD_CHG` reader - UART0 DCD changed status, high action"]
pub struct DCD_CHG_R(crate::FieldReader<bool, bool>);
impl DCD_CHG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCD_CHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCD_CHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS` reader - UART0 CTS action status"]
pub struct CTS_R(crate::FieldReader<bool, bool>);
impl CTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR` reader - UART0 DSR action status"]
pub struct DSR_R(crate::FieldReader<bool, bool>);
impl DSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RI` reader - UART0 RI action status"]
pub struct RI_R(crate::FieldReader<bool, bool>);
impl RI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCD` reader - UART0 DCD action status"]
pub struct DCD_R(crate::FieldReader<bool, bool>);
impl DCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - UART0 CTS changed status, high action"]
    #[inline(always)]
    pub fn cts_chg(&self) -> CTS_CHG_R {
        CTS_CHG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART0 DSR changed status, high action"]
    #[inline(always)]
    pub fn dsr_chg(&self) -> DSR_CHG_R {
        DSR_CHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 RI changed status, high action"]
    #[inline(always)]
    pub fn ri_chg(&self) -> RI_CHG_R {
        RI_CHG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART0 DCD changed status, high action"]
    #[inline(always)]
    pub fn dcd_chg(&self) -> DCD_CHG_R {
        DCD_CHG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 CTS action status"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 DSR action status"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART0 RI action status"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART0 DCD action status"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART0 modem status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
