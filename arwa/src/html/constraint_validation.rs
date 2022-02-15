pub(crate) mod constraint_validation_target_seal {
    pub trait Seal {}
}

pub trait ConstraintValidationTarget: constraint_validation_target_seal::Seal {
    fn validity(&self) -> ValidityState;

    fn validation_message(&self) -> String;

    fn will_validate(&self) -> bool;

    fn check_validity(&self) -> bool;

    fn report_validity(&self) -> bool;

    fn set_custom_validity(&self, error: &str);
}

pub struct ValidityState {
    inner: web_sys::ValidityState,
}

impl ValidityState {
    delegate! {
        to self.inner {
            pub fn value_missing(&self) -> bool;

            pub fn type_mismatch(&self) -> bool;

            pub fn pattern_mismatch(&self) -> bool;

            pub fn too_long(&self) -> bool;

            pub fn too_short(&self) -> bool;

            pub fn range_underflow(&self) -> bool;

            pub fn range_overflow(&self) -> bool;

            pub fn step_mismatch(&self) -> bool;

            pub fn bad_input(&self) -> bool;

            pub fn custom_error(&self) -> bool;

            pub fn valid(&self) -> bool;
        }
    }
}

impl From<web_sys::ValidityState> for ValidityState {
    fn from(inner: web_sys::ValidityState) -> Self {
        ValidityState { inner }
    }
}

impl AsRef<web_sys::ValidityState> for ValidityState {
    fn as_ref(&self) -> &web_sys::ValidityState {
        &self.inner
    }
}

impl_common_wrapper_traits!(ValidityState);
