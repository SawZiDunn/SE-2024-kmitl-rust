use std::mem::{size_of, transmute};
use std::slice::from_raw_parts;

#[allow(dead_code)]
struct Data(i32, char, bool);

fn main() {
    let data1 = Data(5, 'a', true);

    let bytes: &[u8] =
        unsafe { from_raw_parts(&data1 as *const Data as *const u8, size_of::<Data>()) };

    print!("Byte representation of Data Struct: ");

    println!("{:?}", bytes);

    // let bytes2: [u8; size_of::<Data>()] = unsafe { transmute(data1) };

    // println!("{:?}", bytes2);
}
