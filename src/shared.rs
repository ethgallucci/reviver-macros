// Macros
#![no_std]

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
