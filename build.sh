#! /bin/sh
export AVR_CPU_FREQUENCY=16000000
xargo build --target avr-atmega328p $@
