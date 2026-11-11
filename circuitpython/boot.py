import board
import digitalio
from adafruit_debouncer import Button
# import time
# import storage


# import supervisor
# supervisor.set_usb_device_descriptor(
#     vid=0x6666,   # prE(9ototype VID
#     pid=0x0002,
#     manufacturer="peterichardson.com",
#     product="PetePad02 (GRID 1500)",
# )

print("hello from boot.py!")

# By default, disable the usb drive (/Volumes/CIRCUITPY)
# but mount it if the red pushbutton on the back
# of the device is pressed

# gpio 22 represents the red pushbutton
button = digitalio.DigitalInOut(board.GP22)
button.direction = digitalio.Direction.INPUT
button.pull = digitalio.Pull.UP
boot_switch = Button(button)

boot_switch.update()

if boot_switch.pressed:
     print("boot_switch is pressed")
     # storage.disable_usb_drive()
else:
     print("boot_switch is not pressed")

print("goodbye from boot.py!")
