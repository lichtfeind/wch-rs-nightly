#[doc = "Register `EMMC_TRAN_MODE` reader"]
pub struct R(crate::R<EMMC_TRAN_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_TRAN_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_TRAN_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_TRAN_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_TRAN_MODE` writer"]
pub struct W(crate::W<EMMC_TRAN_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_TRAN_MODE_SPEC>;
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
impl From<crate::W<EMMC_TRAN_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_TRAN_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_DMA_DIR` reader - set DMA direction is controller to emmc card"]
pub struct RB_EMMC_DMA_DIR_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_DMA_DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_DMA_DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_DMA_DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_DMA_DIR` writer - set DMA direction is controller to emmc card"]
pub struct RB_EMMC_DMA_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_DMA_DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `RB_EMMC_GAP_STOP` reader - clock stop mode after block completion"]
pub struct RB_EMMC_GAP_STOP_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_GAP_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_GAP_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_GAP_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_GAP_STOP` writer - clock stop mode after block completion"]
pub struct RB_EMMC_GAP_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_GAP_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RB_EMMC_MODE_BOOT` reader - enable emmc boot mode"]
pub struct RB_EMMC_MODE_BOOT_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_MODE_BOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_MODE_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_MODE_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_MODE_BOOT` writer - enable emmc boot mode"]
pub struct RB_EMMC_MODE_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_MODE_BOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RB_EMMC_AUTOGAPSTOP` reader - enable auto set bTM_GAP_STOP when tran start"]
pub struct RB_EMMC_AUTOGAPSTOP_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_AUTOGAPSTOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_AUTOGAPSTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_AUTOGAPSTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_AUTOGAPSTOP` writer - enable auto set bTM_GAP_STOP when tran start"]
pub struct RB_EMMC_AUTOGAPSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_AUTOGAPSTOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `RB_EMMC_FIFO_RDY` reader - FIFO ready select signal when writing EMMC"]
pub struct RB_EMMC_FIFO_RDY_R(crate::FieldReader<u8, u8>);
impl RB_EMMC_FIFO_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_EMMC_FIFO_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_FIFO_RDY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_FIFO_RDY` writer - FIFO ready select signal when writing EMMC"]
pub struct RB_EMMC_FIFO_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_FIFO_RDY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `RB_EMMC_DMATN_CNT` reader - in double buffer mode,set the block count value of buffer switch"]
pub struct RB_EMMC_DMATN_CNT_R(crate::FieldReader<u8, u8>);
impl RB_EMMC_DMATN_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_EMMC_DMATN_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_DMATN_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_DMATN_CNT` writer - in double buffer mode,set the block count value of buffer switch"]
pub struct RB_EMMC_DMATN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_DMATN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `RB_EMMC_DULEDMA_EN` reader - enable double buffer dma"]
pub struct RB_EMMC_DULEDMA_EN_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_DULEDMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_DULEDMA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_DULEDMA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_DULEDMA_EN` writer - enable double buffer dma"]
pub struct RB_EMMC_DULEDMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_DULEDMA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - set DMA direction is controller to emmc card"]
    #[inline(always)]
    pub fn rb_emmc_dma_dir(&self) -> RB_EMMC_DMA_DIR_R {
        RB_EMMC_DMA_DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock stop mode after block completion"]
    #[inline(always)]
    pub fn rb_emmc_gap_stop(&self) -> RB_EMMC_GAP_STOP_R {
        RB_EMMC_GAP_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable emmc boot mode"]
    #[inline(always)]
    pub fn rb_emmc_mode_boot(&self) -> RB_EMMC_MODE_BOOT_R {
        RB_EMMC_MODE_BOOT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - enable auto set bTM_GAP_STOP when tran start"]
    #[inline(always)]
    pub fn rb_emmc_autogapstop(&self) -> RB_EMMC_AUTOGAPSTOP_R {
        RB_EMMC_AUTOGAPSTOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - FIFO ready select signal when writing EMMC"]
    #[inline(always)]
    pub fn rb_emmc_fifo_rdy(&self) -> RB_EMMC_FIFO_RDY_R {
        RB_EMMC_FIFO_RDY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - in double buffer mode,set the block count value of buffer switch"]
    #[inline(always)]
    pub fn rb_emmc_dmatn_cnt(&self) -> RB_EMMC_DMATN_CNT_R {
        RB_EMMC_DMATN_CNT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - enable double buffer dma"]
    #[inline(always)]
    pub fn rb_emmc_duledma_en(&self) -> RB_EMMC_DULEDMA_EN_R {
        RB_EMMC_DULEDMA_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set DMA direction is controller to emmc card"]
    #[inline(always)]
    pub fn rb_emmc_dma_dir(&mut self) -> RB_EMMC_DMA_DIR_W {
        RB_EMMC_DMA_DIR_W { w: self }
    }
    #[doc = "Bit 1 - clock stop mode after block completion"]
    #[inline(always)]
    pub fn rb_emmc_gap_stop(&mut self) -> RB_EMMC_GAP_STOP_W {
        RB_EMMC_GAP_STOP_W { w: self }
    }
    #[doc = "Bit 2 - enable emmc boot mode"]
    #[inline(always)]
    pub fn rb_emmc_mode_boot(&mut self) -> RB_EMMC_MODE_BOOT_W {
        RB_EMMC_MODE_BOOT_W { w: self }
    }
    #[doc = "Bit 4 - enable auto set bTM_GAP_STOP when tran start"]
    #[inline(always)]
    pub fn rb_emmc_autogapstop(&mut self) -> RB_EMMC_AUTOGAPSTOP_W {
        RB_EMMC_AUTOGAPSTOP_W { w: self }
    }
    #[doc = "Bits 6:7 - FIFO ready select signal when writing EMMC"]
    #[inline(always)]
    pub fn rb_emmc_fifo_rdy(&mut self) -> RB_EMMC_FIFO_RDY_W {
        RB_EMMC_FIFO_RDY_W { w: self }
    }
    #[doc = "Bits 8:14 - in double buffer mode,set the block count value of buffer switch"]
    #[inline(always)]
    pub fn rb_emmc_dmatn_cnt(&mut self) -> RB_EMMC_DMATN_CNT_W {
        RB_EMMC_DMATN_CNT_W { w: self }
    }
    #[doc = "Bit 16 - enable double buffer dma"]
    #[inline(always)]
    pub fn rb_emmc_duledma_en(&mut self) -> RB_EMMC_DULEDMA_EN_W {
        RB_EMMC_DULEDMA_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD TRANSFER MODE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_tran_mode](index.html) module"]
pub struct EMMC_TRAN_MODE_SPEC;
impl crate::RegisterSpec for EMMC_TRAN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_tran_mode::R](R) reader structure"]
impl crate::Readable for EMMC_TRAN_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_tran_mode::W](W) writer structure"]
impl crate::Writable for EMMC_TRAN_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_TRAN_MODE to value 0"]
impl crate::Resettable for EMMC_TRAN_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
