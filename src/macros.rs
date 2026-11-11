/// Build matrix scan pin arrays for the RP2040.
///
/// `input`-side pins are the matrix *rows*, configured with internal pull-down.
/// `output`-side pins are the matrix *columns*, driven low while the column is
/// scanned. This matches the device's column-to-row diode orientation (see the
/// `COL2ROW = true` on [`rmk::matrix::Matrix`] in `main.rs`).
macro_rules! config_matrix_pins_rp {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Output::new($p.$out_pin, embassy_rp::gpio::Level::Low)), +];
            let input_pins = [$(Input::new($p.$in_pin, embassy_rp::gpio::Pull::Down)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}