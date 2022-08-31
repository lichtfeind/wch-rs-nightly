#[doc = "Register `LMP_PORT_CAP` reader"]
pub struct R(crate::R<LMP_PORT_CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMP_PORT_CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMP_PORT_CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMP_PORT_CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMP_PORT_CAP` writer"]
pub struct W(crate::W<LMP_PORT_CAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMP_PORT_CAP_SPEC>;
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
impl From<crate::W<LMP_PORT_CAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMP_PORT_CAP_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmp_port_cap](index.html) module"]
pub struct LMP_PORT_CAP_SPEC;
impl crate::RegisterSpec for LMP_PORT_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmp_port_cap::R](R) reader structure"]
impl crate::Readable for LMP_PORT_CAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmp_port_cap::W](W) writer structure"]
impl crate::Writable for LMP_PORT_CAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMP_PORT_CAP to value 0"]
impl crate::Resettable for LMP_PORT_CAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
