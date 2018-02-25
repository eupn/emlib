use std::env;

extern crate cc;

use cc::Build;

fn main() {
    compile_emlib_library();
}

fn compile_emlib_library() {
    println!("The ARM embedded toolchain must be available in the PATH");

    env::set_var("CC", "arm-none-eabi-gcc");
    env::set_var("AR", "arm-none-eabi-ar");

    env::set_var("TARGET", "thumbv6m-none-eabi");

    let mut build = Build::new();

    common_config(&mut build);

    build.compile("libemlib.a");
}

fn common_config(build: &mut Build) -> &mut Build {
    let path = env::var("CARGO_MANIFEST_DIR").ok().unwrap();

    println!("{}", path);

    // Build EMlib sources into a static library
    build
        .define("__arm__", None)
        .define("EFM32HG222F64", None)

        .include("efm32-common/CMSIS/Include")
        .include("efm32-common/Device/EFM32HG/Include")
        .include("efm32-common/emlib/inc")
        .include("efm32-common/support")

        .file("efm32-common/Device/EFM32HG/Source/system_efm32hg.c")

        .file("efm32-common/emlib/src/em_acmp.c")
        .file("efm32-common/emlib/src/em_adc.c")
        .file("efm32-common/emlib/src/em_aes.c")
        .file("efm32-common/emlib/src/em_assert.c")
        .file("efm32-common/emlib/src/em_burtc.c")
        .file("efm32-common/emlib/src/em_chip.c")
        .file("efm32-common/emlib/src/em_cmu.c")
        .file("efm32-common/emlib/src/em_dac.c")
        .file("efm32-common/emlib/src/em_dbg.c")
        .file("efm32-common/emlib/src/em_dma.c")
        .file("efm32-common/emlib/src/em_emu.c")
        .file("efm32-common/emlib/src/em_gpio.c")
        .file("efm32-common/emlib/src/em_i2c.c")
        .file("efm32-common/emlib/src/em_idac.c")
        .file("efm32-common/emlib/src/em_int.c")
        .file("efm32-common/emlib/src/em_lcd.c")
        .file("efm32-common/emlib/src/em_leuart.c")
        .file("efm32-common/emlib/src/em_mpu.c")
        .file("efm32-common/emlib/src/em_msc.c")
        .file("efm32-common/emlib/src/em_opamp.c")
        .file("efm32-common/emlib/src/em_pcnt.c")
        .file("efm32-common/emlib/src/em_prs.c")
        .file("efm32-common/emlib/src/em_rmu.c")
        .file("efm32-common/emlib/src/em_rtc.c")
        .file("efm32-common/emlib/src/em_system.c")
        .file("efm32-common/emlib/src/em_timer.c")
        .file("efm32-common/emlib/src/em_usart.c")
        .file("efm32-common/emlib/src/em_vcmp.c")
        .file("efm32-common/emlib/src/em_wdog.c")

        .file("efm32-common/support/adc.c")
        .file("efm32-common/support/emu.c")
        .file("efm32-common/support/get_acmp.c")
        .file("efm32-common/support/get_adc.c")
        .file("efm32-common/support/get_leuart.c")
        .file("efm32-common/support/get_timer.c")
        .file("efm32-common/support/gpio.c")
        .file("efm32-common/support/i2c.c")
        .file("efm32-common/support/irq.c")
        .file("efm32-common/support/leuart.c")
        .file("efm32-common/support/rtc.c")
        .file("efm32-common/support/timer.c")
        .file("efm32-common/support/usart.c")

        .flag("-Wall")
        .flag("-mthumb")
        .flag("-g")
        .flag("-specs=nano.specs")
        .flag("-specs=nosys.specs")
        .flag("-nostartfiles")
        .flag("-march=armv6-m")
        .flag("-mfloat-abi=soft")
        .flag("-mcpu=cortex-m0plus")
        .flag("-fno-builtin")
        .flag("-fno-builtin-function")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fno-exceptions")
        .flag("-fno-pic")
}