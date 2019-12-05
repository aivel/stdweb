use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};

/// The HTML `<textarea>` element represents a multi-line plain-text editing control.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/HTML/Element/textarea)
// https://html.spec.whatwg.org/#htmltextareaelement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLTextAreaElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct TextAreaElement( Reference );

impl IEventTarget for TextAreaElement {}
impl INode for TextAreaElement {}
impl IElement for TextAreaElement {}
impl IHtmlElement for TextAreaElement {}

impl TextAreaElement {
    /// The value of the control.
    // https://html.spec.whatwg.org/#the-textarea-element:dom-textarea-value
    #[inline]
    pub fn value( &self ) -> String {
        js! (
            return @{self}.value;
        ).try_into().unwrap()
    }

    /// Sets the value of the control.
    // https://html.spec.whatwg.org/#the-textarea-element:dom-textarea-value
    #[inline]
    pub fn set_value( &self, value: &str ) {
        js! { @(no_return)
            @{self}.value = @{value};
        }
    }
    
    /// The offset to the start of the selection.
    // https://html.spec.whatwg.org/#dom-textarea/input-selectionstart
    #[inline]
    pub fn selection_start( &self ) -> u32 {
        js! (
            return @{self}.selectionStart;
        ).try_into().ok()
    }

    /// Sets the offset to the start of the selection.
    // https://html.spec.whatwg.org/#dom-textarea/input-selectionstart
    #[inline]
    pub fn set_selection_start( &self, value: u32 ) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{self}.selectionStart = @{value};
        ).unwrap()
    }

    /// The offset to the end of the selection.
    // https://html.spec.whatwg.org/#dom-textarea/input-selectionend
    #[inline]
    pub fn selection_end( &self ) -> u32 {
        js! (
            return @{self}.selectionEnd;
        ).try_into().ok()
    }

    /// Sets the offset to the end of the selection.
    // https://html.spec.whatwg.org/#dom-textarea/input-selectionend
    #[inline]
    pub fn set_selection_end( &self, value: u32 ) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{self}.selectionEnd = @{value};
        ).unwrap()
    }
}
