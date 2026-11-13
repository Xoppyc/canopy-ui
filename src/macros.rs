//! Declarative JSX-style macro (`ui!`) for building Canopy component trees ergonomically.
//!
//! True JSX syntax:
//! ```ignore
//! let ui = ui! {
//!     <Container width={px(400.0)} height={px(300.0)} bg={hex(0x1E1E2EFF)} gap={10.0} direction={FlexDirection::Column}>
//!         <UIText text="Canopy Playground" scale={1.5} color={hex(0xFFFFFFFF)} />
//!         <Container padding={Edges::all(8.0)} bg={hex(0x313244FF)}>
//!             <UIText text="Inside nested container!" />
//!         </Container>
//!     </Container>
//! };
//! ```

#[macro_export]
macro_rules! ui {
    // TODO: Implement declarative true JSX parsing rules mapping `<Tag prop={val}>...</Tag>` to `.with_child(...)`!
    ($($tokens:tt)*) => {
        todo!("implement declarative JSX ui! macro expansion")
    };
}
