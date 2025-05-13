use std::fmt;

//1.Enum keyword allows for creation of a type which may be one of few variants.
//2.Any variant which is valid as 'struct' is also valid as enum.
//3.An enum variant may be : 'unit-struct','tuple-struct','c-struct' like.

enum WebEvent {
    //unit-struct like
    PageLoad,
    PageUnload,

    //tuple-struct like
    KeyPress(char),
    Paste(String),

    //c-struct like
    Click { x: i64, y: i64 },
}

impl fmt::Display for WebEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebEvent::PageLoad => write!(f, "WebEvent::PageLoad"),
            WebEvent::PageUnload => write!(f, "WebEvent::PageUnload"),
            WebEvent::KeyPress(c) => write!(f, "WebEvent::KeyPress {}", c),
            WebEvent::Paste(s) => write!(f, "WebEvent::Paste {}", s),
            WebEvent::Click { x, y } => write!(f, "WebEvent::Click {},{}", x, y),
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            print!("clicked at x:{}, y:{}\r\n", x, y);
        }
    }
}

fn main() {
    let page_load = WebEvent::PageLoad;
    let page_unload = WebEvent::PageUnload;
    let key_press = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 10, y: 20 };

    println!("WebEvent page loaded {}", page_load);
    println!("WebEvent page unloaded {}", page_unload);
    println!("WebEvent key pressed '{}'", key_press);
    println!("WebEvent paste {}", paste);
    println!("WebEvent click  {}", click);

    //call inspect function
    inspect(page_load);
    inspect(page_unload);
    inspect(key_press);
    inspect(paste);
    inspect(click);
}
