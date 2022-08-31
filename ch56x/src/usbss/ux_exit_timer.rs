#[doc = "Register `UX_EXIT_TIMER` reader"]
pub struct R(crate::R<UX_EXIT_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UX_EXIT_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UX_EXIT_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UX_EXIT_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UX_EXIT_TIMER` writer"]
pub struct W(crate::W<UX_EXIT_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UX_EXIT_TIMER_SPEC>;
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
impl From<crate::W<UX_EXIT_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UX_EXIT_TIMER_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ux_exit_timer](index.html) module"]
pub struct UX_EXIT_TIMER_SPEC;
impl crate::RegisterSpec for UX_EXIT_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ux_exit_timer::R](R) reader structure"]
impl crate::Readable for UX_EXIT_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ux_exit_timer::W](W) writer structure"]
impl crate::Writable for UX_EXIT_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UX_EXIT_TIMER to value 0"]
impl crate::Resettable for UX_EXIT_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}