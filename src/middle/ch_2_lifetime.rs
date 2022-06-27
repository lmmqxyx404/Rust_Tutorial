/*
 * @Author: Lmmqxyx
 * @Date: 2022-06-21 07:42:10
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-06-28 03:30:36
 * @FilePath: \Rust_Tutorial\src\middle\ch_2_lifetime.rs
 * @Description:
 */
/*
A longer lifetime can be coerced into a shorter one so that it works inside a scope it normally wouldn't work in.
This comes in the form of inferred coercion by the Rust compiler, and also in the form of declaring a lifetime difference:
 */
// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
    first
}
#[test]
fn lifetime() {
    // pay aattention to the mutable reference
    let first = 45;
    {
        let second = 2;
        // about lifetime coercion
        assert_eq!(90, multiply(&first, &second));
        assert_eq!(45, *choose_first(&first, &second));
    }
}
