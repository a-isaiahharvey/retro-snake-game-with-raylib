use std::collections::VecDeque;

use raylib::{ffi::GetTime, math::Vector2};

use crate::LAST_UPDATE_TIME;

pub macro deque {
    () => {
        std::collections::VecDeque::new()
    },

    ($($x:expr),*) => {
        {
            let mut temp_deque = std::collections::VecDeque::new();
            $(
                temp_deque.push_back($x);
            )*
            temp_deque
        }
    }
}

pub fn element_in_deque(element: Vector2, deque: &VecDeque<Vector2>) -> bool {
    for item in deque {
        if *item == element {
            return true;
        }
    }
    false
}

pub fn event_triggered(interval: f64) -> bool {
    let current_time = unsafe { GetTime() };

    if current_time - unsafe { LAST_UPDATE_TIME } >= interval {
        unsafe {
            LAST_UPDATE_TIME = current_time;
        }
        return true;
    }
    false
}
