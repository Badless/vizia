use vizia_style::{BorderRadius, Rect, Transform};

use super::internal;
use crate::prelude::*;
use crate::style::SystemFlags;

/// Modifiers for changing the style properties of a view.
pub trait StyleModifiers: internal::Modifiable {
    // Selectors

    /// Sets the ID name of the view.
    ///
    /// The ID name can be references by a CSS selector.
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// Element::new(cx).id("foo");
    /// ```
    /// css
    /// ```css
    /// #foo {
    ///     background-color: red;
    /// }
    ///```
    fn id(mut self, id: impl Into<String>) -> Self {
        let id = id.into();
        let entity = self.entity();
        self.context().style.ids.insert(entity, id.clone()).expect("Could not insert id");
        self.context().needs_restyle();

        self.context().entity_identifiers.insert(id, entity);

        self
    }

    /// Adds a class name to the view.
    fn class(mut self, name: &str) -> Self {
        let entity = self.entity();
        if let Some(class_list) = self.context().style.classes.get_mut(entity) {
            class_list.insert(name.to_string());
        }

        self.context().needs_restyle();

        self
    }

    /// Sets whether a view should have the given class name.
    fn toggle_class(mut self, name: &str, applied: impl Res<bool>) -> Self {
        let name = name.to_owned();
        let entity = self.entity();
        applied.set_or_bind(self.context(), entity, move |cx, entity, applied| {
            if let Some(class_list) = cx.style.classes.get_mut(entity) {
                if applied {
                    class_list.insert(name.clone());
                } else {
                    class_list.remove(&name);
                }
            }

            cx.needs_restyle();
        });

        self
    }

    // PseudoClassFlags
    // TODO: Should these have their own modifiers trait?

    /// Sets the state of the view to checked.
    fn checked<U: Into<bool>>(mut self, state: impl Res<U>) -> Self {
        let entity = self.entity();
        state.set_or_bind(self.context(), entity, |cx, entity, val| {
            let val = val.into();
            if let Some(pseudo_classes) = cx.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClassFlags::CHECKED, val.into());
            } else {
                let mut pseudo_class_flags = PseudoClassFlags::empty();
                pseudo_class_flags.set(PseudoClassFlags::CHECKED, val.into());
                cx.style.pseudo_classes.insert(entity, pseudo_class_flags).unwrap();
            }

            if val {
                // Setting a checked state should make it checkable... probably
                if let Some(abilities) = cx.style.abilities.get_mut(entity) {
                    abilities.set(Abilities::CHECKABLE, true);
                } else {
                    let mut abilities = Abilities::empty();
                    abilities.set(Abilities::CHECKABLE, true);
                    cx.style.abilities.insert(entity, abilities).unwrap();
                }
            }

            cx.needs_restyle();
        });

        self
    }

    modifier!(
        /// Sets the view to be disabled.
        ///
        /// This property is inherited by the descendants of the view.
        disabled,
        bool,
        SystemFlags::RESTYLE
    );

    modifier!(
        /// Sets whether the view should be positioned and rendered.
        ///
        /// A display value of `Display::None` causes the view to be ignored by both layout and rendering.
        display,
        Display,
        SystemFlags::RELAYOUT | SystemFlags::REDRAW
    );

    modifier!(
        /// Sets whether the view should be rendered.
        ///
        /// The layout system will still compute the size and position of an invisible view.
        visibility,
        Visibility,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the opacity of the view.
        opacity,
        Opacity,
        SystemFlags::REDRAW
    );

    /// Sets the z-order index of the view.
    ///
    /// Views with a higher z-order will be rendered on top of those with a lower z-order.
    /// Views with the same z-order are rendered in tree order.
    fn z_order<U: Into<i32>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.tree.set_z_order(entity, value);
            cx.needs_redraw();
        });

        self
    }

    fn overflow<U: Into<Overflow>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.overflowx.insert(entity, value);
            cx.style.overflowy.insert(entity, value);

            cx.needs_redraw();
        });

        self
    }

    modifier!(
        /// Sets the overflow behavior of the view in the horizontal direction.
        ///
        /// The overflow behavior determines whether child views can render outside the bounds of their parent.
        overflowx,
        Overflow,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the overflow behavior of the view in the vertical direction.
        ///
        /// The overflow behavior determines whether child views can render outside the bounds of their parent.
        overflowy,
        Overflow,
        SystemFlags::REDRAW
    );

    // Background Properties
    modifier!(
        /// Sets the background color of the view.
        background_color,
        Color,
        SystemFlags::REDRAW
    );

    fn background_image<'i, U: Into<Vec<BackgroundImage<'i>>>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, val| {
            let images = val.into();
            let gradients = images
                .into_iter()
                .filter_map(|img| match img {
                    BackgroundImage::Gradient(gradient) => Some(*gradient),
                    _ => None,
                })
                .collect::<Vec<_>>();
            cx.style.background_gradient.insert(entity, gradients);
            cx.needs_redraw();
        });

        self
    }

    // TODO: Docs for this.
    fn image<U: ToString>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, val| {
            let val = val.to_string();
            if let Some(prev_data) = cx.style.image.get(entity) {
                if prev_data != &val {
                    cx.style.image.insert(entity, val);
                    cx.style.needs_text_layout.insert(entity, true).unwrap();
                    cx.needs_redraw();
                }
            } else {
                cx.style.image.insert(entity, val);
                cx.style.needs_text_layout.insert(entity, true).unwrap();
                cx.needs_redraw();
            }
        });

        self
    }

    // Border Properties
    modifier!(
        /// Sets the border width of the view.
        border_width,
        LengthOrPercentage,
        SystemFlags::RELAYOUT | SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border color of the view.
        border_color,
        Color,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border radius for the top-left corner of the view.
        border_top_left_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border radius for the top-right corner of the view.
        border_top_right_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border radius for the bottom-left corner of the view.
        border_bottom_left_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border radius for the bottom-right corner of the view.
        border_bottom_right_radius,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    /// Sets the border radius for all four corners of the view.
    fn border_radius<U: std::fmt::Debug + Into<BorderRadius>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.border_top_left_radius.insert(entity, value.top_left);
            cx.style.border_top_right_radius.insert(entity, value.top_right);
            cx.style.border_bottom_left_radius.insert(entity, value.bottom_left);
            cx.style.border_bottom_right_radius.insert(entity, value.bottom_right);

            cx.needs_redraw();
        });

        self
    }

    modifier!(
        /// Sets the border corner shape for the top-left corner of the view.
        border_top_left_shape,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border corner shape for the top-right corner of the view.
        border_top_right_shape,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border corner shape for the bottom-left corner of the view.
        border_bottom_left_shape,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the border corner shape for the bottom-right corner of the view.
        border_bottom_right_shape,
        BorderCornerShape,
        SystemFlags::REDRAW
    );

    /// Sets the border corner shape for all four corners of the view.
    fn border_corner_shape<U: std::fmt::Debug + Into<Rect<BorderCornerShape>>>(
        mut self,
        value: impl Res<U>,
    ) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.border_top_left_shape.insert(entity, value.0);
            cx.style.border_top_right_shape.insert(entity, value.1);
            cx.style.border_bottom_right_shape.insert(entity, value.2);
            cx.style.border_bottom_left_shape.insert(entity, value.3);

            cx.needs_redraw();
        });

        self
    }

    // Outine Properties
    modifier!(
        /// Sets the outline width of the view.
        outline_width,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the outline color of the view.
        outline_color,
        Color,
        SystemFlags::REDRAW
    );
    modifier!(
        /// Sets the outline offset of the view.
        outline_offset,
        LengthOrPercentage,
        SystemFlags::REDRAW
    );

    modifier!(
        /// Sets the mouse cursor used when the view is hovered.
        cursor,
        CursorIcon,
        SystemFlags::empty()
    );

    fn transform<U: Into<Vec<Transform>>>(mut self, value: impl Res<U>) -> Self {
        let entity = self.entity();
        value.set_or_bind(self.context(), entity, |cx, entity, v| {
            let value = v.into();
            cx.style.transform.insert(entity, value);
            cx.needs_redraw();
        });

        self
    }

    // // Transform Properties
    // modifier!(
    //     /// Sets the angle of rotation for the view.
    //     ///
    //     /// Rotation applies to the rendered view and does not affect layout.
    //     rotate,
    //     f32
    // );
    // modifier!(
    //     /// Sets the translation offset of the view.
    //     ///
    //     /// Translation applies to the rendered view and does not affect layout.
    //     translate,
    //     (f32, f32)
    // );
    // modifier!(
    //     /// Sets the scale of the view.
    //     ///
    //     /// Scale applies to the rendered view and does not affect layout.
    //     scale,
    //     (f32, f32)
    // );
}

impl<'a, V: View> StyleModifiers for Handle<'a, V> {}
