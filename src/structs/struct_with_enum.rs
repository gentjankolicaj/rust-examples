#[derive(Debug, Copy, Clone)]
enum Switch {
    ON,
    OFF,
}

#[derive(Debug, Copy, Clone)]
enum RobotSex<'a> {
    MALE(&'a str),
    FEMALE(&'a str),
}

#[derive(Debug, Copy, Clone)]
enum RobotDS {
    SCALAR(u8),
    ARRAY(u8),
    MAP(u8),
    TREE(u8),
    BTREE(u8),

}

#[derive(Debug, Copy, Clone)]
struct Robot {
    id: i64,
    serial: i128,
    switch: Switch,
    sex: RobotSex,
    ds: RobotDS,
}

fn robot_factory() -> &'static [Robot] {
    const ROBOT_NUMBER: usize = 10;
    let mut array: [Robot; ROBOT_NUMBER] = [Robot { id: 0i64, serial: 0i128, switch: Switch::ON, sex: RobotSex::FEMALE("female"), ds: RobotDS::TREE(1) }; 10];
    return &array;
}

fn main() {
    let robot_slice = robot_factory();
    println!("{:?}", robot_slice);
}



