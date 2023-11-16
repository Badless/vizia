use super::internal;
use crate::{prelude::*, style::SystemFlags};
use cosmic_text::FamilyOwned;
use vizia_style::{FontSize, FontStretch, FontStyle, FontWeight};

/// Modifiers for changing the text properties of a view.
pub trait TextModifiers: internal::Modifiable {
    /// Sets the text content of the view.
    fn text<T: ToStringLocalized>(mut self, value: impl Res<T>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, val| {
                let cx: &mut EventContext<'_> = &mut EventContext::new_with_current(cx, cx.current);
                let text_data = val.get(cx).to_string_local(cx);
                cx.text_context.set_text(cx.current, &text_data);

                cx.style.needs_text_layout.insert(cx.current, true);
                cx.needs_relayout();
                cx.needs_redraw();
            });
        });

        self
    }

    modifier!(
        /// Sets the font that should be used by the view.
        ///
        /// The font name refers to the name assigned when the font is added to context.
        font_family,
        Vec<FamilyOwned>,
        SystemFlags::REFLOW
    );

    modifier!(
        /// Sets the font weight that should be used by the view.
        font_weight,
        FontWeight,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the font style that should be used by the view.
        font_style,
        FontStyle,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the font stretch that should be used by the view if the font supports it.
        font_stretch,
        FontStretch,
        SystemFlags::REDRAW
    );

    /// Sets the text color of the view.
    fn color<U: Clone + Into<Color>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, |cx, v| {
                cx.style.font_color.insert(cx.current, v.get(cx).into());
                cx.style.needs_redraw();
            });
        });
        self
    }

    /// Sets the font size of the view.
    fn font_size<U: Into<FontSize>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        let current = self.current();
        self.context().with_current(current, |cx| {
            value.set_or_bind(cx, entity, move |cx, v| {
                cx.style.font_size.insert(cx.current, v.get(cx).into());
                cx.style.needs_text_layout.insert(cx.current, true);
            });
        });
        self
    }

    modifier!(
        /// Sets the ext caret color of the view.
        caret_color,
        Color,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the color used to highlight selected text within the view.
        selection_color,
        Color,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets whether the text of the view should be allowed to wrap.
        text_wrap,
        bool,
        SystemFlags::REFLOW
    );

    modifier!(
        /// Sets the horizontal alignment of text within the view.
        text_align,
        TextAlign,
        SystemFlags::REDRAW
    );
}

impl<'a, V> TextModifiers for Handle<'a, V> {}
