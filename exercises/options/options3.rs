// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// I AM N0T DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // _&_ denotes that your pattern expects a reference to an object. Hence _&_ is a part of said pattern: _&Foo_ matches different objects than _Foo_ does.
    // _ref_ indicates that you want a reference to an unpacked value. It is not matched against: _Foo(ref foo)_ matches the same objects as _Foo(foo)_.
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
