
struct Mono<T>{
    t: T
}

struct Di<T,U>{
    t: T,
    u: U,
}

struct Tri<T,U,V>{
    t: T,
    u:U,
    v:V,
}

impl<T> Mono<T>{
    fn new(t: T) -> Mono<T>{
        Mono{t }
    }
}

impl<T,U> Di<T,U>{
    fn new(t: T,u: U) -> Di<T,U>{
        Di{t,u}
    }
}

impl<T,U,V> Tri<T,U,V>{
    fn new(t: T,u:U,v:V) -> Tri<T,U,V>{
        Tri{t,u,v}
    }
}

