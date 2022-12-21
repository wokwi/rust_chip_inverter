// Wokwi Custom Chips with Rust
//
// Very rough prototype by Uri Shaked
//
// Look at chipInit() at the bottom, and open Chrome devtools console to see the debugPrint().

use std::ffi::{c_void, CString};

use wokwi_chip_ll::{
    debugPrint, pinInit, pinWatch, pinWrite, PinId, WatchConfig, BOTH, HIGH, INPUT, LOW, OUTPUT,
};

struct Chip {
    pin_in: PinId,
    pin_out: PinId,
}

// chipInit() will be called once per chip instance. We use CHIP_VEC to keep track of all the
// instances, and use the user_data pointer to index into CHIP_VEC.
static mut CHIP_VEC: Vec<Chip> = Vec::new();

pub unsafe fn on_pin_change(user_data: *const c_void, _pin: PinId, value: u32) {
    let chip = &CHIP_VEC[user_data as usize];
    if value == HIGH {
        pinWrite(chip.pin_out, LOW);
    } else {
        pinWrite(chip.pin_out, HIGH);
    }
}

#[no_mangle]
pub unsafe extern "C" fn chipInit() {
    debugPrint(CString::new("Hello Rust!").unwrap().into_raw());

    let chip = Chip {
        pin_in: pinInit(CString::new("IN").unwrap().into_raw(), INPUT),
        pin_out: pinInit(CString::new("OUT").unwrap().into_raw(), OUTPUT),
    };
    CHIP_VEC.push(chip);
    let chip = CHIP_VEC.last().unwrap();

    let watch_config = WatchConfig {
        user_data: (CHIP_VEC.len() - 1) as *const c_void,
        edge: BOTH,
        pin_change: on_pin_change as *const c_void,
    };

    pinWatch(chip.pin_in, &watch_config);
}
