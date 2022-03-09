use crate::html::{FormEncoding, FormMethod};
use crate::url::Url;

pub(crate) mod form_submitter_element_seal {
    pub trait Seal {}
}

pub trait FormSubmitterElement: form_submitter_element_seal::Seal {
    fn form_method(&self) -> FormMethod;

    fn set_form_method(&self, method: FormMethod);

    fn form_encoding(&self) -> Option<FormEncoding>;

    fn set_form_encoding(&self, encoding: Option<FormEncoding>);

    fn form_action(&self) -> Option<Url>;

    fn set_form_action(&self, form_action: &Url);

    fn form_target(&self) -> String;

    fn set_form_target(&self, form_target: &str);

    fn form_no_validate(&self) -> bool;

    fn set_form_no_validate(&self, form_no_validate: bool);
}
