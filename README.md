# emlib

Rust bindings for Silicon Labs emlib.

`emlib` compiles and runs on Silicon Labs EFM32 Microcontrollers. 
It's a proof-of-concept and work in progress, so currently only parts the
[EFM32 Happy Gecko](http://www.silabs.com/products/mcu/32-bit/efm32-giant-gecko/pages/efm32-happy-gecko.aspx)
family of their microcontrollers are supported.

# Compiling emlib
`emlib` requires the following tools to build:
* [ARM GCC Embedded Toolchain](https://launchpad.net/gcc-arm-embedded) - Used to build 
Silicon Labs emlib for the EFM32.
* [SEGGER JLink](https://www.segger.com/jlink-software.html) - Used for GDB debugging and to 
flash the EFM32's from the command line

If you're on linux, you can install the ARM GCC toolchain like this:
```bash
$ sudo add-apt-repository ppa:terry.guo/gcc-arm-embedded -y
$ sudo apt-get update -q
$ sudo apt-get install gcc-arm-none-eabi
```