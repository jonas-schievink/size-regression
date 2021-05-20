#![no_main]
#![no_std]

use regression as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    regression::exit()
}
