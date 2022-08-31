#[doc = "Register `LMP_RX_DATA2` reader"]
pub struct R(crate::R<LMP_RX_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMP_RX_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMP_RX_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMP_RX_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMP_RX_DATA2` writer"]
pub struct W(crate::W<LMP_RX_DATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMP_RX_DATA2_SPEC>;
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
impl From<crate::W<LMP_RX_DATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMP_RX_DATA2_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmp_rx_data2](index.html) module"]
pub struct LMP_RX_DATA2_SPEC;
impl crate::RegisterSpec for LMP_RX_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmp_rx_data2::R](R) reader structure"]
impl crate::Readable for LMP_RX_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmp_rx_data2::W](W) writer structure"]
impl crate::Writable for LMP_RX_DATA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMP_RX_DATA2 to value 0"]
impl crate::Resettable for LMP_RX_DATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
