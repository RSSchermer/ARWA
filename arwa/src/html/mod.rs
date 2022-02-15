mod constraint_validation;
pub use self::constraint_validation::*;

mod form_listed_element;
pub use self::form_listed_element::*;

mod form_submitter;
pub use self::form_submitter::*;

mod generic_html_elements;
pub use self::generic_html_elements::*;

mod html_a_element;
pub use self::html_a_element::*;

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

mod html_datalist_element;
pub use self::html_datalist_element::*;

mod html_details_element;
pub use self::html_details_element::*;

mod html_dialog_element;
pub use self::html_dialog_element::*;

mod html_div_element;
pub use self::html_div_element::*;

mod html_dl_element;
pub use self::html_dl_element::*;

mod html_document;
pub use self::html_document::*;

mod html_element;
pub use self::html_element::*;

mod html_embed_element;
pub use self::html_embed_element::*;

mod html_fieldset_element;
pub use self::html_fieldset_element::*;

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

mod html_img_element;
pub use self::html_img_element::*;

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

mod html_menu_element;
pub use self::html_menu_elementt::*;

mod html_meta_element;
pub use self::html_meta_element::*;

mod html_meter_element;
pub use self::html_meter_element::*;

mod html_mod_element;
pub use self::html_mod_element::*;

mod html_object_element;
pub use self::html_object_element::*;

mod html_ol_element;
pub use self::html_ol_element::*;

mod html_optgroup_element;
pub use self::html_optgroup_element::*;

mod html_option_element;
pub use self::html_option_element::*;

mod html_output_element;
pub use self::html_output_element::*;

mod html_p_element;
pub use self::html_p_element::*;

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

mod html_caption_element;
pub use self::html_caption_element::*;

mod html_table_cell_element;
pub use self::html_table_cell_element::*;

mod html_colgroup_element;
pub use self::html_colgroup_element::*;

mod html_table_element;
pub use self::html_table_element::*;

mod html_tr_element;
pub use self::html_tr_element::*;

mod html_table_section_element;
pub use self::html_table_section_element::*;

mod html_template_element;
pub use self::html_template_element::*;

mod html_textarea_element;
pub use self::html_textarea_element::*;

mod html_time_element;
pub use self::html_time_element::*;

mod html_title_element;
pub use self::html_title_element::*;

mod html_track_element;
pub use self::html_track_element::*;

mod html_ul_element;
pub use self::html_ul_element::*;

mod html_video_element;
pub use self::html_video_element::*;

mod labelable_element;
pub use self::labelable_element::*;

mod link_types;
pub use self::link_types::*;

mod media_element;
pub use self::media_element::*;

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
