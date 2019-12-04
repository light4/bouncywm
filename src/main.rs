use x11::xlib::*;

use std::mem::MaybeUninit;
use std::ptr;
// use std::process;
use std::thread::sleep;
use std::time::Duration;
use std::ffi::CString;
use std::{u32, i32};

fn main() {
    unsafe {
        let mut attr_ = MaybeUninit::<XWindowAttributes>::uninit();
        let mut ev_ = MaybeUninit::<XEvent>::uninit();
        let dpy = XOpenDisplay(ptr::null());

        // process::exit(1);
        let root = XDefaultRootWindow(dpy);

    let e_str = CString::new("e").expect("CString::new failed");
    let e_keycode: KeyCode = XKeysymToKeycode(dpy, XStringToKeysym(e_str.as_ptr()));

    XGrabKey(dpy,
             i32::from(e_keycode),
             ControlMask,
             XDefaultRootWindow(dpy),
             True, GrabModeAsync, GrabModeAsync);

        XGrabButton(
            dpy,
            1,
            Mod1Mask,
            root,
            True,
            ButtonPressMask as u32,
            GrabModeAsync,
            GrabModeAsync,
            0,
            0,
        );

        let mut right = 1;
        let mut up = 1;
        let mut x;
        let mut y;

        loop {
            XNextEvent(dpy, ev_.as_mut_ptr());
            let ev = ev_.assume_init();

            if ev.key.keycode == u32::from(e_keycode) && ev.key.subwindow != 0 {
                XGetWindowAttributes(dpy, ev.button.subwindow, attr_.as_mut_ptr());
                let attr = attr_.assume_init();

                x = 40;
                y = 40;
                loop {
                    XMoveWindow(dpy, ev.button.subwindow, x, y);
                    XSync(dpy, 0);
                    if x > 1280 - attr.width {
                        right = 0;
                    } else if x < 0 {
                        right = 1;
                    }

                    if y < 0 {
                        up = 1;
                    } else if y > 800 - attr.height {
                        up = 0;
                    }

                    if right == 1 {
                        x += 8;
                    } else {
                        x -= 6
                    }
                    if up == 1 {
                        y += 6
                    } else {
                        y -= 4
                    }
                    sleep(Duration::from_micros(100));
                }
            }
        }
    }
}
