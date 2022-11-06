fn test_one() {
    let mut x = 5;
    println!("The val x is {x}");
    x = 7;
    println!("The val x is {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The val x is {THREE_HOURS_IN_SECONDS}");

    let x = 5;

    {
        // 覆盖前一个x
        let x = x + 1;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The val x is {x}");
    
    // .* 意味着name前一个位置是格式化的参数
    println!("{}, `{name:.*}` has 3 characters", "Hello", 3, name=1234.56);
    println!("{}, `{name:.*}` has 3 characters", "Hello", 3, name="1234.56");

    let tup = (100,0.2,3);
    let (x, y, _z) = tup;

    println!("The (x, y, z) is ({x}, {1}, {})", tup.2, y);

    another_function(9, 's');
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

    let y = {
        let x = value;
        // x + 1是一个语句，赋值给y。不能加分号，加了就是表达式，没有返回值
        x + 1
    };

    println!("loop_func({y}) == {0}", loop_func(y));
}

fn loop_func(mut counter: i32) -> i32{
    // break表示退出时传回的值
    loop {
        counter += 1;

        if counter % 10 == 0 {
            break counter * 2;
        }
    }
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if 9 == remaining {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}")
}

fn for_rev() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    // test_one();
    loop_label();
    for_rev();
}
