fn main() {
    let plus_one = |x: i32| x + 1;

    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    };

    assert_eq!(4, plus_two(2));

    let mut num = 5;

    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }

    assert_eq!(5, num);
}
