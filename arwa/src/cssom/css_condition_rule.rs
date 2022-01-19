pub(crate) mod css_condition_rule_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_css_condition_rule(&self) -> &web_sys::CssConditionRule;
    }
}

pub trait CssConditionRule: css_condition_rule_seal::Seal {
    fn condition_text(&self) -> String {
        self.as_web_sys_css_condition_rule().condition_text()
    }

    fn set_condition_text(&self, condition_text: &str) {
        self.as_web_sys_css_condition_rule()
            .set_condition_text(condition_text);
    }
}
