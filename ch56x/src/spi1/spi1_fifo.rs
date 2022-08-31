#[doc = "Register `SPI1_FIFO` reader"]
pub struct R(crate::R<SPI1_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI1_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI1_FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI1_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI1_FIFO` writer"]
pub struct W(crate::W<SPI1_FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI1_FIFO_SPEC>;
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
impl From<crate::W<SPI1_FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI1_FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_FIFO` reader - SPI FIFO register"]
pub struct SPI1_FIFO_R(crate::FieldReader<u8, u8>);
impl SPI1_FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI1_FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_FIFO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_FIFO` writer - SPI FIFO register"]
pub struct SPI1_FIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_FIFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI FIFO register"]
    #[inline(always)]
    pub fn spi1_fifo(&self) -> SPI1_FIFO_R {
        SPI1_FIFO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI FIFO register"]
    #[inline(always)]
    pub fn spi1_fifo(&mut self) -> SPI1_FIFO_W {
        SPI1_FIFO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 FIFO register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_fifo](index.html) module"]
pub struct SPI1_FIFO_SPEC;
impl crate::RegisterSpec for SPI1_FIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spi1_fifo::R](R) reader structure"]
impl crate::Readable for SPI1_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi1_fifo::W](W) writer structure"]
impl crate::Writable for SPI1_FIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI1_FIFO to value 0"]
impl crate::Resettable for SPI1_FIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
