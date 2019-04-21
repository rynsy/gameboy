mod cpu;
mod mmu;
pub mod gb;

/*
 *      Need this for the FromPrimitive function in OpCode
 *      NOTE: May be a better way of doing this.
 */
extern crate num;
#[macro_use]
extern crate num_derive;
