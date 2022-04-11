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

/// Analog reads in two pins into an array
#[macro_export]
macro_rules! two_pins_to_arr_analog {
    ($adc:expr, $a0:expr, $a1:expr) => {
        {
            let values = [
                $a0.analog_read(&mut $adc),
                $a1.analog_read(&mut $adc),
            ];

            values
        }
    }
}

/// Outputs two pins analog values to serial console
#[macro_export]
macro_rules! two_pins_analog_to_ser_out {
    ($ser:expr, $val:expr) => {
        {
            for(i, v) in $val.iter().enumerate() {
                ufmt::uwriteln!(&mut $ser, "A{}: {} ", i, v).void_unwrap();
            }
            ufmt::uwriteln!(&mut $ser, "").void_unwrap();
        }
    }
}
