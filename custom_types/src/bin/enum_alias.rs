use std::fmt;

//1.Enum keyword allows for creation of a type which may be one of few variants.
//2.Any variant which is valid as 'struct' is also valid as enum.
//3.An enum variant may be : 'unit-struct','tuple-struct','c-struct' like.
//4.We use Aliases when referring to type is generic/verbose/inappropriate.
//5.We use aliases to refer to types of enums.
//6.If you use a type alias, you can refer to each enum variant via its alias.
// This might be useful if the enum's name is too long or too generic, and you want to rename it.

//NOTE: Self is alias

enum ArithmeticOps {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

//create a type alias
type Ops = ArithmeticOps;

//NOTE: Self is alias
impl fmt::Display for ArithmeticOps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ADD => write!(f, "ADD"),
            Self::SUBTRACT => write!(f, "SUBTRACT"),
            Self::MULTIPLY => write!(f, "MULTIPLY"),
            Self::DIVIDE => write!(f, "DIVIDE"),
        }
    }
}

fn main() {
    let add = Ops::ADD;
    let subtract = Ops::SUBTRACT;

    println!("ArithmeticOps.ADD with alias and variable add {}", add);
    println!(
        "ArithmeticOps.SUBTRACT with alias and variable subtract {}",
        subtract
    );
}
