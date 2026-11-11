use rmk::types::action::KeyAction;
use rmk::types::modifier::ModifierCombination;
use rmk::{a, k, layer, wm};

pub(crate) const COL: usize = 5;
pub(crate) const ROW: usize = 5;
pub(crate) const NUM_LAYER: usize = 1;

/// The keypad's single layer, keyed to the physical matrix.
///
/// Row order matches the row pins in `main.rs` (GP20 at the top through GP16 at
/// the bottom). Column order matches the column pins (GP11 on the left through
/// GP15 on the right).
///
/// This replicates the CircuitPython (KMK) keymap in `circuitpython/code.py`.
/// The digits stay on the numpad keycodes; the shifted letters type A-F; and the
/// operators use the keypad '*' '/' '-' '+' and decimal-point codes, which send
/// the same characters as the main-row codes the old firmware used.
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [wm!(A, ModifierCombination::LSHIFT), wm!(B, ModifierCombination::LSHIFT), wm!(C, ModifierCombination::LSHIFT), wm!(D, ModifierCombination::LSHIFT), k!(Backspace)],
            [k!(Kp7), k!(Kp8), k!(Kp9), wm!(E, ModifierCombination::LSHIFT), k!(KpSlash)],
            [k!(Kp4), k!(Kp5), k!(Kp6), wm!(F, ModifierCombination::LSHIFT), k!(KpMinus)],
            [k!(Kp1), k!(Kp2), k!(Kp3), k!(KpEnter), k!(KpAsterisk)],
            [k!(Kp0), a!(No), k!(KpDot), a!(No), k!(KpPlus)]
        ]),
    ]
}