#[doc = "Register `LINK_ERR_STATUS` reader"]
pub struct R(crate::R<LINK_ERR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINK_ERR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINK_ERR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINK_ERR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINK_ERR_STATUS` writer"]
pub struct W(crate::W<LINK_ERR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINK_ERR_STATUS_SPEC>;
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
impl From<crate::W<LINK_ERR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINK_ERR_STATUS_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [link_err_status](index.html) module"]
pub struct LINK_ERR_STATUS_SPEC;
impl crate::RegisterSpec for LINK_ERR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [link_err_status::R](R) reader structure"]
impl crate::Readable for LINK_ERR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [link_err_status::W](W) writer structure"]
impl crate::Writable for LINK_ERR_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINK_ERR_STATUS to value 0"]
impl crate::Resettable for LINK_ERR_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
