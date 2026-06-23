#[allow(unused)]
use super::*;

/// The handedness of the coordinate system, with:
/// <br>&emsp;+X rightward
/// <br>&emsp;+Y upward
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Chirality {
    /// +Z forward
    Left,
    /// +Z backward
    #[default]
    Right,
}

/// Which direction defines the front face.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Winding {
    Clockwise,
    #[default]
    Counterclockwise,
}

/// Defines the [`Shape`]'s local/object space.
///
/// Generally not sufficient for direct use in the Normalized Device Coordinates (NDC) space.
#[derive(Default, Clone, Copy)]
pub struct Orientation {
    /// Whether the coordinate space is left- or right-handed.
    pub chirality: Chirality,
    /// How the vertex order defines the front and back faces.
    pub winding: Winding,
}

impl Orientation {
    /// Returns `true` is the orientation is left-handed.
    pub fn is_left(&self) -> bool {
        self.chirality == Chirality::Left
    }

    /// Returns `true` is the orientation is right-handed.
    pub fn is_right(&self) -> bool {
        self.chirality == Chirality::Right
    }

    /// Returns `true` is the orientation winds clockwise.
    pub fn is_cw(&self) -> bool {
        self.winding == Winding::Clockwise
    }

    /// Returns `true` is the orientation winds counterclockwise.
    pub fn is_ccw(&self) -> bool {
        self.winding == Winding::Counterclockwise
    }
}

/// Passed to [`Shaper::shape()`] to configure the returned [`Shape`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Configuration {
    /// Controls the coordinate space [`Chirality`] and primitive [`Winding`] order.
    pub orientation: Orientation,
    /// Whether the [`Shape`] should include normal data.
    pub generate_normals: bool,
    /// Provides index data as triangle strips, if possible.
    pub prefer_strips: bool,
}

