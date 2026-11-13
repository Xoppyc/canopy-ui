//! Core DOM component trait, rendering abstraction, and the `Container` (`<div>`) node.

use std::any::Any;

use crate::event::{EventResult, UIEvent};
use crate::layout::{compute_flex_layout, BoundingBox, FlexItem, LayoutConstraints, MeasuredSize};
use crate::style::{Color, Dimension, Edges, Style};

/// Abstraction for measuring text dimensions during Pass 1 (Measure).
/// Decouples the layout engine from Horizon's GPU font atlas so layouts can be unit-tested.
pub trait TextMeasurer {
    fn measure_text(&self, text: &str, scale: f32) -> MeasuredSize;
}

/// Abstraction for 2D/3D/Shader drawing commands during Pass 3 (Render).
/// In production, this wraps Horizon's `HudBatch` & `wgpu` context.
pub trait RenderBackend {
    /// Fills a solid axis-aligned rectangle.
    fn fill_rect(&mut self, bounds: BoundingBox, color: Color);

    /// Draws a text string at `(x, y)`.
    fn draw_text(&mut self, x: f32, y: f32, text: &str, color: Color, scale: f32);

    /// Draws a hollow rectangular border inside `bounds` with thickness `border_width`.
    /// Default implementation uses 4 `fill_rect` calls (top, bottom, left, right).
    fn draw_border(&mut self, bounds: BoundingBox, border_width: f32, color: Color) {
        if border_width <= 0.0 {
            return;
        }
        let top_border = BoundingBox {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: border_width.min(bounds.height),
        };
        let bottom_border = BoundingBox {
            x: bounds.x,
            y: bounds.y + bounds.height - border_width,
            width: bounds.width,
            height: border_width.min(bounds.height),
        };
        let left_border = BoundingBox {
            x: bounds.x,
            y: bounds.y + border_width,
            width: border_width,
            height: (bounds.height - 2.0 * border_width).max(0.0),
        };
        let right_border = BoundingBox {
            x: bounds.x + bounds.width - border_width,
            y: bounds.y + border_width,
            width: border_width,
            height: (bounds.height - 2.0 * border_width).max(0.0),
        };
        self.fill_rect(top_border, color);
        self.fill_rect(bottom_border, color);
        self.fill_rect(left_border, color);
        self.fill_rect(right_border, color);
    }

    /// Fills a rectangle with rounded corners (for Elementa's `UIRoundedRectangle`).
    fn fill_rounded_rect(&mut self, bounds: BoundingBox, _radius: f32, color: Color) {
        // Fallback until custom SDF shader is wired: draw standard rect
        self.fill_rect(bounds, color);
    }

    /// Pushes a scissor clip rectangle onto the clip stack (for `ScrollComponent`).
    /// Any drawing outside `bounds` will be clipped by the GPU.
    fn push_clip(&mut self, _bounds: BoundingBox) {}

    /// Pops the most recent scissor clip rectangle from the clip stack.
    fn pop_clip(&mut self) {}

    /// Escape hatch for 3D Model components, custom WGSL Shaders, and Native Minecraft UI bridges!
    /// Allows downcasting `&mut dyn RenderBackend` to the concrete `HorizonRenderBackend`.
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        None
    }
}

/// The core DOM node interface (analogous to Elementa's `UIComponent` or React's component instance).
pub trait Component {
    fn name(&self) -> &'static str;
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn bounds(&self) -> BoundingBox;
    fn measured_size(&self) -> MeasuredSize;
    fn children(&self) -> &[Box<dyn Component>] {
        &[]
    }
    fn measure(
        &mut self,
        constraints: LayoutConstraints,
        measurer: &dyn TextMeasurer,
    ) -> MeasuredSize;
    fn layout(&mut self, bounds: BoundingBox);
    fn render(&self, backend: &mut dyn RenderBackend);
    fn handle_event(&mut self, event: &mut UIEvent) -> EventResult;

    fn with_bg(mut self, color: Color) -> Self where Self: Sized {
        self.style_mut().background = Some(color);
        self
    }

    fn with_border(mut self, width: f32, color: Color) -> Self where Self: Sized {
        self.style_mut().border_width = width;
        self.style_mut().border_color = Some(color);
        self
    }

    fn with_padding(mut self, padding: Edges) -> Self where Self: Sized {
        self.style_mut().padding = padding;
        self
    }

    fn with_margin(mut self, margin: Edges) -> Self where Self: Sized {
        self.style_mut().margin = margin;
        self
    }

    fn with_size(mut self, width: Dimension, height: Dimension) -> Self where Self: Sized {
        self.style_mut().width = width;
        self.style_mut().height = height;
        self
    }

    fn with_width(mut self, width: Dimension) -> Self where Self: Sized {
        self.style_mut().width = width;
        self
    }

    fn with_height(mut self, height: Dimension) -> Self where Self: Sized {
        self.style_mut().height = height;
        self
    }

    fn with_gap(mut self, gap: f32) -> Self where Self: Sized {
        self.style_mut().gap = gap;
        self
    }

    fn with_direction(mut self, direction: crate::style::FlexDirection) -> Self where Self: Sized {
        self.style_mut().direction = direction;
        self
    }

    fn with_align(mut self, align: crate::style::AlignItems) -> Self where Self: Sized {
        self.style_mut().align_items = align;
        self
    }

    fn with_justify(mut self, justify: crate::style::JustifyContent) -> Self where Self: Sized {
        self.style_mut().justify_content = justify;
        self
    }
}

/// The foundational Flexbox container node (equivalent to Elementa's `UIContainer` + `UIBlock` or HTML `<div>`).
pub struct Container {
    pub style: Style,
    pub bounds: BoundingBox,
    pub measured: MeasuredSize,
    pub children: Vec<Box<dyn Component>>,
}

impl Container {
    pub fn new(style: Style) -> Self {
        Self {
            style,
            bounds: BoundingBox::ZERO,
            measured: MeasuredSize::ZERO,
            children: Vec::new(),
        }
    }

    pub fn row() -> Self {
        Self::new(Style::default().with_direction(crate::style::FlexDirection::Row))
    }

    pub fn column() -> Self {
        Self::new(Style::default().with_direction(crate::style::FlexDirection::Column))
    }

    pub fn with_child<C: Component + 'static>(mut self, child: C) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = Box<dyn Component>>) -> Self {
        self.children.extend(children);
        self
    }

    #[inline]
    pub fn content_inset(&self) -> Edges {
        Edges {
            top: self.style.padding.top + self.style.border_width,
            right: self.style.padding.right + self.style.border_width,
            bottom: self.style.padding.bottom + self.style.border_width,
            left: self.style.padding.left + self.style.border_width,
        }
    }
}

impl Component for Container {
    fn name(&self) -> &'static str {
        "Container"
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
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
        constraints: LayoutConstraints,
        measurer: &dyn TextMeasurer,
    ) -> MeasuredSize {
        let inset = self.content_inset();
        let resolved_w = self.style.width.resolve(constraints.max_width);
        let resolved_h = self.style.height.resolve(constraints.max_height);
        let child_max_w = match resolved_w {
            Some(w) => (w - inset.horizontal()).max(0.0),
            None => constraints.inset(&inset).max_width,
        };

        let child_max_h = match resolved_h {
            Some(h) => (h - inset.vertical()).max(0.0),
            None => constraints.inset(&inset).max_height,
        };
        let child_constraints = LayoutConstraints::new(child_max_w, child_max_h);
        let mut content_w: f32 = 0.0;
        let mut content_h: f32 = 0.0;
        let is_row = self.style.direction.is_row();
        for (i, child) in self.children.iter_mut().enumerate() {
            let size = child.measure(child_constraints, measurer);
            let margin = child.style().margin;

            let gap = if i > 0 { self.style.gap } else { 0.0 };

            if is_row {
                content_w += size.width + margin.horizontal() + gap;
                content_h = content_h.max(size.height + margin.vertical());
            } else {
                content_w = content_w.max(size.width + margin.horizontal());
                content_h += size.height + margin.vertical() + gap;
            }
        }

        let final_w = match resolved_w {
            Some(w) => w,
            None => content_w + inset.horizontal(),
        };

        let final_h = match resolved_h {
            Some(h) => h,
            None => content_h + inset.vertical(),
        };
        self.measured = MeasuredSize::new(final_w, final_h);
        self.measured
    }

    fn layout(&mut self, bounds: BoundingBox) {
        self.bounds = bounds;
        let content_box = bounds.inset(&self.content_inset());
        let items: Vec<FlexItem> = self
            .children
            .iter()
            .map(|child| FlexItem {
                style: *child.style(),
                measured: child.measured_size(),
            })
            .collect();

        let child_boxes = compute_flex_layout(
            content_box,
            self.style.direction,
            self.style.justify_content,
            self.style.align_items,
            self.style.gap,
            &items,
        );

        for (child, box_bounds) in self.children.iter_mut().zip(child_boxes) {
            child.layout(box_bounds);
        }
    }

    fn render(&self, backend: &mut dyn RenderBackend) {
        if let Some(bg) = self.style.background {
            backend.fill_rect(self.bounds, bg);
        }
        if let Some(border_color) = self.style.border_color {
            if self.style.border_width > 0.0 {
                backend.draw_border(self.bounds, self.style.border_width, border_color);
            }
        }
        for child in &self.children {
            child.render(backend);
        }
    }

    fn handle_event(&mut self, event: &mut UIEvent) -> EventResult {
        if let Some((x, y)) = event.coords() {
            if !self.bounds.contains(x, y) {
                return EventResult::Ignored;
            }
        }

        let mut result = EventResult::Ignored;
        for child in self.children.iter_mut().rev() {
            let r = child.handle_event(event);
            result = result.or(r);

            if event.is_propagation_stopped() {
                return result;
            }
        }

        result
    }
}
