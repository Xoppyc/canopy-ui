//! Layout engine and spatial primitives for Canopy.

use crate::style::{AlignItems, Edges, FlexDirection, JustifyContent, Style};

/// Screen-space rectangular bounding box.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Left edge coordinate
    #[inline]
    pub fn left(&self) -> f32 {
        return self.x;
    }

    /// Right edge coordinate
    #[inline]
    pub fn right(&self) -> f32 {
        return self.x + self.width;
    }

    /// Bottom edge coordinate
    #[inline]
    pub fn bottom(&self) -> f32 {
        return self.y + self.height;
    }

    /// Top edge coordinate
    #[inline]
    pub fn top(&self) -> f32 {
        return self.y;
    }

    /// Returns true if the given screen coordinate lies within this box.
    #[inline]
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.left() && px <= self.right() && py >= self.top() && py <= self.bottom()
    }

    /// Shrinks the bounding box inward by the given edges (e.g. for padding/border).
    #[inline]
    pub fn inset(&self, edges: &Edges) -> Self {
        Self {
            x: self.x + edges.left,
            y: self.y + edges.top,
            width: (self.width - edges.left - edges.right).max(0.0),
            height: (self.height - edges.top - edges.bottom).max(0.0),
        }
    }
}

/// Sizing constraints passed from parent to child during Pass 1 (Measure).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutConstraints {
    pub max_width: f32,
    pub max_height: f32,
}

impl LayoutConstraints {
    pub const UNCONSTRAINED: Self = Self {
        max_width: f32::INFINITY,
        max_height: f32::INFINITY,
    };

    pub const fn new(max_width: f32, max_height: f32) -> Self {
        Self {
            max_width,
            max_height,
        }
    }

    /// Shrinks the constraints inward by the given edges (padding/borders).
    #[inline]
    pub fn inset(&self, edges: &Edges) -> Self {
        Self {
            max_width: (self.max_width - edges.left - edges.right).max(0.0),
            max_height: (self.max_height - edges.top - edges.bottom).max(0.0),
        }
    }
}

/// Computed intrinsic size from Pass 1 (Measure).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeasuredSize {
    pub width: f32,
    pub height: f32,
}

impl MeasuredSize {
    pub const ZERO: Self = Self {
        width: 0.0,
        height: 0.0,
    };

    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// Descriptor of an item to be arranged along a flex axis.
pub struct FlexItem {
    pub style: Style,
    pub measured: MeasuredSize,
}

/// Pass 2: Computes the screen-space bounding boxes for children within `content_box`.
pub fn compute_flex_layout(
    content_box: BoundingBox,
    direction: FlexDirection,
    justify: JustifyContent,
    align: AlignItems,
    items: &[FlexItem],
) -> Vec<BoundingBox> {
    todo!("implement flex distribution along main axis and cross axis")
}
