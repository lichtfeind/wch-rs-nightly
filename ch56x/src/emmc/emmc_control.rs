#[doc = "Register `EMMC_CONTROL` reader"]
pub struct R(crate::R<EMMC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_CONTROL` writer"]
pub struct W(crate::W<EMMC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_CONTROL_SPEC>;
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
impl From<crate::W<EMMC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_LW_MASK` reader - effctive data width for sending or receiving data"]
pub struct RB_EMMC_LW_MASK_R(crate::FieldReader<u8, u8>);
impl RB_EMMC_LW_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_EMMC_LW_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_LW_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_LW_MASK` writer - effctive data width for sending or receiving data"]
pub struct RB_EMMC_LW_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_LW_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u8 & 3);
        self.w
    }
}
#[doc = "Field `RB_EMMC_ALL_CLR` reader - reset all the inner logic, default is valid"]
pub struct RB_EMMC_ALL_CLR_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_ALL_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_ALL_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_ALL_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_ALL_CLR` writer - reset all the inner logic, default is valid"]
pub struct RB_EMMC_ALL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_ALL_CLR_W<'a> {
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
#[doc = "Field `RB_EMMC_DMAEN` reader - enable the dma"]
pub struct RB_EMMC_DMAEN_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_DMAEN` writer - enable the dma"]
pub struct RB_EMMC_DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_DMAEN_W<'a> {
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
#[doc = "Field `RB_EMMC_RST_LGC` reader - reset the data tran/recv logic"]
pub struct RB_EMMC_RST_LGC_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_RST_LGC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_RST_LGC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_RST_LGC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_RST_LGC` writer - reset the data tran/recv logic"]
pub struct RB_EMMC_RST_LGC_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_RST_LGC_W<'a> {
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
#[doc = "Field `RB_EMMC_NEGSMP` reader - controller use nagedge sample cmd"]
pub struct RB_EMMC_NEGSMP_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_NEGSMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_NEGSMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_NEGSMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_NEGSMP` writer - controller use nagedge sample cmd"]
pub struct RB_EMMC_NEGSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_NEGSMP_W<'a> {
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
    #[doc = "Bits 0:1 - effctive data width for sending or receiving data"]
    #[inline(always)]
    pub fn rb_emmc_lw_mask(&self) -> RB_EMMC_LW_MASK_R {
        RB_EMMC_LW_MASK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reset all the inner logic, default is valid"]
    #[inline(always)]
    pub fn rb_emmc_all_clr(&self) -> RB_EMMC_ALL_CLR_R {
        RB_EMMC_ALL_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable the dma"]
    #[inline(always)]
    pub fn rb_emmc_dmaen(&self) -> RB_EMMC_DMAEN_R {
        RB_EMMC_DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reset the data tran/recv logic"]
    #[inline(always)]
    pub fn rb_emmc_rst_lgc(&self) -> RB_EMMC_RST_LGC_R {
        RB_EMMC_RST_LGC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - controller use nagedge sample cmd"]
    #[inline(always)]
    pub fn rb_emmc_negsmp(&self) -> RB_EMMC_NEGSMP_R {
        RB_EMMC_NEGSMP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - effctive data width for sending or receiving data"]
    #[inline(always)]
    pub fn rb_emmc_lw_mask(&mut self) -> RB_EMMC_LW_MASK_W {
        RB_EMMC_LW_MASK_W { w: self }
    }
    #[doc = "Bit 2 - reset all the inner logic, default is valid"]
    #[inline(always)]
    pub fn rb_emmc_all_clr(&mut self) -> RB_EMMC_ALL_CLR_W {
        RB_EMMC_ALL_CLR_W { w: self }
    }
    #[doc = "Bit 3 - enable the dma"]
    #[inline(always)]
    pub fn rb_emmc_dmaen(&mut self) -> RB_EMMC_DMAEN_W {
        RB_EMMC_DMAEN_W { w: self }
    }
    #[doc = "Bit 4 - reset the data tran/recv logic"]
    #[inline(always)]
    pub fn rb_emmc_rst_lgc(&mut self) -> RB_EMMC_RST_LGC_W {
        RB_EMMC_RST_LGC_W { w: self }
    }
    #[doc = "Bit 5 - controller use nagedge sample cmd"]
    #[inline(always)]
    pub fn rb_emmc_negsmp(&mut self) -> RB_EMMC_NEGSMP_W {
        RB_EMMC_NEGSMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 8bits control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_control](index.html) module"]
pub struct EMMC_CONTROL_SPEC;
impl crate::RegisterSpec for EMMC_CONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [emmc_control::R](R) reader structure"]
impl crate::Readable for EMMC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_control::W](W) writer structure"]
impl crate::Writable for EMMC_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_CONTROL to value 0x15"]
impl crate::Resettable for EMMC_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
