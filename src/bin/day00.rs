use crate::io;

#[no_mangle]
pub fn day_00() {
    for line in io::stdin().lines() {
        let ln = line.unwrap();
        io::println!("{}", ln);
    }
}
