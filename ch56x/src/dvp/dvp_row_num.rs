#[doc = "Register `DVP_ROW_NUM` reader"]
pub struct R(crate::R<DVP_ROW_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_ROW_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_ROW_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_ROW_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVP_ROW_NUM` writer"]
pub struct W(crate::W<DVP_ROW_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_ROW_NUM_SPEC>;
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
impl From<crate::W<DVP_ROW_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_ROW_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_ROW_NUM` reader - the number of rows contained in a frame of image data"]
pub struct RB_DVP_ROW_NUM_R(crate::FieldReader<u16, u16>);
impl RB_DVP_ROW_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RB_DVP_ROW_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_ROW_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_ROW_NUM` writer - the number of rows contained in a frame of image data"]
pub struct RB_DVP_ROW_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_ROW_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - the number of rows contained in a frame of image data"]
    #[inline(always)]
    pub fn rb_dvp_row_num(&self) -> RB_DVP_ROW_NUM_R {
        RB_DVP_ROW_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of rows contained in a frame of image data"]
    #[inline(always)]
    pub fn rb_dvp_row_num(&mut self) -> RB_DVP_ROW_NUM_W {
        RB_DVP_ROW_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DVP row number of a frame indicator register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_row_num](index.html) module"]
pub struct DVP_ROW_NUM_SPEC;
impl crate::RegisterSpec for DVP_ROW_NUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dvp_row_num::R](R) reader structure"]
impl crate::Readable for DVP_ROW_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_row_num::W](W) writer structure"]
impl crate::Writable for DVP_ROW_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DVP_ROW_NUM to value 0"]
impl crate::Resettable for DVP_ROW_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
