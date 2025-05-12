## Creating a Library
Let's create a library, and then see how to link it to another crate.

```
In funcs.rs:

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

```

Libraries get prefixed with "lib", and by default they get named after their crate file, but this default name can be overridden by passing the --crate-name option to rustc or by using the crate_name attribute.

```shell
rustc --crate-type=lib funcs.rs && ls lib*
```

