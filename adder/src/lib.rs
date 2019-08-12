#[test]
#[should_panic(expected = "assertion failed")]
fn it_works() {
    assert_eq!("Hello", "world");
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn add_two_works() {
    assert_eq!(4, add_two(2));
}

#[test]
#[ignore]
fn expensive_test() {
    let ten_secs = std::time::Duration::from_secs(10);
    std::thread::sleep(ten_secs);
    assert_eq!(10, ten_secs.as_secs());
}

pub mod mod_sample;
