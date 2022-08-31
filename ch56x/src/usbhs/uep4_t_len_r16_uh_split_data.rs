#[doc = "Register `UEP4_T_LEN_R16_UH_SPLIT_DATA` reader"]
pub struct R(crate::R<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP4_T_LEN_R16_UH_SPLIT_DATA` writer"]
pub struct W(crate::W<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>;
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
impl From<crate::W<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP4_T_LEN_UH_SPLIT_DATA` reader - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
pub struct UEP4_T_LEN_UH_SPLIT_DATA_R(crate::FieldReader<u16, u16>);
impl UEP4_T_LEN_UH_SPLIT_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UEP4_T_LEN_UH_SPLIT_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UEP4_T_LEN_UH_SPLIT_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UEP4_T_LEN_UH_SPLIT_DATA` writer - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
pub struct UEP4_T_LEN_UH_SPLIT_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> UEP4_T_LEN_UH_SPLIT_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
    #[inline(always)]
    pub fn uep4_t_len_uh_split_data(&self) -> UEP4_T_LEN_UH_SPLIT_DATA_R {
        UEP4_T_LEN_UH_SPLIT_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
    #[inline(always)]
    pub fn uep4_t_len_uh_split_data(&mut self) -> UEP4_T_LEN_UH_SPLIT_DATA_W {
        UEP4_T_LEN_UH_SPLIT_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 4 transmittal length and USB host Tx SPLIT packet data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep4_t_len_r16_uh_split_data](index.html) module"]
pub struct UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC;
impl crate::RegisterSpec for UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep4_t_len_r16_uh_split_data::R](R) reader structure"]
impl crate::Readable for UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep4_t_len_r16_uh_split_data::W](W) writer structure"]
impl crate::Writable for UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP4_T_LEN_R16_UH_SPLIT_DATA to value 0"]
impl crate::Resettable for UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
