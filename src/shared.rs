/* Macros defined for the uno-revive-rs project
    Authored by: Ethan Gallucci 
*/

#![no_std]

// Reads from ADC channels: Vbandgap, Ground, Temperature
#[macro_export]
macro_rules! vgt {
        ($adc:expr, $ser:expr) => {
            {
                let (vbg, gnd, tmp) = (
                    $adc.read_blocking(&adc::channel::Vbg),
                    $adc.read_blocking(&adc::channel::Gnd),
                    $adc.read_blocking(&adc::channel::Temperature),
                );

                ufmt::uwriteln!(&mut $ser, "Vbandgap: {}", vbg).void_unwrap();
                ufmt::uwriteln!(&mut $ser, "Ground: {}", gnd).void_unwrap();
                ufmt::uwriteln!(&mut $ser, "Temperature: {}", tmp).void_unwrap();
            }
        }
}

// Setup a watchdog timer with an 8ms timeout
#[macro_export]
macro_rules! watchdog8ms {
    ($periph:expr) => {
        let mut watchdog = wdt::Wdt::new($periph.wdt, &periph.CPU.mcusr);
        watchdog.start(wdt::Timeout::Ms8000).unwrap();
    }
}