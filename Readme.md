# raytracer-rs

This is an explorative attempt to implement [Raytracing in one Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in idiomatic Rust, using only std ~without any further dependencies~ and `rand`.

![Sample](https://raw.githubusercontent.com/mkulke/raytracer-rs/main/my.jpg)

## Build

Built and tested with Rust 1.57.0

```bash
cargo b
```

## Run

```bash
cargo r --release --quiet > my.ppm
open my.ppm
```

## Test

```bash
cargo t
```
