use std::marker;

use wasm_bindgen::UnwrapThrowExt;

use crate::collection::{Collection, Sequence};
use crate::dom::TextDirectionality;
use crate::file::File;
use crate::url::Url;

pub enum InputType {
    /// String describing the type of a non-standardized input event.
    NonStandard(String),

    /// Insert typed plain text.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertText,

    /// Replace existing text by means of a spell checker, auto-correct or similar.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertReplacementText,

    /// Insert a line break.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertLineBreak,

    /// Insert a paragraph break.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertParagraph,

    /// Insert a numbered list.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertOrderedList,

    /// Insert a bulleted list.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertUnorderedList,

    /// Insert a horizontal rule.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertHorizontalRule,

    /// Replace the current selection with content stored in a
    /// [kill buffer](https://w3c.github.io/input-events/#dfn-kill-buffer).
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertFromYank,

    /// Insert content by means of drop.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertFromDrop,

    /// Paste content from clipboard or paste image from client provided image library.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertFromPaste,

    /// Paste content from the clipboard as a quotation.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertFromPasteAsQuotation,

    /// Transpose the last two characters that were entered.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertTranspose,

    /// Replace the current composition string.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | Yes                     | No                       | Any
    InsertCompositionText,

    /// Insert a finalized composed string that will not form part of the next composition string.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | Yes                     | Yes                      | Any
    InsertFromComposition,

    /// Insert a link.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    InsertLink,

    /// Remove a part of the DOM in order to recompose this part using IME.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | Yes                     | Yes                      | Any
    DeleteByComposition,

    /// Delete the current composition string before committing a finalized string to the DOM.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | Yes                     | No                       | Any
    DeleteCompositionText,

    /// Delete a word directly before the caret position.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Collapsed
    DeleteWordBackward,

    /// Delete a word directly after the caret position.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Collapsed
    DeleteWordForward,

    /// Delete from the caret to the nearest visual line break before the caret position.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Collapsed
    DeleteSoftLineBackward,

    /// Delete from the caret to the nearest visual line break after the caret position.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Collapsed
    DeleteSoftLineForward,

    /// Delete from to the nearest visual line break before the caret position to the nearest visual
    /// line break after the caret position.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Collapsed
    DeleteEntireSoftLine,

    /// Delete from the caret to the nearest beginning of a block element or `br` element before the
    /// caret position.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Collapsed
    DeleteHardLineBackward,

    /// Delete from the caret to the nearest end of a block element or `br` element after the caret
    /// position.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Collapsed
    DeleteHardLineForward,

    /// Remove content from the DOM by means of drag.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    DeleteByDrag,

    /// Remove the current selection as part of a cut.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    DeleteByCut,

    /// Delete the selection without specifying the direction of the deletion and this intention is
    /// not covered by another inputType.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Non-collapsed
    DeleteContent,

    /// Delete the content directly before the caret position and this intention is not covered by
    /// another inputType or delete the selection with the selection collapsing to its start after
    /// the deletion.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    DeleteContentBackward,

    /// Delete the content directly after the caret position and this intention is not covered by
    /// another inputType or delete the selection with the selection collapsing to its end after the
    /// deletion.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    DeleteContentForward,

    /// Undo the last editing action.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    HistoryUndo,

    /// To redo the last undone editing action.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    HistoryRedo,

    /// Initiate bold text.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatBold,

    /// Initiate italic text.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatItalic,

    /// Initiate underline text.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatUnderline,

    /// Initiate stricken through text.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatStrikeThrough,

    /// Initiate superscript text.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatSuperscript,

    /// Initiate subscript text.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatSubscript,

    /// Make the current selection fully justified.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatJustifyFull,

    /// Center align the current selection.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatJustifyCenter,

    /// Right align the current selection.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatJustifyRight,

    /// Left align the current selection.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatJustifyLeft,

    /// Indent the current selection.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatIndent,

    /// Outdent the current selection.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatOutdent,

    /// Remove all formatting from the current selection.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatRemove,

    /// Set the text block direction.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatSetBlockTextDirection,

    /// Set the text inline direction.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatSetInlineTextDirection,

    /// Change the background color.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatBackColor,

    /// Change the font color.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatFontColor,

    /// Change the font-family.
    ///
    /// | Part of IME composition | `BeforeInput` cancelable | State of selection
    /// | ----------------------- | ------------------------ | ------------------
    /// | No                      | Yes                      | Any
    FormatFontName,
}

pub(crate) mod user_input_event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_input_event(&self) -> &web_sys::InputEvent;
    }
}

pub trait UserInputEvent: user_input_event_seal::Seal {
    fn input_type(&self) -> InputType {
        let input_type = self.as_web_sys_input_event().input_type();

        match input_type.as_ref() {
            "insertText" => InputType::InsertText,
            "insertReplacementText" => InputType::InsertReplacementText,
            "insertLineBreak" => InputType::InsertLineBreak,
            "insertParagraph" => InputType::InsertParagraph,
            "insertOrderedList" => InputType::InsertOrderedList,
            "insertUnorderedList" => InputType::InsertUnorderedList,
            "insertHorizontalRule" => InputType::InsertHorizontalRule,
            "insertFromYank" => InputType::InsertFromYank,
            "insertFromDrop" => InputType::InsertFromDrop,
            "insertFromPaste" => InputType::InsertFromPaste,
            "insertFromPasteAsQuotation" => InputType::InsertFromPasteAsQuotation,
            "insertTranspose" => InputType::InsertTranspose,
            "insertCompositionText" => InputType::InsertCompositionText,
            "insertFromComposition" => InputType::InsertFromComposition,
            "insertLink" => InputType::InsertLink,
            "deleteByComposition" => InputType::DeleteByComposition,
            "deleteCompositionText" => InputType::DeleteCompositionText,
            "deleteWordBackward" => InputType::DeleteWordBackward,
            "deleteWordForward" => InputType::DeleteWordForward,
            "deleteSoftLineBackward" => InputType::DeleteSoftLineBackward,
            "deleteSoftLineForward" => InputType::DeleteSoftLineForward,
            "deleteEntireSoftLine" => InputType::DeleteEntireSoftLine,
            "deleteHardLineBackward" => InputType::DeleteHardLineBackward,
            "deleteHardLineForward" => InputType::DeleteHardLineForward,
            "deleteByDrag" => InputType::DeleteByDrag,
            "deleteByCut" => InputType::DeleteByCut,
            "deleteContent" => InputType::DeleteContent,
            "deleteContentBackward" => InputType::DeleteContentBackward,
            "deleteContentForward" => InputType::DeleteContentForward,
            "historyUndo" => InputType::HistoryUndo,
            "historyRedo" => InputType::HistoryRedo,
            "formatBold" => InputType::FormatBold,
            "formatItalic" => InputType::FormatItalic,
            "formatUnderline" => InputType::FormatUnderline,
            "formatStrikeThrough" => InputType::FormatStrikeThrough,
            "formatSuperscript" => InputType::FormatSuperscript,
            "formatSubscript" => InputType::FormatSubscript,
            "formatJustifyFull" => InputType::FormatJustifyFull,
            "formatJustifyCenter" => InputType::FormatJustifyCenter,
            "formatJustifyRight" => InputType::FormatJustifyRight,
            "formatJustifyLeft" => InputType::FormatJustifyLeft,
            "formatIndent" => InputType::FormatIndent,
            "formatOutdent" => InputType::FormatOutdent,
            "formatRemove" => InputType::FormatRemove,
            "formatSetBlockTextDirection" => InputType::FormatSetBlockTextDirection,
            "formatSetInlineTextDirection" => InputType::FormatSetInlineTextDirection,
            "formatBackColor" => InputType::FormatBackColor,
            "formatFontColor" => InputType::FormatFontColor,
            "formatFontName" => InputType::FormatFontName,
            _ => InputType::NonStandard(input_type),
        }
    }

    fn data(&self) -> InputEventData {
        fn resolve_dir(data: &str) -> Option<TextDirectionality> {
            match data {
                "ltr" => Some(TextDirectionality::LeftToRight),
                "rtl" => Some(TextDirectionality::RightToLeft),
                "auto" => Some(TextDirectionality::Auto),
                _ => None,
            }
        }

        if let Some(inner) = self.as_web_sys_input_event().data_transfer() {
            InputEventData::Transfer(InputEventDataTransfer { inner })
        } else if let Some(data) = self.as_web_sys_input_event().data() {
            match self.input_type() {
                InputType::FormatSetInlineTextDirection => {
                    InputEventData::TextDirectionChange(resolve_dir(&data))
                }
                InputType::FormatSetBlockTextDirection => {
                    InputEventData::TextDirectionChange(resolve_dir(&data))
                }
                InputType::FormatBackColor => InputEventData::Color(data),
                InputType::FormatFontColor => InputEventData::Color(data),
                // TODO: can an inserted link be a malformed URL? Assume its always a correction
                // URL for now. If this assumption turns out the be wrong, maybe add a
                // `MalformedUrl`, variant to InputEventData.
                InputType::InsertLink => InputEventData::Url(Url::parse(&data).unwrap_throw()),
                _ => InputEventData::PlainText(data),
            }
        } else {
            InputEventData::None
        }
    }

    // TODO:
    // fn target_ranges(&self) -> InputEventRanges {
    //     InputEventRanges::new(self.as_web_sys_input_event().get_target_ranges())
    // }
}

pub enum InputEventData {
    PlainText(String),
    TextDirectionChange(Option<TextDirectionality>),
    Color(String),
    FontName(String),
    Url(Url),
    Transfer(InputEventDataTransfer),
    None,
}

#[derive(Clone)]
pub struct InputEventDataTransfer {
    inner: web_sys::DataTransfer,
}

impl InputEventDataTransfer {
    pub fn text_plain(&self) -> Option<String> {
        self.inner.get_data("text/plain").ok()
    }

    pub fn text_html(&self) -> Option<String> {
        self.inner.get_data("text/html").ok()
    }

    pub fn text_uri_list(&self) -> Option<String> {
        self.inner.get_data("text/uri-list").ok()
    }

    pub fn files(&self) -> InputEventFiles {
        InputEventFiles {
            inner: self.inner.files().unwrap_throw(),
        }
    }
}

#[derive(Clone)]
pub struct InputEventFiles {
    inner: web_sys::FileList,
}

impl Collection for InputEventFiles {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for InputEventFiles {
    type Item = File;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|f| File::from(f))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

// TODO:
// unchecked_cast_array!(StaticRange, Object, InputEventRanges);

macro_rules! input_event {
    ($event:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $event<T> {
            inner: web_sys::InputEvent,
            _marker: marker::PhantomData<T>,
        }

        impl<T> user_input_event_seal::Seal for $event<T> {
            fn as_web_sys_input_event(&self) -> &web_sys::InputEvent {
                &self.inner
            }
        }

        impl<T> UserInputEvent for $event<T> {}

        impl<T> AsRef<web_sys::InputEvent> for $event<T> {
            fn as_ref(&self) -> &web_sys::InputEvent {
                use crate::ui::user_input_event_seal::Seal;

                self.as_web_sys_input_event()
            }
        }

        $crate::event::impl_typed_event_traits!($event, InputEvent, $name);
    };
}

input_event!(BeforeInputEvent, "beforeinput");
input_event!(InputEvent, "input");
