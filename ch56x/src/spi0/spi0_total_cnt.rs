#[doc = "Register `SPI0_TOTAL_CNT` reader"]
pub struct R(crate::R<SPI0_TOTAL_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI0_TOTAL_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI0_TOTAL_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI0_TOTAL_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI0_TOTAL_CNT` writer"]
pub struct W(crate::W<SPI0_TOTAL_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI0_TOTAL_CNT_SPEC>;
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
impl From<crate::W<SPI0_TOTAL_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI0_TOTAL_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0_TOTAL_CNT` reader - SPI total byte count, only low 12 bit"]
pub struct SPI0_TOTAL_CNT_R(crate::FieldReader<u16, u16>);
impl SPI0_TOTAL_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SPI0_TOTAL_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_TOTAL_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_TOTAL_CNT` writer - SPI total byte count, only low 12 bit"]
pub struct SPI0_TOTAL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_TOTAL_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SPI total byte count, only low 12 bit"]
    #[inline(always)]
    pub fn spi0_total_cnt(&self) -> SPI0_TOTAL_CNT_R {
        SPI0_TOTAL_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI total byte count, only low 12 bit"]
    #[inline(always)]
    pub fn spi0_total_cnt(&mut self) -> SPI0_TOTAL_CNT_W {
        SPI0_TOTAL_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 total byte count, only low 12 bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_total_cnt](index.html) module"]
pub struct SPI0_TOTAL_CNT_SPEC;
impl crate::RegisterSpec for SPI0_TOTAL_CNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi0_total_cnt::R](R) reader structure"]
impl crate::Readable for SPI0_TOTAL_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi0_total_cnt::W](W) writer structure"]
impl crate::Writable for SPI0_TOTAL_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI0_TOTAL_CNT to value 0"]
impl crate::Resettable for SPI0_TOTAL_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
