#[doc = "Register `HSPI_RX_ADDR0` reader"]
pub struct R(crate::R<HSPI_RX_ADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_RX_ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_RX_ADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_RX_ADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_RX_ADDR0` writer"]
pub struct W(crate::W<HSPI_RX_ADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_RX_ADDR0_SPEC>;
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
impl From<crate::W<HSPI_RX_ADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_RX_ADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_RX_ADDR0` reader - parallel if dma rx addr0"]
pub struct RB_HSPI_RX_ADDR0_R(crate::FieldReader<u32, u32>);
impl RB_HSPI_RX_ADDR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RB_HSPI_RX_ADDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_RX_ADDR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_RX_ADDR0` writer - parallel if dma rx addr0"]
pub struct RB_HSPI_RX_ADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_RX_ADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - parallel if dma rx addr0"]
    #[inline(always)]
    pub fn rb_hspi_rx_addr0(&self) -> RB_HSPI_RX_ADDR0_R {
        RB_HSPI_RX_ADDR0_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - parallel if dma rx addr0"]
    #[inline(always)]
    pub fn rb_hspi_rx_addr0(&mut self) -> RB_HSPI_RX_ADDR0_W {
        RB_HSPI_RX_ADDR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if dma rx addr0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_rx_addr0](index.html) module"]
pub struct HSPI_RX_ADDR0_SPEC;
impl crate::RegisterSpec for HSPI_RX_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hspi_rx_addr0::R](R) reader structure"]
impl crate::Readable for HSPI_RX_ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_rx_addr0::W](W) writer structure"]
impl crate::Writable for HSPI_RX_ADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_RX_ADDR0 to value 0"]
impl crate::Resettable for HSPI_RX_ADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
