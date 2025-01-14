#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(
    unused_macros,
    unused_must_use,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::absurd_extreme_comparisons,
    clippy::cmp_null,
    clippy::missing_safety_doc
)]

pub mod singletons;
mod table_const;
mod sonic;
mod lucario;
pub mod element;
mod eflame;
mod elight;

#[skyline::main(name = "vanilla_status")]
pub fn main() {
    sonic::install();
    lucario::install();
    eflame::install();
    elight::install();
}