/*
 * @Author: Jerry.Yang
 * @Date: 2024-10-16 10:57:00
 * @LastEditors: Jerry.Yang
 * @LastEditTime: 2024-10-29 11:27:39
 * @Description:
 */
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod service;
pub mod volcengine;
