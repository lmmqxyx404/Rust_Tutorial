fn convert_by_question_mark(flag: i32) -> Result<i32, i64> {
    if flag % 2 > 0 {
        Err(64)
    } else {
        Ok(0)
    }
}

fn hello() -> Result<i32, i64> {
    let p = convert_by_question_mark(45)?;
    Ok(45)
}

#[test]
fn test() {
    let res = hello();
    let p = hello();
    assert_eq!(p.is_err(), true);
    assert_eq!(res.err().unwrap(), 64);
}
