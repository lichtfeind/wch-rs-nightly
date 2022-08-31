#[doc = "Register `LINK_STREAM_ID` reader"]
pub struct R(crate::R<LINK_STREAM_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINK_STREAM_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINK_STREAM_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINK_STREAM_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINK_STREAM_ID` writer"]
pub struct W(crate::W<LINK_STREAM_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINK_STREAM_ID_SPEC>;
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
impl From<crate::W<LINK_STREAM_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINK_STREAM_ID_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [link_stream_id](index.html) module"]
pub struct LINK_STREAM_ID_SPEC;
impl crate::RegisterSpec for LINK_STREAM_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [link_stream_id::R](R) reader structure"]
impl crate::Readable for LINK_STREAM_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [link_stream_id::W](W) writer structure"]
impl crate::Writable for LINK_STREAM_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINK_STREAM_ID to value 0"]
impl crate::Resettable for LINK_STREAM_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
