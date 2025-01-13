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

use microflow::buffer::Buffer2D;
use microflow::model;

#[path = "../../../samples/features/person_detect.rs"]
mod features;

#[model("../../models/person_detect.tflite")]
struct PersonDetect;

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

    info!("Input sample: 'person.bmp'...");
    let start = timer.get_counter();
    let person_predicted = PersonDetect::predict_quantized(features::PERSON);
    let end = timer.get_counter();
    info!(" ");
    print_prediction(person_predicted);
    info!("Execution time: {:tus} us", (end - start).to_micros());

    info!("Input sample: 'no_person.bmp'...");
    let start = timer.get_counter();
    let no_person_predicted = PersonDetect::predict_quantized(features::NO_PERSON);
    let end = timer.get_counter();
    info!(" ");
    print_prediction(no_person_predicted);
    info!("Execution time: {:tus} us", (end - start).to_micros());

    loop {
        cortex_m::asm::nop();
    }
}

fn print_prediction(prediction: Buffer2D<f32, 1, 2>) {
    info!(
        "Prediction: {}% no person, {}% person",
        prediction[0] * 100.,
        prediction[1] * 100.,
    );
    info!(
        "Outcome: {}",
        match prediction.iamax_full().1 {
            0 => "NO PERSON",
            1 => "PERSON",
            _ => unreachable!(),
        }
    );
}
