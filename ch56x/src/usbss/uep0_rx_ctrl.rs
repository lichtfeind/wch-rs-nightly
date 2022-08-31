#[doc = "Register `UEP0_RX_CTRL` reader"]
pub struct R(crate::R<UEP0_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP0_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP0_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP0_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP0_RX_CTRL` writer"]
pub struct W(crate::W<UEP0_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP0_RX_CTRL_SPEC>;
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
impl From<crate::W<UEP0_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP0_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - None"]
pub struct LEN_R(crate::FieldReader<u16, u16>);
impl LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN` writer - None"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `NUMP` reader - None"]
pub struct NUMP_R(crate::FieldReader<u8, u8>);
impl NUMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMP` writer - None"]
pub struct NUMP_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `IT` reader - None"]
pub struct IT_R(crate::FieldReader<u8, u8>);
impl IT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IT` writer - None"]
pub struct IT_W<'a> {
    w: &'a mut W,
}
impl<'a> IT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
#[doc = "Field `STATUS` reader - None"]
pub struct STATUS_R(crate::FieldReader<u8, u8>);
impl STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS` writer - None"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `IS_STATUS` reader - Received STATUS package"]
pub struct IS_STATUS_R(crate::FieldReader<bool, bool>);
impl IS_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS_STATUS` writer - Received STATUS package"]
pub struct IS_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_STATUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `IS_SETUP` reader - Received SETUP package"]
pub struct IS_SETUP_R(crate::FieldReader<bool, bool>);
impl IS_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IS_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS_SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS_SETUP` writer - Received SETUP package"]
pub struct IS_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_SETUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - None"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - None"]
    #[inline(always)]
    pub fn nump(&self) -> NUMP_R {
        NUMP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - None"]
    #[inline(always)]
    pub fn it(&self) -> IT_R {
        IT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:27 - None"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 29 - Received STATUS package"]
    #[inline(always)]
    pub fn is_status(&self) -> IS_STATUS_R {
        IS_STATUS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Received SETUP package"]
    #[inline(always)]
    pub fn is_setup(&self) -> IS_SETUP_R {
        IS_SETUP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - None"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Bits 16:20 - None"]
    #[inline(always)]
    pub fn nump(&mut self) -> NUMP_W {
        NUMP_W { w: self }
    }
    #[doc = "Bits 21:25 - None"]
    #[inline(always)]
    pub fn it(&mut self) -> IT_W {
        IT_W { w: self }
    }
    #[doc = "Bits 26:27 - None"]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
    #[doc = "Bit 29 - Received STATUS package"]
    #[inline(always)]
    pub fn is_status(&mut self) -> IS_STATUS_W {
        IS_STATUS_W { w: self }
    }
    #[doc = "Bit 30 - Received SETUP package"]
    #[inline(always)]
    pub fn is_setup(&mut self) -> IS_SETUP_W {
        IS_SETUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep0_rx_ctrl](index.html) module"]
pub struct UEP0_RX_CTRL_SPEC;
impl crate::RegisterSpec for UEP0_RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep0_rx_ctrl::R](R) reader structure"]
impl crate::Readable for UEP0_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep0_rx_ctrl::W](W) writer structure"]
impl crate::Writable for UEP0_RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP0_RX_CTRL to value 0"]
impl crate::Resettable for UEP0_RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
