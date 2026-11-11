# GRiD 1500 keypad

<img src="docs/grid-keypad.png" alt="GRiD 1500 Series numeric keypad with cream and black keycaps and a red button on top" width="400">

Firmware backup for a custom USB numeric keypad. The case is a GRiD 1500 Series
keypad. The electronics and keycaps are new: a Raspberry Pi Pico running
CircuitPython and [KMK](https://github.com/KMKfw/kmk_firmware).

This repo is a direct copy of the `KEYPAD` drive, taken 2026-11-11.

## Versions on the device

- CircuitPython 9.1.1 (2024-07-22), board `raspberry_pi_pico` (see `boot_out.txt`)
- KMK: copy in `kmk/`, dated 2024-08-02
- Adafruit libraries in `lib/`: `adafruit_debouncer`, `adafruit_ticks`

## Restore

1. Install CircuitPython 9.1.1 for the Raspberry Pi Pico (hold BOOTSEL, plug in,
   copy the `.uf2` file to `RPI-RP2`).
2. Copy all files in this repo (not `.git`, `.gitignore`, `README.md` or `docs/`) to the
   root of the CircuitPython drive.
3. Optional: rename the drive to `KEYPAD`. `.VolumeIcon.icns` is the drive icon.

## Files

- `code.py` – key matrix pins (cols GP11–GP15, rows GP20–GP16) and the keymap
- `boot.py` – reads the red button on the back (GP22) at boot
- `kmk/` – KMK firmware (third-party, GPL-3.0)
- `lib/` – Adafruit CircuitPython libraries (third-party, MIT)
