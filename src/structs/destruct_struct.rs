#[derive(Debug)]
struct Coord(f64, f64);

#[derive(Debug)]
struct CityA {
    name: String,
    center: Coord,
    radius: f64,
}

#[derive(Debug)]
struct CityB {
    name: String,
    center: (f64, f64),
    radius: f64,
}


fn main() {
    let tirane_city = CityA { name: "Tirane".to_string(), center: Coord(41.327953, 19.819025), radius: 10.0 };
    let Coord(tirane_lat, tirane_lon) = tirane_city.center;
    println!("tirane_city: {:?}", tirane_city);
    println!("destruct tirane_city.coord lat:{:?},lon:{:?}", tirane_lat, tirane_lon);

    //
    let shkodra_city = CityB { name: "Shkodra".to_string(), center: (42.068056, 19.511944), radius: 5.0 };
    let (shkodra_lat, shkodra_lon) = shkodra_city.center;
    println!("shkodra_city: {:?}", shkodra_city);
    println!("destruct shkodra_city.center lat:{:?},lon:{:?}", shkodra_lat, shkodra_lon);
}
