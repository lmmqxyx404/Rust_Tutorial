/*
 * @Author: Lmmqxyx
 * @Date: 2022-07-07 22:46:05
 * @LastEditors:
 * @LastEditTime: 2022-07-07 22:46:22
 * @FilePath: \Rust_Tutorial\src\hard\mod.rs
 * @Description:
 */
mod ch_0_hello;
mod ch_1_chain;
mod ch_2_vec_iter;
mod ch_3_fn;
pub mod ch_4_copy_clone;
pub mod ch_5_list_node;
pub mod ch_6_concurrency;
pub mod ch_7_unsafe;
pub mod ch_8_pointer;
pub mod ch_9_std_misc;

/// In order to explore the effect of ?.
/// In the inner of a function(return value is Result),if a statement call another function(return value is Result too).
/// Then the called function retrurn Err, using ? in the end of the statement will cause the original function return Err.
pub mod ch_10_unwrap;