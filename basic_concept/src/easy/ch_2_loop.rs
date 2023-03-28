/*
 * @Author: Lmmqxyx
 * @Date: 2022-04-11 11:07:53
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-04-11 11:29:52
 * @FilePath: \Rust_Tutorial\src\simple\ch_2_loop.rs
 * @Description:
 *  the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.
 */
fn testloop() -> char {
    let mut i = 0;
    ('flag: loop {
        for j in 0..100 {
            i = j;
            if i == 5 {
                break 'flag i;
            }
        }
    }) as u8 as char
}

#[test]
fn sloop() {
    assert_eq!(testloop() as u8 + '0' as u8, '5' as u8);
}
