//! # Component
//! Component trait: while Transform holds the 3D position and scene tree information, Components hold the logic and actual object data.

use super::transform::TransformId;

/// # Component
/// A component attaches to a Transform and gives it functionality. It handles the life cycle of a scene object.
pub trait Component{

    /// Returns the parent Transform of the component.
    fn parent(&self) -> &TransformId;

    /// Function executed  when the component is appended to its parent transform.
    fn initialize(&self) -> () {}

    /// Function executed at the start of the first frame.
    fn start(&self) -> () {}

    /// Function executed each frame.
    fn update(&self) -> () {}

    /// Function executed before destroying the component.
    fn destroy(&self) -> () {}
}
