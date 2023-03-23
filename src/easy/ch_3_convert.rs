/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-07 18:07:57
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-05-13 10:28:56
 * @FilePath: \Rust_Tutorial\src\simple\ch_3_convert.rs
 * @Description:
 */

// basic type conversions
fn char_to_u8(a: char) -> u8 {
    a as u8
}

fn i32_to_char(a: i32) -> char {
    a as u8 as char
}

fn string_to_char_array(_s: String) -> Vec<char> {
    vec!['a']
}

struct pp {}

struct qq {
    qq: i32,
}

impl From<pp> for qq {
    fn from(value: pp) -> Self {
        Self { qq: 32 }
    }
}

#[test]
fn var() {
    let mut a = 'a';
    // println!("{}",a as u8);
    assert_eq!(97, char_to_u8(a));
    assert_eq!(a, i32_to_char(97));
    assert_eq!(1, std::mem::size_of_val(&(a as u8)));

    
    let p: qq = pp {}.into();
    let q = qq { qq: 32 };
    assert_eq!(q.qq, p.qq);
}
