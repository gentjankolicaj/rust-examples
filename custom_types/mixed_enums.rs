//1.Enum keyword allows for creation of a type which may be one of few variants.
//2.Any variant which is valid as 'struct' is also valid as enum.

enum SwitchButton {
    ON,
    OFF,
}

impl fmt::Display for SwitchButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ON => write!(f, "ON"),
            Self::OFF => write!(f, "OFF"),
        }
    }
}

fn main() {}
