#[doc = "Register `ECDC_SRAM_LEN` reader"]
pub struct R(crate::R<ECDC_SRAM_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECDC_SRAM_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECDC_SRAM_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECDC_SRAM_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECDC_SRAM_LEN` writer"]
pub struct W(crate::W<ECDC_SRAM_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECDC_SRAM_LEN_SPEC>;
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
impl From<crate::W<ECDC_SRAM_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECDC_SRAM_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_ECDC_SRAM_LEN` reader - encryption and decryption sram size register"]
pub struct RB_ECDC_SRAM_LEN_R(crate::FieldReader<u32, u32>);
impl RB_ECDC_SRAM_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RB_ECDC_SRAM_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_SRAM_LEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_SRAM_LEN` writer - encryption and decryption sram size register"]
pub struct RB_ECDC_SRAM_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_SRAM_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - encryption and decryption sram size register"]
    #[inline(always)]
    pub fn rb_ecdc_sram_len(&self) -> RB_ECDC_SRAM_LEN_R {
        RB_ECDC_SRAM_LEN_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - encryption and decryption sram size register"]
    #[inline(always)]
    pub fn rb_ecdc_sram_len(&mut self) -> RB_ECDC_SRAM_LEN_W {
        RB_ECDC_SRAM_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "encryption and decryption sram size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecdc_sram_len](index.html) module"]
pub struct ECDC_SRAM_LEN_SPEC;
impl crate::RegisterSpec for ECDC_SRAM_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecdc_sram_len::R](R) reader structure"]
impl crate::Readable for ECDC_SRAM_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecdc_sram_len::W](W) writer structure"]
impl crate::Writable for ECDC_SRAM_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECDC_SRAM_LEN to value 0"]
impl crate::Resettable for ECDC_SRAM_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
