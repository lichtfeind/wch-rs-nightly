#[doc = "Register `HSPI_RTX_STATUS` reader"]
pub struct R(crate::R<HSPI_RTX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_RTX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_RTX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_RTX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_RTX_STATUS` writer"]
pub struct W(crate::W<HSPI_RTX_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_RTX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HSPI_RTX_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_RTX_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_CRC_ERR` reader - CRC error occur"]
pub struct RB_HSPI_CRC_ERR_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_CRC_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_CRC_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_CRC_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_CRC_ERR` writer - CRC error occur"]
pub struct RB_HSPI_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_CRC_ERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u8 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RB_HSPI_NUM_MIS` reader - rx and tx sequence number mismatch"]
pub struct RB_HSPI_NUM_MIS_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_NUM_MIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_NUM_MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_NUM_MIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_NUM_MIS` writer - rx and tx sequence number mismatch"]
pub struct RB_HSPI_NUM_MIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_NUM_MIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u8 & 1) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - CRC error occur"]
    #[inline(always)]
    pub fn rb_hspi_crc_err(&self) -> RB_HSPI_CRC_ERR_R {
        RB_HSPI_CRC_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx and tx sequence number mismatch"]
    #[inline(always)]
    pub fn rb_hspi_num_mis(&self) -> RB_HSPI_NUM_MIS_R {
        RB_HSPI_NUM_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CRC error occur"]
    #[inline(always)]
    pub fn rb_hspi_crc_err(&mut self) -> RB_HSPI_CRC_ERR_W {
        RB_HSPI_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 2 - rx and tx sequence number mismatch"]
    #[inline(always)]
    pub fn rb_hspi_num_mis(&mut self) -> RB_HSPI_NUM_MIS_W {
        RB_HSPI_NUM_MIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel rtx status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_rtx_status](index.html) module"]
pub struct HSPI_RTX_STATUS_SPEC;
impl crate::RegisterSpec for HSPI_RTX_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hspi_rtx_status::R](R) reader structure"]
impl crate::Readable for HSPI_RTX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_rtx_status::W](W) writer structure"]
impl crate::Writable for HSPI_RTX_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_RTX_STATUS to value 0"]
impl crate::Resettable for HSPI_RTX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
