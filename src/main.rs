use std::f64::consts::PI;

#[no_mangle]
#[inline(never)]
fn const_fold() -> i32 {
    let b = 20 * 50 + 80;
    return b;
}

#[no_mangle]
#[inline(never)]
fn get_complex_double() -> f64 {
    let n = 5.2 / (PI * PI);
    return n;
}

#[no_mangle]
#[inline(never)]
fn f(x: i32) -> i32 {
    let a = 30 + x;
    let b = 9;
    let mut c = b * 3;
    if c > 10 {
        c = a - 10 + c;
    }
    for i in 1..b {
        c += i * x;
    }
    return c * (60 / b);
}

#[allow(dead_code)]
fn f_optimized(x: i32) -> i32 {
    return 6 * x + 282;
}

#[no_mangle]
#[inline(never)]
fn multiply_by_8(x: u32) -> u32 {
    return x * 8;
}

#[no_mangle]
#[inline(never)]
fn divide_by_16(x: u32) -> u32 {
    return x / 16;
}

#[no_mangle]
#[inline(never)]
fn multiply_by_30(x: u32) -> u32 {
    return x * 30;
}

#[no_mangle]
#[inline(never)]
fn divide_by_3(x: u32) -> u32 {
    return x / 3;
}

#[no_mangle]
#[inline(never)]
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[no_mangle]
#[inline(never)]
fn sub(x: i32, y: i32) -> i32 {
    return add(x, -y);
}

#[no_mangle]
#[inline(never)]
fn print(x: i32) {
    println!("{}", x);
}

#[no_mangle]
#[inline(never)]
fn loop_print() {
    for i in 0..100 {
        print(i * 999);
    }
}

#[no_mangle]
#[inline(never)]
fn loop_print2() {
    let mut x = 1;
    while x * x < 1000000 {
        print(x);
        x += 1;
    }
}

#[allow(dead_code)]
fn loop_print2_optimized() {
    let mut x = 1;
    while x < 1000 {
        print(x);
        x += 1;
    }
}

#[no_mangle]
#[inline(never)]
fn sum_to_x(x: u32) -> u32 {
    let mut sum = 0;
    for i in 0..x + 1 {
        sum += i;
    }
    return sum;
}

#[no_mangle]
#[inline(never)]
fn square_sum_to_x(x: u32) -> u32 {
    let mut sum = 0;
    for i in 0..x + 1 {
        sum += i * i;
    }
    return sum;
}

#[no_mangle]
#[inline(never)]
fn loop_print3() {
    for i in 1..100 {
        print(i);
    }
}

fn main() {
    // 1. Constant folding
    const_fold();
    get_complex_double();

    // 2. Constant propagation
    f(10);

    // 3. Multiplication and Division optimization
    multiply_by_8(10);
    divide_by_16(1200);
    let x = multiply_by_30(10);
    println!("{}", x);
    let x = divide_by_3(9);
    println!("{}", x);

    // 4. Function inlining
    sub(10, 4);

    // 5. Strength reduction
    loop_print();

    // 6. Canonicalize Induction Variables
    loop_print2();

    // 7. Loop unrolling
    loop_print3();

    // 8. Sum-Product Optimization
    sum_to_x(100);
    square_sum_to_x(10);
}
