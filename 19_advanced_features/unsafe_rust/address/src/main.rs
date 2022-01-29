use std::slice;

fn main() {
    let address = 0x01234usize;

    let r = address as *mut i32;

    let slice = unsafe {slice::from_raw_parts_mut(r, 5)};

    dbg!(slice);

}
