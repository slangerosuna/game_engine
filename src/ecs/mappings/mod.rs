pub mod spatial;
pub mod table;

/**
 *
 */
pub trait Mapping {}

pub trait Query {
    type Mapping: Mapping;
    type Out;

    fn query(&self, map: Self::Mapping) -> Self::Out;
}
