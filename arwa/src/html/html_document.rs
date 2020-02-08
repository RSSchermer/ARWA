use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::html::{
    GenericHtmlElement, HtmlAnchorElement, HtmlAreaElement, HtmlAudioElement, HtmlBaseElement,
    HtmlBodyElement, HtmlBrElement, HtmlButtonElement, HtmlCanvasElement, HtmlDListElement,
    HtmlDataElement, HtmlDataListElement, HtmlDetailsElement, HtmlDialogElement, HtmlDivElement,
    HtmlEmbedElement, HtmlFieldSetElement, HtmlFormElement, HtmlHeadElement, HtmlHeadingElement,
    HtmlHrElement, HtmlHtmlElement, HtmlIFrameElement, HtmlImageElement, HtmlInputElement,
    HtmlLabelElement, HtmlLegendElement, HtmlLiElement, HtmlLinkElement, HtmlMapElement,
    HtmlMetaElement, HtmlMeterElement, HtmlModElement, HtmlOListElement, HtmlObjectElement,
    HtmlOptGroupElement, HtmlOptionElement, HtmlOutputElement, HtmlParagraphElement,
    HtmlParamElement, HtmlPictureElement, HtmlPreElement, HtmlProgressElement, HtmlQuoteElement,
    HtmlScriptElement, HtmlSelectElement, HtmlSlotElement, HtmlSourceElement, HtmlSpanElement,
    HtmlStyleElement, HtmlTableCaptionElement, HtmlTableCellElement, HtmlTableColElement,
    HtmlTableElement, HtmlTableRowElement, HtmlTableSectionElement, HtmlTemplateElement,
    HtmlTextAreaElement, HtmlTimeElement, HtmlTitleElement, HtmlTrackElement, HtmlUListElement,
    HtmlVideoElement,
};
use crate::{Document, GlobalEventHandlers, Node};

pub struct HtmlDocument {
    inner: web_sys::HtmlDocument,
}

impl HtmlDocument {
    delegate! {
        target self.inner {

        }
    }

    // TODO: cookie

    // TODO: default_view

    pub fn design_mode_enabled(&self) -> bool {
        match &*self.inner.design_mode() {
            "on" => true,
            _ => false,
        }
    }

    pub fn set_design_mode_enabled(&self, design_mode_enabled: bool) {
        let design_mode = if design_mode_enabled { "on" } else { "off" };

        self.inner.set_design_mode(design_mode);
    }

    pub fn create_a_element(&self) -> HtmlAnchorElement {
        let element: web_sys::HtmlAnchorElement =
            self.inner.create_element("a").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_abbr_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("abbr").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_address_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("address")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_area_element(&self) -> HtmlAreaElement {
        let element: web_sys::HtmlAreaElement =
            self.inner.create_element("area").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_article_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("article")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_aside_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("aside").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_audio_element(&self) -> HtmlAudioElement {
        let element: web_sys::HtmlAudioElement =
            self.inner.create_element("audio").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_b_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("b").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_base_element(&self) -> HtmlBaseElement {
        let element: web_sys::HtmlBaseElement =
            self.inner.create_element("base").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_bdi_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("bdi").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_bdo_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("bdo").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_blockquote_element(&self) -> HtmlQuoteElement {
        let element: web_sys::HtmlQuoteElement = self
            .inner
            .create_element("blockquote")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_body_element(&self) -> HtmlBodyElement {
        let element: web_sys::HtmlBodyElement =
            self.inner.create_element("body").unwrap().unchecked_into();

        HtmlBodyElement::from(element)
    }

    pub fn create_br_element(&self) -> HtmlBrElement {
        let element: web_sys::HtmlBrElement =
            self.inner.create_element("br").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_button_element(&self) -> HtmlButtonElement {
        let element: web_sys::HtmlButtonElement = self
            .inner
            .create_element("button")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_canvas_element(&self) -> HtmlCanvasElement {
        let element: web_sys::HtmlCanvasElement = self
            .inner
            .create_element("canvas")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_caption_element(&self) -> HtmlTableCaptionElement {
        let element: web_sys::HtmlTableCaptionElement = self
            .inner
            .create_element("caption")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_cite_element(&self) -> HtmlQuoteElement {
        let element: web_sys::HtmlQuoteElement =
            self.inner.create_element("cite").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_code_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("code").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_col_element(&self) -> HtmlTableColElement {
        let element: web_sys::HtmlTableColElement =
            self.inner.create_element("col").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_colgroup_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("colgroup")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_data_element(&self) -> HtmlDataElement {
        let element: web_sys::HtmlDataElement =
            self.inner.create_element("data").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_datalist_element(&self) -> HtmlDataListElement {
        let element: web_sys::HtmlDataListElement = self
            .inner
            .create_element("datalist")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_dd_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("dd").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_del_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("del").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_details_element(&self) -> HtmlDetailsElement {
        let element: web_sys::HtmlDetailsElement = self
            .inner
            .create_element("details")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_dfn_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("dfn").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_dialog_element(&self) -> HtmlDialogElement {
        let element: web_sys::HtmlDialogElement = self
            .inner
            .create_element("dialog")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_div_element(&self) -> HtmlDivElement {
        let element: web_sys::HtmlDivElement =
            self.inner.create_element("div").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_dl_element(&self) -> HtmlDListElement {
        let element: web_sys::HtmlDListElement =
            self.inner.create_element("dl").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_dt_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("dt").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_em_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("em").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_embed_element(&self) -> HtmlEmbedElement {
        let element: web_sys::HtmlEmbedElement =
            self.inner.create_element("embed").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_fieldset_element(&self) -> HtmlFieldSetElement {
        let element: web_sys::HtmlFieldSetElement = self
            .inner
            .create_element("fieldset")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_figcaption_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("figcaption")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_figure_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("figure")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_form_element(&self) -> HtmlFormElement {
        let element: web_sys::HtmlFormElement =
            self.inner.create_element("form").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_footer_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("footer")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_h1_element(&self) -> HtmlHeadingElement {
        let element: web_sys::HtmlHeadingElement =
            self.inner.create_element("h1").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_h2_element(&self) -> HtmlHeadingElement {
        let element: web_sys::HtmlHeadingElement =
            self.inner.create_element("h2").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_h3_element(&self) -> HtmlHeadingElement {
        let element: web_sys::HtmlHeadingElement =
            self.inner.create_element("h3").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_h4_element(&self) -> HtmlHeadingElement {
        let element: web_sys::HtmlHeadingElement =
            self.inner.create_element("h4").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_h5_element(&self) -> HtmlHeadingElement {
        let element: web_sys::HtmlHeadingElement =
            self.inner.create_element("h5").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_h6_element(&self) -> HtmlHeadingElement {
        let element: web_sys::HtmlHeadingElement =
            self.inner.create_element("h6").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_head_element(&self) -> HtmlHeadElement {
        let element: web_sys::HtmlHeadElement =
            self.inner.create_element("head").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_header_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("header")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_hgroup_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("hgroup")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_hr_element(&self) -> HtmlHrElement {
        let element: web_sys::HtmlHrElement =
            self.inner.create_element("hr").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_html_element(&self) -> HtmlHtmlElement {
        let element: web_sys::HtmlHtmlElement =
            self.inner.create_element("html").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_i_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("i").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_iframe_element(&self) -> HtmlIFrameElement {
        let element: web_sys::HtmlIFrameElement = self
            .inner
            .create_element("iframe")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_img_element(&self) -> HtmlImageElement {
        let element: web_sys::HtmlImageElement =
            self.inner.create_element("img").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_input_element(&self) -> HtmlInputElement {
        let element: web_sys::HtmlInputElement =
            self.inner.create_element("input").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_ins_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("ins").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_kbd_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("kbd").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_label_element(&self) -> HtmlLabelElement {
        let element: web_sys::HtmlLabelElement =
            self.inner.create_element("label").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_legend_element(&self) -> HtmlLegendElement {
        let element: web_sys::HtmlLegendElement = self
            .inner
            .create_element("legend")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_li_element(&self) -> HtmlLiElement {
        let element: web_sys::HtmlLiElement =
            self.inner.create_element("li").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_link_element(&self) -> HtmlLinkElement {
        let element: web_sys::HtmlLinkElement =
            self.inner.create_element("link").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_main_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("main").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_map_element(&self) -> HtmlMapElement {
        let element: web_sys::HtmlMapElement =
            self.inner.create_element("map").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_mark_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("mark").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_menu_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("menu").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_meta_element(&self) -> HtmlMetaElement {
        let element: web_sys::HtmlMetaElement =
            self.inner.create_element("meta").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_meter_element(&self) -> HtmlMeterElement {
        let element: web_sys::HtmlMeterElement =
            self.inner.create_element("meter").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_mod_element(&self) -> HtmlModElement {
        let element: web_sys::HtmlModElement =
            self.inner.create_element("mod").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_nav_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("nav").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_noscript_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("noscript")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_object_element(&self) -> HtmlObjectElement {
        let element: web_sys::HtmlObjectElement = self
            .inner
            .create_element("object")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_ol_element(&self) -> HtmlOListElement {
        let element: web_sys::HtmlOListElement =
            self.inner.create_element("ol").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_optgroup_element(&self) -> HtmlOptGroupElement {
        let element: web_sys::HtmlOptGroupElement = self
            .inner
            .create_element("optgroup")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_option_element(&self) -> HtmlOptionElement {
        let element: web_sys::HtmlOptionElement = self
            .inner
            .create_element("option")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_output_element(&self) -> HtmlOutputElement {
        let element: web_sys::HtmlOutputElement = self
            .inner
            .create_element("output")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_p_element(&self) -> HtmlParagraphElement {
        let element: web_sys::HtmlParagraphElement =
            self.inner.create_element("p").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_param_element(&self) -> HtmlParamElement {
        let element: web_sys::HtmlParamElement =
            self.inner.create_element("param").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_picture_element(&self) -> HtmlPictureElement {
        let element: web_sys::HtmlPictureElement = self
            .inner
            .create_element("picture")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_pre_element(&self) -> HtmlPreElement {
        let element: web_sys::HtmlPreElement =
            self.inner.create_element("pre").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_progress_element(&self) -> HtmlProgressElement {
        let element: web_sys::HtmlProgressElement = self
            .inner
            .create_element("progress")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_q_element(&self) -> HtmlQuoteElement {
        let element: web_sys::HtmlQuoteElement =
            self.inner.create_element("q").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_rb_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("rb").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_rt_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("rt").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_rtc_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("rtc").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_ruby_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("ruby").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_s_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("s").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_samp_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("samp").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_script_element(&self) -> HtmlScriptElement {
        let element: web_sys::HtmlScriptElement = self
            .inner
            .create_element("script")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_small_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("small").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_section_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("section")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_select_element(&self) -> HtmlSelectElement {
        let element: web_sys::HtmlSelectElement = self
            .inner
            .create_element("select")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_slot_element(&self) -> HtmlSlotElement {
        let element: web_sys::HtmlSlotElement =
            self.inner.create_element("slot").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_source_element(&self) -> HtmlSourceElement {
        let element: web_sys::HtmlSourceElement = self
            .inner
            .create_element("source")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_span_element(&self) -> HtmlSpanElement {
        let element: web_sys::HtmlSpanElement =
            self.inner.create_element("span").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_strong_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("strong")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_style_element(&self) -> HtmlStyleElement {
        let element: web_sys::HtmlStyleElement =
            self.inner.create_element("style").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_sub_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("sub").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_summary_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement = self
            .inner
            .create_element("summary")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_sup_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("sup").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_td_element(&self) -> HtmlTableCellElement {
        let element: web_sys::HtmlTableCellElement =
            self.inner.create_element("td").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_th_element(&self) -> HtmlTableCellElement {
        let element: web_sys::HtmlTableCellElement =
            self.inner.create_element("th").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_table_element(&self) -> HtmlTableElement {
        let element: web_sys::HtmlTableElement =
            self.inner.create_element("table").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_tr_element(&self) -> HtmlTableRowElement {
        let element: web_sys::HtmlTableRowElement =
            self.inner.create_element("tr").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_thead_element(&self) -> HtmlTableSectionElement {
        let element: web_sys::HtmlTableSectionElement =
            self.inner.create_element("thead").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_tbody_element(&self) -> HtmlTableSectionElement {
        let element: web_sys::HtmlTableSectionElement =
            self.inner.create_element("tbody").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_tfoot_element(&self) -> HtmlTableSectionElement {
        let element: web_sys::HtmlTableSectionElement =
            self.inner.create_element("tfoot").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_template_element(&self) -> HtmlTemplateElement {
        let element: web_sys::HtmlTemplateElement = self
            .inner
            .create_element("template")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_textarea_element(&self) -> HtmlTextAreaElement {
        let element: web_sys::HtmlTextAreaElement = self
            .inner
            .create_element("textarea")
            .unwrap()
            .unchecked_into();

        element.into()
    }

    pub fn create_time_element(&self) -> HtmlTimeElement {
        let element: web_sys::HtmlTimeElement =
            self.inner.create_element("time").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_title_element(&self) -> HtmlTitleElement {
        let element: web_sys::HtmlTitleElement =
            self.inner.create_element("title").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_track_element(&self) -> HtmlTrackElement {
        let element: web_sys::HtmlTrackElement =
            self.inner.create_element("track").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_u_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("u").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_ul_element(&self) -> HtmlUListElement {
        let element: web_sys::HtmlUListElement =
            self.inner.create_element("ul").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_var_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("var").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_video_element(&self) -> HtmlVideoElement {
        let element: web_sys::HtmlVideoElement =
            self.inner.create_element("video").unwrap().unchecked_into();

        element.into()
    }

    pub fn create_wbr_element(&self) -> GenericHtmlElement {
        let element: web_sys::HtmlElement =
            self.inner.create_element("wbr").unwrap().unchecked_into();

        element.into()
    }
}

impl From<web_sys::HtmlDocument> for HtmlDocument {
    fn from(inner: web_sys::HtmlDocument) -> Self {
        HtmlDocument { inner }
    }
}

impl AsRef<web_sys::HtmlDocument> for HtmlDocument {
    fn as_ref(&self) -> &web_sys::HtmlDocument {
        &self.inner
    }
}

impl AsRef<web_sys::Document> for HtmlDocument {
    fn as_ref(&self) -> &web_sys::Document {
        self.inner.as_ref()
    }
}

impl AsRef<web_sys::Node> for HtmlDocument {
    fn as_ref(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}

impl AsRef<web_sys::EventTarget> for HtmlDocument {
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl Write for HtmlDocument {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl GlobalEventHandlers for HtmlDocument {}
impl Node for HtmlDocument {}
impl Document for HtmlDocument {}
