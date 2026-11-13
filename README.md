# Canopy

A DOM GUI library for the [Horizon](https://playhorizon.dev) Minecraft client engine.

Canopy implements a CSS-style box model, Flexbox layout, event bubbling, and fine-grained reactivity in native Rust, providing a component-based workflow similar to SolidJS and React.

---

## Architecture

### Components and Retained Mode
Components are reusable functions that return a component tree:

```rust
#[component]
fn PlayerCard(name: &str, health: Signal<f32>) -> impl Component {
    ui! {
        <Container padding=(8.0, 16.0) bg="#181825" border=(1.5, "#89b4fa") direction=FlexDirection::Column gap=8.0>
            <UIText text=name.to_string() color="#cdd6f4" />
            <HealthBar value=health />
        </Container>
    }
}
```

The component tree mounts once and stays resident in memory (`CanopyRoot`). Layout and paint operations are separated with dirty flags:
* `Dirty::PAINT`: Color, opacity, or texture updates trigger a repaint without recalculating box dimensions.
* `Dirty::LAYOUT`: Size, margin, padding, or gap changes trigger a flexbox pass on the affected subtree. When nothing on screen changes, layout uses 0% CPU.

### Fine-Grained Reactivity
Canopy uses signals instead of a virtual DOM. Component functions run once during setup to wire the tree. When a signal updates, it notifies only the subscribing leaf node directly, leaving the rest of the tree untouched.

### Syntax and DSL
The `ui!` macro supports JSX-style markup with convenience conversions:
* Edges accept tuples like `padding=(8.0, 16.0)` or single floats like `padding=12.0`.
* Colors accept standard CSS hex strings like `bg="#313244"`.
* For developers who prefer not to use macros, all components expose standard builder methods (`Container::new(...).with_child(...)`).

### Animation
The engine supports three types of animation:
* Interruptible physics springs ($F = -kx - cv$) and cubic-bezier transitions.
* Keyframe timelines for multi-step animations with loop and completion controls.
* Sprite sheet and UV scroll animations for Minecraft textures, running entirely within the paint pass.

### Modularity
The core crate (`canopy`) remains zero-dependency, covering layout, events, drawing abstractions, and reactivity. Utility class scanning (similar to Tailwind) is handled by a separate companion tool so the core engine stays small. External stylesheet loading is planned for Minecraft resource pack support.

---

## Roadmap

### Phase 1: Core Layout Engine and Box Model (Done)
- [x] Style primitives (`Dimension`, `Edges`, `FlexDirection`, `AlignItems`, `JustifyContent`, `gap`, `hex`, `px`, `pct`)
- [x] Two-pass layout engine (`compute_flex_layout`)
- [x] Bounding box geometry (`contains`, `inset`, `outset`, `translate`)
- [x] Event bubbling and input handling (`UIEvent`, `EventResult`)

### Phase 2: DOM Nodes and Horizon Mod Bridge (Done)
- [x] `RenderBackend` and `TextMeasurer` traits for GPU and test decoupling
- [x] `Container` node implementing measure, layout, render, and hit-testing
- [x] Horizon native mod bridge with HUD rendering hook (`hud.rs`)
- [x] Packaging script (`bundle.py`) to sync library files into the mixin bundle

### Phase 3: Reactivity and Signals (Next)
- [ ] SolidJS-style signal primitives (`create_signal`, `create_effect`, `create_memo`)
- [ ] Direct leaf node signal subscriptions and dirty-flag propagation

### Phase 4: JSX Macro (`ui!`)
- [ ] Macro expansion for `<Tag prop=val> ... </Tag>` syntax
- [ ] Component attribute macro (`#[component]`)

### Phase 5: Component Catalog and Tooling (`widgets.rs`)
- [ ] `UIText` and `UIWrappedText` with font atlas measurement
- [ ] `ScrollComponent` with GPU scissor clipping
- [ ] `UITextInput` with keyboard focus and cursor navigation
- [ ] Rounded rectangle and shader rendering
- [ ] In-game box model inspector overlay
