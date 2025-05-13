//1.In rust-lang we have 'match' instead of 'switch'
//2.In rust-lang 'match' is an expression.
//3.'match' being an expression means that we can use it in assigment statements;
//4.'match' arms must cover all possible values
//5.'match' can be used to destruct tuples
//6.'match' can be used to destruct arrays
//7.'match' can be used to destruct enums

//Define enum
// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Color {
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
    DUMMY(u32),
}

fn main() {
    let first_color = Color::RGB(122, 17, 40);

    println!("1.What color is it?");
    //destruct enum with match
    match first_color {
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        Color::DUMMY(d) => {
            println!("Dummy: {}", d);
        } // Don't need another arm because all variants have been examined
    }

    let second_color = Color::DUMMY(1);
    println!("2.What color is it ?");
    //destruct enum with match
    match second_color {
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        Color::DUMMY(d) => {
            println!("Dummy: {}", d);
        } // Don't need another arm because all variants have been examined
    }
}
