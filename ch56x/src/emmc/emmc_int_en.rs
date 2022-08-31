#[doc = "Register `EMMC_INT_EN` reader"]
pub struct R(crate::R<EMMC_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_INT_EN` writer"]
pub struct W(crate::W<EMMC_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_INT_EN_SPEC>;
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
impl From<crate::W<EMMC_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_IE_RE_TMOUT` reader - command response timeout interrupt enable"]
pub struct RB_EMMC_IE_RE_TMOUT_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_RE_TMOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_RE_TMOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_RE_TMOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_RE_TMOUT` writer - command response timeout interrupt enable"]
pub struct RB_EMMC_IE_RE_TMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_RE_TMOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_RECRC_WR` reader - response CRC check error interrupt enable"]
pub struct RB_EMMC_IE_RECRC_WR_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_RECRC_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_RECRC_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_RECRC_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_RECRC_WR` writer - response CRC check error interrupt enable"]
pub struct RB_EMMC_IE_RECRC_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_RECRC_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_REIDX_ER` reader - response index check error interrupt enable"]
pub struct RB_EMMC_IE_REIDX_ER_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_REIDX_ER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_REIDX_ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_REIDX_ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_REIDX_ER` writer - response index check error interrupt enable"]
pub struct RB_EMMC_IE_REIDX_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_REIDX_ER_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_CMDDONE` reader - command completion interrupt enable"]
pub struct RB_EMMC_IE_CMDDONE_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_CMDDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_CMDDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_CMDDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_CMDDONE` writer - command completion interrupt enable"]
pub struct RB_EMMC_IE_CMDDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_CMDDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_DATTMO` reader - data timeout interrupt enable"]
pub struct RB_EMMC_IE_DATTMO_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_DATTMO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_DATTMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_DATTMO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_DATTMO` writer - data timeout interrupt enable"]
pub struct RB_EMMC_IE_DATTMO_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_DATTMO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_TRANERR` reader - blocks transfer CRC error interrupt enable"]
pub struct RB_EMMC_IE_TRANERR_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_TRANERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_TRANERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_TRANERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_TRANERR` writer - blocks transfer CRC error interrupt enable"]
pub struct RB_EMMC_IE_TRANERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_TRANERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_TRANDONE` reader - all blocks transfer complete interrupt enable"]
pub struct RB_EMMC_IE_TRANDONE_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_TRANDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_TRANDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_TRANDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_TRANDONE` writer - all blocks transfer complete interrupt enable"]
pub struct RB_EMMC_IE_TRANDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_TRANDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_BKGAP` reader - single block transmission completion interrupt enable"]
pub struct RB_EMMC_IE_BKGAP_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_BKGAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_BKGAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_BKGAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_BKGAP` writer - single block transmission completion interrupt enable"]
pub struct RB_EMMC_IE_BKGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_BKGAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_FIFO_OV` reader - FIFO overflow interrupt enable"]
pub struct RB_EMMC_IE_FIFO_OV_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_FIFO_OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_FIFO_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_FIFO_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_FIFO_OV` writer - FIFO overflow interrupt enable"]
pub struct RB_EMMC_IE_FIFO_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_FIFO_OV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `RB_EMMC_IE_SDIOINT` reader - SDIO card interrupt enable"]
pub struct RB_EMMC_IE_SDIOINT_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IE_SDIOINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IE_SDIOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IE_SDIOINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IE_SDIOINT` writer - SDIO card interrupt enable"]
pub struct RB_EMMC_IE_SDIOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IE_SDIOINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - command response timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_re_tmout(&self) -> RB_EMMC_IE_RE_TMOUT_R {
        RB_EMMC_IE_RE_TMOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - response CRC check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_recrc_wr(&self) -> RB_EMMC_IE_RECRC_WR_R {
        RB_EMMC_IE_RECRC_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - response index check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_reidx_er(&self) -> RB_EMMC_IE_REIDX_ER_R {
        RB_EMMC_IE_REIDX_ER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - command completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_cmddone(&self) -> RB_EMMC_IE_CMDDONE_R {
        RB_EMMC_IE_CMDDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_dattmo(&self) -> RB_EMMC_IE_DATTMO_R {
        RB_EMMC_IE_DATTMO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - blocks transfer CRC error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_tranerr(&self) -> RB_EMMC_IE_TRANERR_R {
        RB_EMMC_IE_TRANERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - all blocks transfer complete interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_trandone(&self) -> RB_EMMC_IE_TRANDONE_R {
        RB_EMMC_IE_TRANDONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - single block transmission completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_bkgap(&self) -> RB_EMMC_IE_BKGAP_R {
        RB_EMMC_IE_BKGAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_fifo_ov(&self) -> RB_EMMC_IE_FIFO_OV_R {
        RB_EMMC_IE_FIFO_OV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO card interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_sdioint(&self) -> RB_EMMC_IE_SDIOINT_R {
        RB_EMMC_IE_SDIOINT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - command response timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_re_tmout(&mut self) -> RB_EMMC_IE_RE_TMOUT_W {
        RB_EMMC_IE_RE_TMOUT_W { w: self }
    }
    #[doc = "Bit 1 - response CRC check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_recrc_wr(&mut self) -> RB_EMMC_IE_RECRC_WR_W {
        RB_EMMC_IE_RECRC_WR_W { w: self }
    }
    #[doc = "Bit 2 - response index check error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_reidx_er(&mut self) -> RB_EMMC_IE_REIDX_ER_W {
        RB_EMMC_IE_REIDX_ER_W { w: self }
    }
    #[doc = "Bit 3 - command completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_cmddone(&mut self) -> RB_EMMC_IE_CMDDONE_W {
        RB_EMMC_IE_CMDDONE_W { w: self }
    }
    #[doc = "Bit 4 - data timeout interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_dattmo(&mut self) -> RB_EMMC_IE_DATTMO_W {
        RB_EMMC_IE_DATTMO_W { w: self }
    }
    #[doc = "Bit 5 - blocks transfer CRC error interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_tranerr(&mut self) -> RB_EMMC_IE_TRANERR_W {
        RB_EMMC_IE_TRANERR_W { w: self }
    }
    #[doc = "Bit 6 - all blocks transfer complete interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_trandone(&mut self) -> RB_EMMC_IE_TRANDONE_W {
        RB_EMMC_IE_TRANDONE_W { w: self }
    }
    #[doc = "Bit 7 - single block transmission completion interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_bkgap(&mut self) -> RB_EMMC_IE_BKGAP_W {
        RB_EMMC_IE_BKGAP_W { w: self }
    }
    #[doc = "Bit 8 - FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_fifo_ov(&mut self) -> RB_EMMC_IE_FIFO_OV_W {
        RB_EMMC_IE_FIFO_OV_W { w: self }
    }
    #[doc = "Bit 9 - SDIO card interrupt enable"]
    #[inline(always)]
    pub fn rb_emmc_ie_sdioint(&mut self) -> RB_EMMC_IE_SDIOINT_W {
        RB_EMMC_IE_SDIOINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 16bits interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_int_en](index.html) module"]
pub struct EMMC_INT_EN_SPEC;
impl crate::RegisterSpec for EMMC_INT_EN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [emmc_int_en::R](R) reader structure"]
impl crate::Readable for EMMC_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_int_en::W](W) writer structure"]
impl crate::Writable for EMMC_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_INT_EN to value 0"]
impl crate::Resettable for EMMC_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
