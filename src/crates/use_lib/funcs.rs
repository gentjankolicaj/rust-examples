#[derive(Debug)]
pub struct Mono<T> {
    t: T,
}

#[derive(Debug)]
pub struct Di<T, U> {
    t: T,
    u: U,
}

#[derive(Debug)]
pub struct Tri<T, U, V> {
    t: T,
    u: U,
    v: V,
}

impl<T> Mono<T> {
    pub fn new(t: T) -> Mono<T> {
        Mono { t }
    }
}

impl<T, U> Di<T, U> {
    pub fn new(t: T, u: U) -> Di<T, U> {
        Di { t, u }
    }
}

impl<T, U, V> Tri<T, U, V> {
    pub fn new(t: T, u: U, v: V) -> Tri<T, U, V> {
        Tri { t, u, v }
    }
}
