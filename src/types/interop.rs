/// The interopable binding between the Rust and C structure.
pub trait Interopable
where
    Self: TryFrom<Self::Target> + TryInto<Self::Target>,
{
    /// The type of the raw structure.
    type Target;
}
