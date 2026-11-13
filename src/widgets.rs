//! Elementa-inspired component catalog, Custom Render / Native Minecraft UI bridges, and the Inspector.

use crate::event::{EventResult, UIEvent};
use crate::layout::{BoundingBox, LayoutConstraints, MeasuredSize};
use crate::node::{Component, RenderBackend, TextMeasurer};
use crate::style::{Color, Style};

// ============================================================================
// 1. UIText & UIWrappedText
// ============================================================================

pub struct UIText {
    pub text: String,
    pub scale: f32,
    pub color: Color,
    pub wrap: bool,
    pub style: Style,
    pub bounds: BoundingBox,
    pub measured: MeasuredSize,
}

impl UIText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            scale: 1.0,
            color: [255, 255, 255, 255],
            wrap: false,
            style: Style::default(),
            bounds: BoundingBox::ZERO,
            measured: MeasuredSize::ZERO,
        }
    }

    pub fn with_scale(mut self, _scale: f32) -> Self {
        todo!("set self.scale and return self")
    }

    pub fn with_color(mut self, _color: Color) -> Self {
        todo!("set self.color and return self")
    }

    pub fn with_wrap(mut self, _wrap: bool) -> Self {
        todo!("set self.wrap and return self")
    }

    pub fn with_style(mut self, _style: Style) -> Self {
        todo!("set self.style and return self")
    }
}

impl Component for UIText {
    fn name(&self) -> &'static str {
        if self.wrap { "UIWrappedText" } else { "UIText" }
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn bounds(&self) -> BoundingBox {
        self.bounds
    }

    fn measured_size(&self) -> MeasuredSize {
        self.measured
    }

    fn measure(
        &mut self,
        _constraints: LayoutConstraints,
        _measurer: &dyn TextMeasurer,
    ) -> MeasuredSize {
        todo!("use measurer.measure_text(&self.text, self.scale) plus padding/border to compute MeasuredSize")
    }

    fn layout(&mut self, _bounds: BoundingBox) {
        todo!("store self.bounds = bounds")
    }

    fn render(&self, _backend: &mut dyn RenderBackend) {
        todo!("draw background/border if any, then call backend.draw_text at inset origin")
    }

    fn handle_event(&mut self, _event: &mut UIEvent) -> EventResult {
        EventResult::Ignored
    }
}

// ============================================================================
// 2. UIRoundedRectangle
// ============================================================================

pub struct UIRoundedRectangle {
    pub radius: f32,
    pub style: Style,
    pub bounds: BoundingBox,
    pub measured: MeasuredSize,
}

impl UIRoundedRectangle {
    pub fn new(_radius: f32, _style: Style) -> Self {
        todo!("construct UIRoundedRectangle")
    }
}

impl Component for UIRoundedRectangle {
    fn name(&self) -> &'static str {
        "UIRoundedRectangle"
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn bounds(&self) -> BoundingBox {
        self.bounds
    }

    fn measured_size(&self) -> MeasuredSize {
        self.measured
    }

    fn measure(
        &mut self,
        _constraints: LayoutConstraints,
        _measurer: &dyn TextMeasurer,
    ) -> MeasuredSize {
        todo!("resolve style width and height against constraints")
    }

    fn layout(&mut self, _bounds: BoundingBox) {
        todo!("store bounds")
    }

    fn render(&self, _backend: &mut dyn RenderBackend) {
        todo!("call backend.fill_rounded_rect with self.radius")
    }

    fn handle_event(&mut self, _event: &mut UIEvent) -> EventResult {
        EventResult::Ignored
    }
}

// ============================================================================
// 3. ScrollComponent (Elementa Scrollable Container)
// ============================================================================

pub struct ScrollComponent {
    pub style: Style,
    pub bounds: BoundingBox,
    pub measured: MeasuredSize,
    pub scroll_y: f32,
    pub content_height: f32,
    pub children: Vec<Box<dyn Component>>,
}

impl ScrollComponent {
    pub fn new(style: Style) -> Self {
        Self {
            style,
            bounds: BoundingBox::ZERO,
            measured: MeasuredSize::ZERO,
            scroll_y: 0.0,
            content_height: 0.0,
            children: Vec::new(),
        }
    }

    pub fn with_child<C: Component + 'static>(mut self, child: C) -> Self {
        self.children.push(Box::new(child));
        self
    }

    /// Maximum valid `scroll_y` offset so the user cannot scroll past the bottom of the content.
    #[inline]
    pub fn max_scroll(&self) -> f32 {
        todo!("return (self.content_height - inner_height).max(0.0)")
    }
}

impl Component for ScrollComponent {
    fn name(&self) -> &'static str {
        "ScrollComponent"
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn bounds(&self) -> BoundingBox {
        self.bounds
    }

    fn measured_size(&self) -> MeasuredSize {
        self.measured
    }

    fn children(&self) -> &[Box<dyn Component>] {
        &self.children
    }

    fn measure(
        &mut self,
        _constraints: LayoutConstraints,
        _measurer: &dyn TextMeasurer,
    ) -> MeasuredSize {
        todo!("measure children with UNCONSTRAINED max_height, record self.content_height, and clamp outer height to constraints")
    }

    fn layout(&mut self, _bounds: BoundingBox) {
        todo!("layout children and translate their bounds by (0.0, -self.scroll_y)")
    }

    fn render(&self, _backend: &mut dyn RenderBackend) {
        todo!("push_clip(self.bounds), render children, draw scrollbar thumb, pop_clip()")
    }

    fn handle_event(&mut self, _event: &mut UIEvent) -> EventResult {
        todo!("forward event to children first; if MouseWheel inside bounds, adjust self.scroll_y clamped to 0..=max_scroll() and stop_propagation()")
    }
}

// ============================================================================
// 4. UITextInput (Focusable Keyboard Input Box)
// ============================================================================

pub struct UITextInput {
    pub value: String,
    pub placeholder: String,
    pub focused: bool,
    pub style: Style,
    pub bounds: BoundingBox,
    pub measured: MeasuredSize,
}

impl UITextInput {
    pub fn new(placeholder: impl Into<String>, style: Style) -> Self {
        Self {
            value: String::new(),
            placeholder: placeholder.into(),
            focused: false,
            style,
            bounds: BoundingBox::ZERO,
            measured: MeasuredSize::ZERO,
        }
    }
}

impl Component for UITextInput {
    fn name(&self) -> &'static str {
        "UITextInput"
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn bounds(&self) -> BoundingBox {
        self.bounds
    }

    fn measured_size(&self) -> MeasuredSize {
        self.measured
    }

    fn measure(
        &mut self,
        _constraints: LayoutConstraints,
        _measurer: &dyn TextMeasurer,
    ) -> MeasuredSize {
        todo!("measure text or placeholder plus padding/border")
    }

    fn layout(&mut self, _bounds: BoundingBox) {
        todo!("store bounds")
    }

    fn render(&self, _backend: &mut dyn RenderBackend) {
        todo!("render box, border (highlighted if self.focused), and value or placeholder")
    }

    fn handle_event(&mut self, _event: &mut UIEvent) -> EventResult {
        todo!("on MouseDown: set self.focused = self.bounds.contains(x, y); on CharInput/KeyDown when focused: append/backspace self.value")
    }
}

// ============================================================================
// 5. CustomRenderComponent (Bridge for 3D Models, Custom Shaders, & Native Minecraft UI)
// ============================================================================

/// Allows embedding arbitrary Horizon rendering (3D Player Models, WGSL Shaders, Native ItemStacks)
/// inside a standard Canopy Flexbox layout!
pub struct CustomRenderComponent<F>
where
    F: Fn(&BoundingBox, &mut dyn RenderBackend),
{
    pub style: Style,
    pub bounds: BoundingBox,
    pub measured: MeasuredSize,
    pub render_fn: F,
}

impl<F> CustomRenderComponent<F>
where
    F: Fn(&BoundingBox, &mut dyn RenderBackend),
{
    pub fn new(style: Style, render_fn: F) -> Self {
        Self {
            style,
            bounds: BoundingBox::ZERO,
            measured: MeasuredSize::ZERO,
            render_fn,
        }
    }
}

impl<F> Component for CustomRenderComponent<F>
where
    F: Fn(&BoundingBox, &mut dyn RenderBackend),
{
    fn name(&self) -> &'static str {
        "CustomRenderComponent"
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn bounds(&self) -> BoundingBox {
        self.bounds
    }

    fn measured_size(&self) -> MeasuredSize {
        self.measured
    }

    fn measure(
        &mut self,
        _constraints: LayoutConstraints,
        _measurer: &dyn TextMeasurer,
    ) -> MeasuredSize {
        todo!("resolve self.style.width and self.style.height against constraints")
    }

    fn layout(&mut self, _bounds: BoundingBox) {
        todo!("store self.bounds = bounds")
    }

    fn render(&self, _backend: &mut dyn RenderBackend) {
        todo!("invoke (self.render_fn)(&self.bounds, backend)")
    }

    fn handle_event(&mut self, _event: &mut UIEvent) -> EventResult {
        EventResult::Ignored
    }
}

// ============================================================================
// 6. CanopyInspector (Live DevTools Overlay — like Elementa Inspector / Chrome DevTools)
// ============================================================================

pub struct CanopyInspector {
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub enabled: bool,
}

impl CanopyInspector {
    pub const fn new() -> Self {
        Self {
            cursor_x: 0.0,
            cursor_y: 0.0,
            enabled: false,
        }
    }

    /// Walks `root` recursively to find the deepest component containing `(cursor_x, cursor_y)`
    /// and draws its Margin (orange), Border/Padding (green), and Content (blue) overlays, plus a tag label!
    pub fn render_overlay(&self, _root: &dyn Component, _backend: &mut dyn RenderBackend) {
        todo!("recursively find deepest hit component in root and draw Chrome-DevTools-style box model overlay")
    }
}
