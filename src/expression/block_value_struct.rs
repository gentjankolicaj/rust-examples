//1.In rust blocks are expressions to, which mean that it can be used as value in assignments.
//1.In rust blocks are expressions too, which mean that it can be used as value in assignments.
//2.If the last expression of the block ends with a semicolon, the return value will be ().

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}
fn main() {
    let x = Point { x: 0, y: 0 };

    let y = {
        let tmp = 20;
        let mut tmp1 = 13;
        tmp1 = tmp + tmp1;

        //last expression in block is used as value in block variable
        //If the last expression of the block ends with a semicolon, the return value will be ().
        Point { x: tmp, y: tmp1 }
    };

    let z = {
        let tmp = 40;
        let mut tmp1 = 15;
        tmp1 = tmp + tmp1;

        //last expression in block is used as value in block variable
        //If the last expression of the block ends with a semicolon, the return value will be ().
        Point { x: tmp, y: tmp1 }
    };

    println!("x:{:?},y:{:?},z:{:?}", x, y, z);
}
