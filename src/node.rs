//! Core DOM component trait, rendering abstraction, and the `Container` (`<div>`) node.

use std::any::Any;

use crate::event::{EventResult, UIEvent};
use crate::layout::{compute_flex_layout, BoundingBox, FlexItem, LayoutConstraints, MeasuredSize};
use crate::style::{Color, Edges, Style};
use crate::Dimension;

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
            height: bounds.height - 2.0 * border_width.max(0.0),
        };
        let right_border = BoundingBox {
            x: bounds.x + bounds.width - border_width,
            y: bounds.y + border_width,
            width: border_width,
            height: bounds.height - 2.0 * border_width.max(0.0),
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
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// The core DOM node interface (analogous to Elementa's `UIComponent` or React's component instance).
pub trait Component {
    /// Human-readable component type name (e.g. `"Container"`, `"UIText"`, `"ScrollComponent"`).
    /// Used by the in-game `CanopyInspector` tree view.
    fn name(&self) -> &'static str;

    /// Returns this component's style rules.
    fn style(&self) -> &Style;

    /// Returns the screen-space bounding box computed during the most recent `layout` pass.
    fn bounds(&self) -> BoundingBox;

    /// Returns the intrinsic size computed during the most recent `measure` pass.
    fn measured_size(&self) -> MeasuredSize;

    /// Returns a slice of child components (defaults to empty `&[]` for leaf nodes).
    /// Enables the `CanopyInspector` to walk the entire DOM tree generically.
    fn children(&self) -> &[Box<dyn Component>] {
        &[]
    }

    /// Pass 1 (Bottom-Up): Computes and stores the desired outer size (content + padding + border).
    fn measure(
        &mut self,
        constraints: LayoutConstraints,
        measurer: &dyn TextMeasurer,
    ) -> MeasuredSize;

    /// Pass 2 (Top-Down): Assigns final screen-space `bounds` and positions any children.
    fn layout(&mut self, bounds: BoundingBox);

    /// Pass 3: Emits draw calls to the `RenderBackend`.
    fn render(&self, backend: &mut dyn RenderBackend);

    /// Pass 4: Processes an input event (hit-testing & bubbling).
    fn handle_event(&mut self, event: &mut UIEvent) -> EventResult;
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

    /// Builder method to append a child component.
    pub fn with_child<C: Component + 'static>(mut self, child: C) -> Self {
        self.children.push(Box::new(child));
        self
    }

    /// Helper: returns the total inset edges combining both `padding` and `border_width`.
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

        // running flex engine on collected items
        let child_boxes = compute_flex_layout(
            content_box,
            self.style.direction,
            self.style.justify_content,
            self.style.align_items,
            &items,
        );

        for (child, box_bounds) in self.children.iter_mut().zip(child_boxes) {
            child.layout(box_bounds);
        }
    }

    fn render(&self, _backend: &mut dyn RenderBackend) {
        todo!("Pass 3: draw background, draw border, and render each child")
    }

    fn handle_event(&mut self, _event: &mut UIEvent) -> EventResult {
        todo!("Pass 4: hit-test children in reverse order and bubble the event")
    }
}
