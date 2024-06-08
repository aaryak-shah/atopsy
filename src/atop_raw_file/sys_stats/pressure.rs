#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct Psi {
    avg10: f32,
    avg60: f32,
    avg300: f32,
    total: i64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pressure {
    present: i8, // using i8 to represent a bool
    future: [i8; 3],
    cpu_some: Psi,
    mem_some: Psi,
    mem_full: Psi,
    io_some: Psi,
    io_full: Psi,
}
