#[doc = "Register `UEP2_RX_CTRL_R8_UH_RX_CTRL` reader"]
pub struct R(crate::R<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP2_RX_CTRL_R8_UH_RX_CTRL` writer"]
pub struct W(crate::W<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>;
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
impl From<crate::W<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UEP_RRES_MASK_RB_UH_RRES_MASK` reader - bit mask of handshake response type for USB endpoint X receiving (OUT) and Host reeiver response control bit"]
pub struct RB_UEP_RRES_MASK_RB_UH_RRES_MASK_R(crate::FieldReader<u8, u8>);
impl RB_UEP_RRES_MASK_RB_UH_RRES_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_UEP_RRES_MASK_RB_UH_RRES_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UEP_RRES_MASK_RB_UH_RRES_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UEP_RRES_MASK_RB_UH_RRES_MASK` writer - bit mask of handshake response type for USB endpoint X receiving (OUT) and Host reeiver response control bit"]
pub struct RB_UEP_RRES_MASK_RB_UH_RRES_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UEP_RRES_MASK_RB_UH_RRES_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u8 & 3);
        self.w
    }
}
#[doc = "Field `RB_UEP_RRES_NO_RB_UH_RRES_NO` reader - Prepared no response and Response control bit of host receiver"]
pub struct RB_UEP_RRES_NO_RB_UH_RRES_NO_R(crate::FieldReader<bool, bool>);
impl RB_UEP_RRES_NO_RB_UH_RRES_NO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UEP_RRES_NO_RB_UH_RRES_NO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UEP_RRES_NO_RB_UH_RRES_NO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UEP_RRES_NO_RB_UH_RRES_NO` writer - Prepared no response and Response control bit of host receiver"]
pub struct RB_UEP_RRES_NO_RB_UH_RRES_NO_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UEP_RRES_NO_RB_UH_RRES_NO_W<'a> {
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
#[doc = "Field `RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK` reader - expected data toggle flag of USB endpoint X receiving and expected data toggle flag of host receiving (IN)"]
pub struct RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_R(crate::FieldReader<u8, u8>);
impl RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK` writer - expected data toggle flag of USB endpoint X receiving and expected data toggle flag of host receiving (IN)"]
pub struct RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 3)) | ((value as u8 & 3) << 3);
        self.w
    }
}
#[doc = "Field `RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG` reader - enable automatic toggle after successful transfer completion on endpoint and enable automatic toggle after successful receiver completion"]
pub struct RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_R(crate::FieldReader<bool, bool>);
impl RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG` writer - enable automatic toggle after successful transfer completion on endpoint and enable automatic toggle after successful receiver completion"]
pub struct RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u8 & 1) << 5);
        self.w
    }
}
#[doc = "Field `RB_UH_RDATA_NO` reader - expect no data packet, for high speed hub in host mode"]
pub struct RB_UH_RDATA_NO_R(crate::FieldReader<bool, bool>);
impl RB_UH_RDATA_NO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UH_RDATA_NO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UH_RDATA_NO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UH_RDATA_NO` writer - expect no data packet, for high speed hub in host mode"]
pub struct RB_UH_RDATA_NO_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UH_RDATA_NO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u8 & 1) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X receiving (OUT) and Host reeiver response control bit"]
    #[inline(always)]
    pub fn rb_uep_rres_mask_rb_uh_rres_mask(&self) -> RB_UEP_RRES_MASK_RB_UH_RRES_MASK_R {
        RB_UEP_RRES_MASK_RB_UH_RRES_MASK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Prepared no response and Response control bit of host receiver"]
    #[inline(always)]
    pub fn rb_uep_rres_no_rb_uh_rres_no(&self) -> RB_UEP_RRES_NO_RB_UH_RRES_NO_R {
        RB_UEP_RRES_NO_RB_UH_RRES_NO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - expected data toggle flag of USB endpoint X receiving and expected data toggle flag of host receiving (IN)"]
    #[inline(always)]
    pub fn rb_uep_r_tog_mask_rb_uh_r_tog_mask(&self) -> RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_R {
        RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint and enable automatic toggle after successful receiver completion"]
    #[inline(always)]
    pub fn rb_uep_r_autotog_rb_uh_r_autotog(&self) -> RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_R {
        RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - expect no data packet, for high speed hub in host mode"]
    #[inline(always)]
    pub fn rb_uh_rdata_no(&self) -> RB_UH_RDATA_NO_R {
        RB_UH_RDATA_NO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X receiving (OUT) and Host reeiver response control bit"]
    #[inline(always)]
    pub fn rb_uep_rres_mask_rb_uh_rres_mask(&mut self) -> RB_UEP_RRES_MASK_RB_UH_RRES_MASK_W {
        RB_UEP_RRES_MASK_RB_UH_RRES_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Prepared no response and Response control bit of host receiver"]
    #[inline(always)]
    pub fn rb_uep_rres_no_rb_uh_rres_no(&mut self) -> RB_UEP_RRES_NO_RB_UH_RRES_NO_W {
        RB_UEP_RRES_NO_RB_UH_RRES_NO_W { w: self }
    }
    #[doc = "Bits 3:4 - expected data toggle flag of USB endpoint X receiving and expected data toggle flag of host receiving (IN)"]
    #[inline(always)]
    pub fn rb_uep_r_tog_mask_rb_uh_r_tog_mask(&mut self) -> RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_W {
        RB_UEP_R_TOG_MASK_RB_UH_R_TOG_MASK_W { w: self }
    }
    #[doc = "Bit 5 - enable automatic toggle after successful transfer completion on endpoint and enable automatic toggle after successful receiver completion"]
    #[inline(always)]
    pub fn rb_uep_r_autotog_rb_uh_r_autotog(&mut self) -> RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_W {
        RB_UEP_R_AUTOTOG_RB_UH_R_AUTOTOG_W { w: self }
    }
    #[doc = "Bit 6 - expect no data packet, for high speed hub in host mode"]
    #[inline(always)]
    pub fn rb_uh_rdata_no(&mut self) -> RB_UH_RDATA_NO_W {
        RB_UH_RDATA_NO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 2 rx control and USb host receive endpoint control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep2_rx_ctrl_r8_uh_rx_ctrl](index.html) module"]
pub struct UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC;
impl crate::RegisterSpec for UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uep2_rx_ctrl_r8_uh_rx_ctrl::R](R) reader structure"]
impl crate::Readable for UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep2_rx_ctrl_r8_uh_rx_ctrl::W](W) writer structure"]
impl crate::Writable for UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP2_RX_CTRL_R8_UH_RX_CTRL to value 0"]
impl crate::Resettable for UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}