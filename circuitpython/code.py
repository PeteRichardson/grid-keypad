import board
import digitalio
import time
from adafruit_debouncer import Button

from kmk.kmk_keyboard import KMKKeyboard
from kmk.keys import KC
from kmk.scanners import DiodeOrientation


keyboard = KMKKeyboard()

keyboard.col_pins = (
    board.GP11,
    board.GP12,
    board.GP13,
    board.GP14,
    board.GP15
)
keyboard.row_pins = (
    board.GP20,
    board.GP19,
    board.GP18,
    board.GP17,
    board.GP16
)
keyboard.diode_orientation = DiodeOrientation.COL2ROW

keyboard.keymap = [
    [KC.LSHIFT(KC.A), KC.LSHIFT(KC.B), KC.LSHIFT(KC.C), KC.LSHIFT(KC.D), KC.BSPACE,
     KC.N7, KC.N8, KC.N9, KC.LSHIFT(KC.E), KC.SLASH,
     KC.N4, KC.N5, KC.N6, KC.LSHIFT(KC.F), KC.MINUS,
     KC.N1, KC.N2, KC.N3, KC.ENTER, KC.ASTERISK,
     KC.N0, KC.NO, KC.DOT, KC.NO, KC.PLUS
     ]
]

if __name__ == '__main__':
    print("--- boot.py log: ", "-"*25)
    try:
        f = open('boot_out.txt', 'r')
        print(f.read().rstrip())
        f.close()
    except:
        print("No boot_out.txt found")
    print("--- code.py log: ", "-"*25)

    keyboard.go()
