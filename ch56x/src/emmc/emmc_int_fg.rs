#[doc = "Register `EMMC_INT_FG` reader"]
pub struct R(crate::R<EMMC_INT_FG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_INT_FG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_INT_FG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_INT_FG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_INT_FG` writer"]
pub struct W(crate::W<EMMC_INT_FG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_INT_FG_SPEC>;
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
impl From<crate::W<EMMC_INT_FG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_INT_FG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_IF_RE_TMOUT` reader - indicate when expect the response, timeout"]
pub struct RB_EMMC_IF_RE_TMOUT_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_RE_TMOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_RE_TMOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_RE_TMOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_RE_TMOUT` writer - indicate when expect the response, timeout"]
pub struct RB_EMMC_IF_RE_TMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_RE_TMOUT_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_RECRC_WR` reader - indicate CRC error of the response"]
pub struct RB_EMMC_IF_RECRC_WR_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_RECRC_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_RECRC_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_RECRC_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_RECRC_WR` writer - indicate CRC error of the response"]
pub struct RB_EMMC_IF_RECRC_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_RECRC_WR_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_REIDX_ER` reader - indicate INDEX error of the response"]
pub struct RB_EMMC_IF_REIDX_ER_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_REIDX_ER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_REIDX_ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_REIDX_ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_REIDX_ER` writer - indicate INDEX error of the response"]
pub struct RB_EMMC_IF_REIDX_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_REIDX_ER_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_CMDDONE` reader - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
pub struct RB_EMMC_IF_CMDDONE_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_CMDDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_CMDDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_CMDDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_CMDDONE` writer - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
pub struct RB_EMMC_IF_CMDDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_CMDDONE_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_DATTMO` reader - data line busy timeout"]
pub struct RB_EMMC_IF_DATTMO_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_DATTMO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_DATTMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_DATTMO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_DATTMO` writer - data line busy timeout"]
pub struct RB_EMMC_IF_DATTMO_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_DATTMO_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_TRANERR` reader - last block have encountered a CRC error"]
pub struct RB_EMMC_IF_TRANERR_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_TRANERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_TRANERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_TRANERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_TRANERR` writer - last block have encountered a CRC error"]
pub struct RB_EMMC_IF_TRANERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_TRANERR_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_TRANDONE` reader - all the blocks have been tran/recv successfully"]
pub struct RB_EMMC_IF_TRANDONE_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_TRANDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_TRANDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_TRANDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_TRANDONE` writer - all the blocks have been tran/recv successfully"]
pub struct RB_EMMC_IF_TRANDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_TRANDONE_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_BKGAP` reader - every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment"]
pub struct RB_EMMC_IF_BKGAP_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_BKGAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_BKGAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_BKGAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_BKGAP` writer - every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment"]
pub struct RB_EMMC_IF_BKGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_BKGAP_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_FIFO_OV` reader - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
pub struct RB_EMMC_IF_FIFO_OV_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_FIFO_OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_FIFO_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_FIFO_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_FIFO_OV` writer - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
pub struct RB_EMMC_IF_FIFO_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_FIFO_OV_W<'a> {
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
#[doc = "Field `RB_EMMC_IF_SDIOINT` reader - interrupt from SDIO card inside"]
pub struct RB_EMMC_IF_SDIOINT_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_IF_SDIOINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_IF_SDIOINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_IF_SDIOINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_IF_SDIOINT` writer - interrupt from SDIO card inside"]
pub struct RB_EMMC_IF_SDIOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_IF_SDIOINT_W<'a> {
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
    #[doc = "Bit 0 - indicate when expect the response, timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_re_tmout(&self) -> RB_EMMC_IF_RE_TMOUT_R {
        RB_EMMC_IF_RE_TMOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - indicate CRC error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_recrc_wr(&self) -> RB_EMMC_IF_RECRC_WR_R {
        RB_EMMC_IF_RECRC_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - indicate INDEX error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_reidx_er(&self) -> RB_EMMC_IF_REIDX_ER_R {
        RB_EMMC_IF_REIDX_ER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
    #[inline(always)]
    pub fn rb_emmc_if_cmddone(&self) -> RB_EMMC_IF_CMDDONE_R {
        RB_EMMC_IF_CMDDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data line busy timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_dattmo(&self) -> RB_EMMC_IF_DATTMO_R {
        RB_EMMC_IF_DATTMO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - last block have encountered a CRC error"]
    #[inline(always)]
    pub fn rb_emmc_if_tranerr(&self) -> RB_EMMC_IF_TRANERR_R {
        RB_EMMC_IF_TRANERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - all the blocks have been tran/recv successfully"]
    #[inline(always)]
    pub fn rb_emmc_if_trandone(&self) -> RB_EMMC_IF_TRANDONE_R {
        RB_EMMC_IF_TRANDONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment"]
    #[inline(always)]
    pub fn rb_emmc_if_bkgap(&self) -> RB_EMMC_IF_BKGAP_R {
        RB_EMMC_IF_BKGAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
    #[inline(always)]
    pub fn rb_emmc_if_fifo_ov(&self) -> RB_EMMC_IF_FIFO_OV_R {
        RB_EMMC_IF_FIFO_OV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - interrupt from SDIO card inside"]
    #[inline(always)]
    pub fn rb_emmc_if_sdioint(&self) -> RB_EMMC_IF_SDIOINT_R {
        RB_EMMC_IF_SDIOINT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - indicate when expect the response, timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_re_tmout(&mut self) -> RB_EMMC_IF_RE_TMOUT_W {
        RB_EMMC_IF_RE_TMOUT_W { w: self }
    }
    #[doc = "Bit 1 - indicate CRC error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_recrc_wr(&mut self) -> RB_EMMC_IF_RECRC_WR_W {
        RB_EMMC_IF_RECRC_WR_W { w: self }
    }
    #[doc = "Bit 2 - indicate INDEX error of the response"]
    #[inline(always)]
    pub fn rb_emmc_if_reidx_er(&mut self) -> RB_EMMC_IF_REIDX_ER_W {
        RB_EMMC_IF_REIDX_ER_W { w: self }
    }
    #[doc = "Bit 3 - when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response"]
    #[inline(always)]
    pub fn rb_emmc_if_cmddone(&mut self) -> RB_EMMC_IF_CMDDONE_W {
        RB_EMMC_IF_CMDDONE_W { w: self }
    }
    #[doc = "Bit 4 - data line busy timeout"]
    #[inline(always)]
    pub fn rb_emmc_if_dattmo(&mut self) -> RB_EMMC_IF_DATTMO_W {
        RB_EMMC_IF_DATTMO_W { w: self }
    }
    #[doc = "Bit 5 - last block have encountered a CRC error"]
    #[inline(always)]
    pub fn rb_emmc_if_tranerr(&mut self) -> RB_EMMC_IF_TRANERR_W {
        RB_EMMC_IF_TRANERR_W { w: self }
    }
    #[doc = "Bit 6 - all the blocks have been tran/recv successfully"]
    #[inline(always)]
    pub fn rb_emmc_if_trandone(&mut self) -> RB_EMMC_IF_TRANDONE_W {
        RB_EMMC_IF_TRANDONE_W { w: self }
    }
    #[doc = "Bit 7 - every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment"]
    #[inline(always)]
    pub fn rb_emmc_if_bkgap(&mut self) -> RB_EMMC_IF_BKGAP_W {
        RB_EMMC_IF_BKGAP_W { w: self }
    }
    #[doc = "Bit 8 - fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow"]
    #[inline(always)]
    pub fn rb_emmc_if_fifo_ov(&mut self) -> RB_EMMC_IF_FIFO_OV_W {
        RB_EMMC_IF_FIFO_OV_W { w: self }
    }
    #[doc = "Bit 9 - interrupt from SDIO card inside"]
    #[inline(always)]
    pub fn rb_emmc_if_sdioint(&mut self) -> RB_EMMC_IF_SDIOINT_W {
        RB_EMMC_IF_SDIOINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 16bits interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_int_fg](index.html) module"]
pub struct EMMC_INT_FG_SPEC;
impl crate::RegisterSpec for EMMC_INT_FG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [emmc_int_fg::R](R) reader structure"]
impl crate::Readable for EMMC_INT_FG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_int_fg::W](W) writer structure"]
impl crate::Writable for EMMC_INT_FG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_INT_FG to value 0"]
impl crate::Resettable for EMMC_INT_FG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
