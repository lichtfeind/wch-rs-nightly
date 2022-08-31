#[doc = "Register `HSPI_BURST_CNT` reader"]
pub struct R(crate::R<HSPI_BURST_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_BURST_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_BURST_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_BURST_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_BURST_CNT` writer"]
pub struct W(crate::W<HSPI_BURST_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_BURST_CNT_SPEC>;
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
impl From<crate::W<HSPI_BURST_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_BURST_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_BURST_CNT` reader - parallel if tx burst count"]
pub struct RB_HSPI_BURST_CNT_R(crate::FieldReader<u8, u8>);
impl RB_HSPI_BURST_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_HSPI_BURST_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_BURST_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_BURST_CNT` writer - parallel if tx burst count"]
pub struct RB_HSPI_BURST_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_BURST_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - parallel if tx burst count"]
    #[inline(always)]
    pub fn rb_hspi_burst_cnt(&self) -> RB_HSPI_BURST_CNT_R {
        RB_HSPI_BURST_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - parallel if tx burst count"]
    #[inline(always)]
    pub fn rb_hspi_burst_cnt(&mut self) -> RB_HSPI_BURST_CNT_W {
        RB_HSPI_BURST_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if tx burst count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_burst_cnt](index.html) module"]
pub struct HSPI_BURST_CNT_SPEC;
impl crate::RegisterSpec for HSPI_BURST_CNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hspi_burst_cnt::R](R) reader structure"]
impl crate::Readable for HSPI_BURST_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_burst_cnt::W](W) writer structure"]
impl crate::Writable for HSPI_BURST_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_BURST_CNT to value 0"]
impl crate::Resettable for HSPI_BURST_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
