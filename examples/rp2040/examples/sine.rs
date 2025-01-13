#![no_std]
#![no_main]
use bsp::entry;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    self as hal,
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};



use libm::sinf;
use microflow::model;
use nalgebra::matrix;


#[model("../../models/sine.tflite")]
struct Sine;


#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;

    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();


    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let x = 0.5;
    let start = timer.get_counter();
    let y_predicted = Sine::predict(matrix![x])[0];
    let end = timer.get_counter();
    let y_exact = sinf(x);
    info!(" ");
    info!("Predicted sin({}): {}", x, y_predicted);
    info!("Exact sin({}): {}", x, y_exact);
    info!("Error: {}", y_exact - y_predicted);
    info!("Execution time: {:tus} us", (end - start).to_micros());

    loop {
        cortex_m::asm::nop();
    }
}
