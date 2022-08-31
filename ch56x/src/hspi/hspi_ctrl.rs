#[doc = "Register `HSPI_CTRL` reader"]
pub struct R(crate::R<HSPI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_CTRL` writer"]
pub struct W(crate::W<HSPI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_CTRL_SPEC>;
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
impl From<crate::W<HSPI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_ENABLE` reader - parallel if enable"]
pub struct RB_HSPI_ENABLE_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_ENABLE` writer - parallel if enable"]
pub struct RB_HSPI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u8 & 1);
        self.w
    }
}
#[doc = "Field `RB_HSPI_DMA_EN` reader - parallel if dma enable"]
pub struct RB_HSPI_DMA_EN_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_DMA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_DMA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_DMA_EN` writer - parallel if dma enable"]
pub struct RB_HSPI_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_DMA_EN_W<'a> {
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
#[doc = "Field `RB_HSPI_SW_ACT` reader - parallel if transmit software trigger"]
pub struct RB_HSPI_SW_ACT_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_SW_ACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_SW_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_SW_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_SW_ACT` writer - parallel if transmit software trigger"]
pub struct RB_HSPI_SW_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_SW_ACT_W<'a> {
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
#[doc = "Field `RB_HSPI_ALL_CLR` reader - parallel if all clear"]
pub struct RB_HSPI_ALL_CLR_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_ALL_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_ALL_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_ALL_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_ALL_CLR` writer - parallel if all clear"]
pub struct RB_HSPI_ALL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_ALL_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u8 & 1) << 3);
        self.w
    }
}
#[doc = "Field `RB_HSPI_TRX_RST` reader - parallel if tx and rx logic clear, high action"]
pub struct RB_HSPI_TRX_RST_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_TRX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_TRX_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_TRX_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_TRX_RST` writer - parallel if tx and rx logic clear, high action"]
pub struct RB_HSPI_TRX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_TRX_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u8 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - parallel if enable"]
    #[inline(always)]
    pub fn rb_hspi_enable(&self) -> RB_HSPI_ENABLE_R {
        RB_HSPI_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parallel if dma enable"]
    #[inline(always)]
    pub fn rb_hspi_dma_en(&self) -> RB_HSPI_DMA_EN_R {
        RB_HSPI_DMA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - parallel if transmit software trigger"]
    #[inline(always)]
    pub fn rb_hspi_sw_act(&self) -> RB_HSPI_SW_ACT_R {
        RB_HSPI_SW_ACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - parallel if all clear"]
    #[inline(always)]
    pub fn rb_hspi_all_clr(&self) -> RB_HSPI_ALL_CLR_R {
        RB_HSPI_ALL_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - parallel if tx and rx logic clear, high action"]
    #[inline(always)]
    pub fn rb_hspi_trx_rst(&self) -> RB_HSPI_TRX_RST_R {
        RB_HSPI_TRX_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - parallel if enable"]
    #[inline(always)]
    pub fn rb_hspi_enable(&mut self) -> RB_HSPI_ENABLE_W {
        RB_HSPI_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - parallel if dma enable"]
    #[inline(always)]
    pub fn rb_hspi_dma_en(&mut self) -> RB_HSPI_DMA_EN_W {
        RB_HSPI_DMA_EN_W { w: self }
    }
    #[doc = "Bit 2 - parallel if transmit software trigger"]
    #[inline(always)]
    pub fn rb_hspi_sw_act(&mut self) -> RB_HSPI_SW_ACT_W {
        RB_HSPI_SW_ACT_W { w: self }
    }
    #[doc = "Bit 3 - parallel if all clear"]
    #[inline(always)]
    pub fn rb_hspi_all_clr(&mut self) -> RB_HSPI_ALL_CLR_W {
        RB_HSPI_ALL_CLR_W { w: self }
    }
    #[doc = "Bit 4 - parallel if tx and rx logic clear, high action"]
    #[inline(always)]
    pub fn rb_hspi_trx_rst(&mut self) -> RB_HSPI_TRX_RST_W {
        RB_HSPI_TRX_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if tx or rx control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_ctrl](index.html) module"]
pub struct HSPI_CTRL_SPEC;
impl crate::RegisterSpec for HSPI_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hspi_ctrl::R](R) reader structure"]
impl crate::Readable for HSPI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_ctrl::W](W) writer structure"]
impl crate::Writable for HSPI_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_CTRL to value 0x18"]
impl crate::Resettable for HSPI_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
