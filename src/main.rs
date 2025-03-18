fn main() {
    dbg!(smaps::read_all("/proc/self/smaps".as_ref()).unwrap());
}
