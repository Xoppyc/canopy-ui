pub type Color = [u8; 4];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    Auto,
    Pixels(f32),
    Percent(f32),
}

impl Dimension {
    #[inline]
    pub fn resolve(&self, parent_size: f32) -> Option<f32> {
        match *self {
            Self::Auto => None,
            Self::Pixels(px) => Some(px.max(0.0)),
            Self::Percent(pct) => Some((parent_size * (pct / 100.0)).max(0.0)),
        }
    }

    #[inline]
    pub fn is_auto(&self) -> bool {
        matches!(self, Self::Auto)
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Edges {
    pub const ZERO: Self = Self {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };
    pub const fn all(val: f32) -> Self {
        Self {
            top: val,
            right: val,
            bottom: val,
            left: val,
        }
    }

    pub const fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    pub const fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    #[inline]
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    #[inline]
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
}

impl FlexDirection {
    #[inline]
    pub fn is_row(&self) -> bool {
        matches!(self, Self::Row)
    }

    #[inline]
    pub fn is_col(&self) -> bool {
        matches!(self, Self::Column)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlignItems {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum JustifyContent {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
}

// first implementation basic styles list
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    pub width: Dimension,
    pub height: Dimension,
    pub padding: Edges,
    pub margin: Edges,
    pub gap: f32,
    pub direction: FlexDirection, // todo \/
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: f32,
}

// default styles
impl Default for Style {
    fn default() -> Self {
        Self {
            width: Dimension::Auto,
            height: Dimension::Auto,
            padding: Edges::ZERO,
            margin: Edges::ZERO,
            gap: 0.0,
            direction: FlexDirection::Row,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            background: None,
            border_color: None,
            border_width: 0.0,
        }
    }
}

impl Style {
    pub const fn new() -> Self {
        Self {
            width: Dimension::Auto,
            height: Dimension::Auto,
            padding: Edges::ZERO,
            margin: Edges::ZERO,
            gap: 0.0,
            direction: FlexDirection::Row,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            background: None,
            border_color: None,
            border_width: 0.0,
        }
    }
    pub fn with_width(mut self, width: Dimension) -> Self {
        self.width = width;
        self
    }
    pub fn with_height(mut self, height: Dimension) -> Self {
        self.height = height;
        self
    }
    pub fn with_size(mut self, width: Dimension, height: Dimension) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn with_padding(mut self, padding: Edges) -> Self {
        self.padding = padding;
        self
    }
    pub fn with_margin(mut self, margin: Edges) -> Self {
        self.margin = margin;
        self
    }
    pub fn with_direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }
    pub fn with_align(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }
    pub fn with_justify(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }
    pub fn with_bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
    pub fn with_border(mut self, width: f32, color: Color) -> Self {
        self.border_width = width;
        self.border_color = Some(color);
        self
    }
    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

/// JSX/DSL shorthand for `Dimension::Pixels(val)`.
#[inline]
pub fn px(val: f32) -> Dimension {
    return Dimension::Pixels(val);
}

/// JSX/DSL shorthand for `Dimension::Percent(val)`.
#[inline]
pub fn pct(val: f32) -> Dimension {
    return Dimension::Percent(val);
}

/// JSX/DSL shorthand to convert a `0xRRGGBBAA` hex literal into `[r, g, b, a]`.
#[inline]
pub fn hex(rgba: u32) -> Color {
    let r = ((rgba >> 24) & 0xFF) as u8;
    let g = ((rgba >> 16) & 0xFF) as u8;
    let b = ((rgba >> 8) & 0xFF) as u8;
    let a = (rgba & 0xFF) as u8;
    return [r, g, b, a];
}
