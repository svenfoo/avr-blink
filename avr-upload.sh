#!/bin/bash

# Gimme the name of a file to load
ELFFILE=${1}

# We need to use the `avrdude` that comes with the Arduino IDE, it seems
# to have some custom changes not in the version we installed from Brew, that
# work with the UPDI-over-USB bootloader on the Arduino Nano Every
AVRDUDE=/home/neo/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/bin/avrdude
AVRCONF=/home/neo/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/etc/avrdude.conf

# Chip option fuses
FUSE_OSCCFG=0x82    # 20 MHz
FUSE_SYSCFG0=0xC9   # No CRC, Reset is Reset, don't erase EEPROM
FUSE_BOOTEND=0x00   # Whole Flash is boot

# Device specific flags
PART=atmega4809
PROGRAMMER=jtag2updi
BAUDRATE=115200

# Where to find it
PORT=/dev/ttyACM0

# We reset the Arduino (and put it into UPDI mode) by opening & closing the
# serial port at 1200baud (this is some kind of 'backdoor' reset process
# built into the USB software that runs on the Nano Every's coprocessor
# for handling USB-to-UPDI.
stty -F "${PORT}" 1200

# Wait for the port to be available again
while [ 1 ]; do
    sleep 0.5
  [ -c "${PORT}" ] && break
done


# NOW, finally, we can actually upload our code
${AVRDUDE} \
   -C ${AVRCONF} \
   -v -p${PART} \
   -c${PROGRAMMER} \
   -P${PORT} -b${BAUDRATE} \
   -e -D \
   -Uflash:w:${ELFFILE}:e \
   -Ufuse2:w:${FUSE_OSCCFG}:m -Ufuse5:w:${FUSE_SYSCFG0}:m -Ufuse8:w:${FUSE_BOOTEND}:m
