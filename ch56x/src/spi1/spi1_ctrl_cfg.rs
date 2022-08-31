#[doc = "Register `SPI1_CTRL_CFG` reader"]
pub struct R(crate::R<SPI1_CTRL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI1_CTRL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI1_CTRL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI1_CTRL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI1_CTRL_CFG` writer"]
pub struct W(crate::W<SPI1_CTRL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI1_CTRL_CFG_SPEC>;
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
impl From<crate::W<SPI1_CTRL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI1_CTRL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SPI_DMA_ENABLE` reader - SPI DMA enable"]
pub struct RB_SPI_DMA_ENABLE_R(crate::FieldReader<bool, bool>);
impl RB_SPI_DMA_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SPI_DMA_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SPI_DMA_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SPI_DMA_ENABLE` writer - SPI DMA enable"]
pub struct RB_SPI_DMA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SPI_DMA_ENABLE_W<'a> {
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
#[doc = "Field `RB_SPI_DMA_LOOP` reader - SPI DMA address loop enable"]
pub struct RB_SPI_DMA_LOOP_R(crate::FieldReader<bool, bool>);
impl RB_SPI_DMA_LOOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SPI_DMA_LOOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SPI_DMA_LOOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SPI_DMA_LOOP` writer - SPI DMA address loop enable"]
pub struct RB_SPI_DMA_LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SPI_DMA_LOOP_W<'a> {
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
#[doc = "Field `RB_SPI_AUTO_IF` reader - enable buffer or FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag"]
pub struct RB_SPI_AUTO_IF_R(crate::FieldReader<bool, bool>);
impl RB_SPI_AUTO_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SPI_AUTO_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SPI_AUTO_IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SPI_AUTO_IF` writer - enable buffer or FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag"]
pub struct RB_SPI_AUTO_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SPI_AUTO_IF_W<'a> {
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
#[doc = "Field `RB_SPI_BIT_ORDER` reader - SPI bit data order"]
pub struct RB_SPI_BIT_ORDER_R(crate::FieldReader<bool, bool>);
impl RB_SPI_BIT_ORDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SPI_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SPI_BIT_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SPI_BIT_ORDER` writer - SPI bit data order"]
pub struct RB_SPI_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SPI_BIT_ORDER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SPI DMA enable"]
    #[inline(always)]
    pub fn rb_spi_dma_enable(&self) -> RB_SPI_DMA_ENABLE_R {
        RB_SPI_DMA_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SPI DMA address loop enable"]
    #[inline(always)]
    pub fn rb_spi_dma_loop(&self) -> RB_SPI_DMA_LOOP_R {
        RB_SPI_DMA_LOOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - enable buffer or FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag"]
    #[inline(always)]
    pub fn rb_spi_auto_if(&self) -> RB_SPI_AUTO_IF_R {
        RB_SPI_AUTO_IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI bit data order"]
    #[inline(always)]
    pub fn rb_spi_bit_order(&self) -> RB_SPI_BIT_ORDER_R {
        RB_SPI_BIT_ORDER_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI DMA enable"]
    #[inline(always)]
    pub fn rb_spi_dma_enable(&mut self) -> RB_SPI_DMA_ENABLE_W {
        RB_SPI_DMA_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - SPI DMA address loop enable"]
    #[inline(always)]
    pub fn rb_spi_dma_loop(&mut self) -> RB_SPI_DMA_LOOP_W {
        RB_SPI_DMA_LOOP_W { w: self }
    }
    #[doc = "Bit 4 - enable buffer or FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag"]
    #[inline(always)]
    pub fn rb_spi_auto_if(&mut self) -> RB_SPI_AUTO_IF_W {
        RB_SPI_AUTO_IF_W { w: self }
    }
    #[doc = "Bit 5 - SPI bit data order"]
    #[inline(always)]
    pub fn rb_spi_bit_order(&mut self) -> RB_SPI_BIT_ORDER_W {
        RB_SPI_BIT_ORDER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 configuration control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_ctrl_cfg](index.html) module"]
pub struct SPI1_CTRL_CFG_SPEC;
impl crate::RegisterSpec for SPI1_CTRL_CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spi1_ctrl_cfg::R](R) reader structure"]
impl crate::Readable for SPI1_CTRL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi1_ctrl_cfg::W](W) writer structure"]
impl crate::Writable for SPI1_CTRL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI1_CTRL_CFG to value 0"]
impl crate::Resettable for SPI1_CTRL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
