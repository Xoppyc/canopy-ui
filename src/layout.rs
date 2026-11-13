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

    #[inline]
    pub fn left(&self) -> f32 {
        self.x
    }

    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    #[inline]
    pub fn top(&self) -> f32 {
        self.y
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

    /// Expands the bounding box outward by the given edges (used by Inspector to draw margin boxes).
    #[inline]
    pub fn outset(&self, edges: &Edges) -> Self {
        Self {
            x: self.x - edges.left,
            y: self.y - edges.top,
            width: (self.width + edges.left + edges.right).max(0.0),
            height: (self.height + edges.top + edges.bottom).max(0.0),
        }
    }

    /// Shifts the bounding box by `(dx, dy)` (used by ScrollComponent to scroll children).
    #[inline]
    pub fn translate(&self, dx: f32, dy: f32) -> Self {
        return BoundingBox {
            x: self.x + dx,
            y: self.y + dy,
            width: self.width,
            height: self.height,
        };
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
    let items_empty = items.is_empty();
    if items_empty {
        return Vec::new();
    }
    let is_row = direction.is_row();
    let (main_avail, cross_avail) = if is_row {
        (content_box.width, content_box.height)
    } else {
        (content_box.height, content_box.width)
    };

    let mut total_main_size: f32 = 0.0;
    for item in items {
        total_main_size += if is_row {
            item.measured.width + item.style.margin.horizontal()
        } else {
            item.measured.height + item.style.margin.vertical()
        }
    }
    let remaining_main = (main_avail - total_main_size).max(0.0);

    let (mut current_main, gap) = match justify {
        JustifyContent::Start => (0.0, 0.0),
        JustifyContent::Center => (remaining_main / 2.0, 0.0),
        JustifyContent::End => (remaining_main, 0.0),
        JustifyContent::SpaceBetween => {
            if items.len() <= 1 {
                (0.0, 0.0)
            } else {
                (0.0, (remaining_main / (items.len() - 1) as f32))
            }
        }
    };
    let mut output: Vec<BoundingBox> = Vec::with_capacity(items.len());
    if is_row {
        for item in items {
            current_main += item.style.margin.left;
            let avail_h =
                (cross_avail - item.measured.height - item.style.margin.vertical()).max(0.0);

            let x = content_box.x + current_main;
            let width = item.measured.width;
            let base_y = content_box.y + item.style.margin.top;
            let (y, height) = match align {
                AlignItems::Start => (base_y, item.measured.height),
                AlignItems::Center => (base_y + avail_h / 2.0, item.measured.height),
                AlignItems::End => (base_y + avail_h, item.measured.height),
                AlignItems::Stretch => (base_y, item.measured.height + avail_h),
            };

            current_main += width + item.style.margin.right + gap;
            output.push(BoundingBox {
                x,
                y,
                width,
                height,
            });
        }
    } else {
        for item in items {
            current_main += item.style.margin.top;
            let avail_w =
                (cross_avail - item.measured.width - item.style.margin.horizontal()).max(0.0);

            let y = content_box.y + current_main;
            let height = item.measured.height;
            let base_x = content_box.x + item.style.margin.left;
            let (x, width) = match align {
                AlignItems::Start => (base_x, item.measured.width),
                AlignItems::Center => (base_x + avail_w / 2.0, item.measured.width),
                AlignItems::End => (base_x + avail_w, item.measured.width),
                AlignItems::Stretch => (base_x, item.measured.width + avail_w),
            };

            current_main += height + item.style.margin.bottom + gap;
            output.push(BoundingBox {
                x,
                y,
                width,
                height,
            });
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_center() {
        let box_area = BoundingBox::new(0.0, 0.0, 200.0, 100.0);
        let items = vec![FlexItem {
            style: Style::default(),
            measured: MeasuredSize::new(50.0, 20.0),
        }];

        let result = compute_flex_layout(
            box_area,
            FlexDirection::Row,
            JustifyContent::Center,
            AlignItems::Center,
            &items,
        );

        // In a 200x100 box, where should a centered 50x20 item land?
        assert_eq!(result[0], BoundingBox::new(75.0, 40.0, 50.0, 20.0));
    }
}
