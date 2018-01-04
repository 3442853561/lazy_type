extern crate lazy_type;

use lazy_type::array::Array;

fn main() {
    let bar = 10;
    let mut foo: Array<u8> = Array::new(0u8, bar);
    foo[1] = 1;
    println!("{:?}", foo);
}
