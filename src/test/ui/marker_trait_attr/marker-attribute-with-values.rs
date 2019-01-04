#![feature(marker_trait_attr)]

#[marker(always)]
trait Marker1 {}
//~^^ ERROR attribute must be of the form `#[marker]`

#[marker("never")]
trait Marker2 {}
//~^^ ERROR attribute must be of the form `#[marker]`

#[marker(key = value)]
trait Marker3 {}
//~^^ ERROR expected unsuffixed literal or identifier, found value

fn main() {}
