#[doc = "Register `RBR_THR` reader"]
pub struct R(crate::R<RBR_THR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBR_THR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBR_THR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBR_THR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBR_THR` writer"]
pub struct W(crate::W<RBR_THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBR_THR_SPEC>;
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
impl From<crate::W<RBR_THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBR_THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBR_R8_UART1_THR` reader - UART receiver buffer, receiving byte_ UART transmitter holding, transmittal byte"]
pub struct RBR_R8_UART1_THR_R(crate::FieldReader<u8, u8>);
impl RBR_R8_UART1_THR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBR_R8_UART1_THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBR_R8_UART1_THR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBR_R8_UART1_THR` writer - UART receiver buffer, receiving byte_ UART transmitter holding, transmittal byte"]
pub struct RBR_R8_UART1_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> RBR_R8_UART1_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART receiver buffer, receiving byte_ UART transmitter holding, transmittal byte"]
    #[inline(always)]
    pub fn rbr_r8_uart1_thr(&self) -> RBR_R8_UART1_THR_R {
        RBR_R8_UART1_THR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART receiver buffer, receiving byte_ UART transmitter holding, transmittal byte"]
    #[inline(always)]
    pub fn rbr_r8_uart1_thr(&mut self) -> RBR_R8_UART1_THR_W {
        RBR_R8_UART1_THR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1 receiver buffer, receiving byte _ UART1 transmitter holding, transmittal byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbr_thr](index.html) module"]
pub struct RBR_THR_SPEC;
impl crate::RegisterSpec for RBR_THR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rbr_thr::R](R) reader structure"]
impl crate::Readable for RBR_THR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbr_thr::W](W) writer structure"]
impl crate::Writable for RBR_THR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RBR_THR to value 0"]
impl crate::Resettable for RBR_THR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
