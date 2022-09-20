extern crate winapi;
use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, INPUT_u, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput, VK_RETURN, VK_TAB};



pub fn inject_string(s: String) {
    let mut v = vec![];
    let s_vec = widestring::U16CString::from_str(s).unwrap().into_vec();
    for c in &s_vec {
        unsafe {
            let key = new_input(c.clone());
            v.push(key.0);
            v.push(key.1);
        }
    }
    let ipsize = std::mem::size_of::<INPUT>() as i32;
    unsafe {
        SendInput(2 * s_vec.len() as u32,  v.as_mut_ptr(), ipsize);
    }
}

unsafe fn new_input(c: u16) -> (INPUT, INPUT) {
    let mut input_u1: INPUT_u = std::mem::zeroed();
    *input_u1.ki_mut() = KEYBDINPUT {
        wVk: if c == '\n' as u16 {VK_RETURN as u16} else if c == '\t' as u16 {VK_TAB as u16} else { 0 },
        dwExtraInfo: 0,
        wScan: if c == '\n' as u16 || c == '\r' as u16 { 0 } else { c }, //
        time: 0,
        dwFlags: KEYEVENTF_UNICODE //
    };
    let input1 = INPUT {
        type_: INPUT_KEYBOARD,
        u: input_u1
    };

    let mut input_u2: INPUT_u = std::mem::zeroed();
    *input_u2.ki_mut() = KEYBDINPUT {
        wVk: if c == '\n' as u16 {VK_RETURN as u16} else if c == '\t' as u16 {VK_TAB as u16} else { 0 },
        dwExtraInfo: 0,
        wScan: if c == '\n' as u16 || c == '\r' as u16 { 0 } else { c }, //
        time: 0,
        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP //
    };
    let input2 = INPUT {
        type_: INPUT_KEYBOARD,
        u: input_u2
    };

    (input1, input2)
}