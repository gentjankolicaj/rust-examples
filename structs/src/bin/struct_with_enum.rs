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
struct Robot<'a> {
    id: i64,
    serial: i128,
    switch: Switch,
    sex: RobotSex<'a>,
    ds: RobotDS,
}

fn robot_factory<'a>() -> Vec<Robot<'a>> {
    const ROBOT_NUMBER: usize = 10;
    let vec:Vec<Robot<'a>> = vec![Robot{
        id: 0i64,
        serial: 0i128,
        switch: Switch::ON,
        sex: RobotSex::FEMALE("female"),
        ds: RobotDS::TREE(1),
    }];
    
    //return vector address value
     vec
}

fn main() {
    let robot_vec = robot_factory();
    println!("{:?}", robot_vec);
}
