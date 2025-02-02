#![no_std]

#[no_mangle]
pub extern "C" fn add_u32(left: u32, right: u32) -> u32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_u32(2, 2);
        assert_eq!(result, 4);
    }
}
