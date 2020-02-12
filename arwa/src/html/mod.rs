macro_rules! impl_html_common_traits {
    ($crate_tpe:ident, $web_sys_tpe:ident) => {
        impl From<web_sys::$web_sys_tpe> for $crate_tpe {
            fn from(inner: web_sys::$web_sys_tpe) -> Self {
                $crate_tpe { inner }
            }
        }

        impl AsRef<web_sys::$web_sys_tpe> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::$web_sys_tpe {
                &self.inner
            }
        }

        impl AsRef<web_sys::HtmlElement> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::HtmlElement {
                self.inner.as_ref()
            }
        }

        impl AsRef<web_sys::Element> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::Element {
                self.inner.as_ref()
            }
        }

        impl AsRef<web_sys::Node> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::Node {
                self.inner.as_ref()
            }
        }

        impl AsRef<web_sys::EventTarget> for $crate_tpe {
            fn as_ref(&self) -> &web_sys::EventTarget {
                self.inner.as_ref()
            }
        }

        impl $crate::console::Write for $crate_tpe {
            fn write(&self, writer: &mut $crate::console::Writer) {
                writer.write_1(self.inner.as_ref());
            }
        }

        impl PartialEq for $crate_tpe {
            fn eq(&self, other: &$crate_tpe) -> bool {
                self.inner.eq(&other.inner)
            }
        }

        impl GlobalEventHandlers for $crate_tpe {}
        impl Node for $crate_tpe {}
        impl Element for $crate_tpe {}
        impl HtmlElement for $crate_tpe {}

        impl TryFrom<GenericEventTarget> for $crate_tpe {
            type Error = InvalidCast<GenericEventTarget>;

            fn try_from(value: GenericEventTarget) -> Result<Self, Self::Error> {
                let value: web_sys::EventTarget = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| InvalidCast(e.into()))
            }
        }

        impl TryFrom<GenericNode> for $crate_tpe {
            type Error = InvalidCast<GenericNode>;

            fn try_from(value: GenericNode) -> Result<Self, Self::Error> {
                let value: web_sys::Node = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| InvalidCast(e.into()))
            }
        }

        impl TryFrom<GenericElement> for $crate_tpe {
            type Error = InvalidCast<GenericElement>;

            fn try_from(value: GenericElement) -> Result<Self, Self::Error> {
                let value: web_sys::Element = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| InvalidCast(e.into()))
            }
        }

        impl TryFrom<GenericHtmlElement> for $crate_tpe {
            type Error = InvalidCast<GenericHtmlElement>;

            fn try_from(value: GenericHtmlElement) -> Result<Self, Self::Error> {
                let value: web_sys::HtmlElement = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| InvalidCast(e.into()))
            }
        }

        impl From<$crate_tpe> for GenericHtmlElement {
            fn from(element: $crate_tpe) -> Self {
                let element: web_sys::HtmlElement = element.inner.unchecked_into();

                element.into()
            }
        }

        impl From<$crate_tpe> for GenericElement {
            fn from(element: $crate_tpe) -> Self {
                let element: web_sys::Element = element.inner.unchecked_into();

                element.into()
            }
        }

        impl From<$crate_tpe> for GenericNode {
            fn from(element: $crate_tpe) -> Self {
                let element: web_sys::Node = element.inner.unchecked_into();

                element.into()
            }
        }
    };
    ($tpe:ident) => {
        impl_html_common_traits!($tpe, $tpe);
    };
}

// TODO: HtmlUnknownElement. Any use-cases where it makes sense in addition to GenericHtmlElement?

// TODO: include wrapper types for all HTML elements, even ones that don't have any additional
// interface over GenericHtmlElement? This would eliminate the risk that the addition of a new
// attribute to an existing element in a future spec would require a breaking change. Currently we
// do include types that are in web_sys that only have deprecated attributes/methods, as even
// though we won't expose deprecated attributes/methods, it may still be useful to be able to drop
// into the web_sys type in a type-safe fashion to access such attributes/methods when necessary.

mod html_anchor_element;
pub use self::html_anchor_element::*;

mod html_area_element;
pub use self::html_area_element::*;

mod html_audio_element;
pub use self::html_audio_element::*;

mod html_base_element;
pub use self::html_base_element::*;

mod html_body_element;
pub use self::html_body_element::*;

mod html_br_element;
pub use self::html_br_element::*;

mod html_button_element;
pub use self::html_button_element::*;

mod html_canvas_element;
pub use self::html_canvas_element::*;

mod html_data_element;
pub use self::html_data_element::*;

mod html_data_list_element;
pub use self::html_data_list_element::*;

mod html_details_element;
pub use self::html_details_element::*;

mod html_dialog_element;
pub use self::html_dialog_element::*;

mod html_div_element;
pub use self::html_div_element::*;

mod html_dlist_element;
pub use self::html_dlist_element::*;

mod html_document;
pub use self::html_document::*;

mod html_element;
pub use self::html_element::*;

mod html_embed_element;
pub use self::html_embed_element::*;

mod html_field_set_element;
pub use self::html_field_set_element::*;

mod html_form_element;
pub use self::html_form_element::*;

mod html_head_element;
pub use self::html_head_element::*;

mod html_heading_element;
pub use self::html_heading_element::*;

mod html_hr_element;
pub use self::html_hr_element::*;

mod html_html_element;
pub use self::html_html_element::*;

mod html_iframe_element;
pub use self::html_iframe_element::*;

mod html_image_element;
pub use self::html_image_element::*;

mod html_input_element;
pub use self::html_input_element::*;

mod html_label_element;
pub use self::html_label_element::*;

mod html_legend_element;
pub use self::html_legend_element::*;

mod html_li_element;
pub use self::html_li_element::*;

mod html_link_element;
pub use self::html_link_element::*;

mod html_map_element;
pub use self::html_map_element::*;

mod html_media_element;
pub use self::html_media_element::*;

mod html_meta_element;
pub use self::html_meta_element::*;

mod html_meter_element;
pub use self::html_meter_element::*;

mod html_mod_element;
pub use self::html_mod_element::*;

mod html_object_element;
pub use self::html_object_element::*;

mod html_olist_element;
pub use self::html_olist_element::*;

mod html_opt_group_element;
pub use self::html_opt_group_element::*;

mod html_option_element;
pub use self::html_option_element::*;

mod html_output_element;
pub use self::html_output_element::*;

mod html_paragraph_element;
pub use self::html_paragraph_element::*;

mod html_param_element;
pub use self::html_param_element::*;

mod html_picture_element;
pub use self::html_picture_element::*;

mod html_pre_element;
pub use self::html_pre_element::*;

mod html_progress_element;
pub use self::html_progress_element::*;

mod html_quote_element;
pub use self::html_quote_element::*;

mod html_script_element;
pub use self::html_script_element::*;

mod html_select_element;
pub use self::html_select_element::*;

mod html_slot_element;
pub use self::html_slot_element::*;

mod html_source_element;
pub use self::html_source_element::*;

mod html_span_element;
pub use self::html_span_element::*;

mod html_style_element;
pub use self::html_style_element::*;

mod html_table_caption_element;
pub use self::html_table_caption_element::*;

mod html_table_cell_element;
pub use self::html_table_cell_element::*;

mod html_table_col_element;
pub use self::html_table_col_element::*;

mod html_table_element;
pub use self::html_table_element::*;

mod html_table_row_element;
pub use self::html_table_row_element::*;

mod html_table_section_element;
pub use self::html_table_section_element::*;

mod html_template_element;
pub use self::html_template_element::*;

mod html_text_area_element;
pub use self::html_text_area_element::*;

mod html_time_element;
pub use self::html_time_element::*;

mod html_title_element;
pub use self::html_title_element::*;

mod html_track_element;
pub use self::html_track_element::*;

mod html_ulist_element;
pub use self::html_ulist_element::*;

mod html_video_element;
pub use self::html_video_element::*;

mod labels;
pub use self::labels::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AutoComplete {
    On,
    Off,
}

impl Default for AutoComplete {
    fn default() -> Self {
        AutoComplete::On
    }
}
