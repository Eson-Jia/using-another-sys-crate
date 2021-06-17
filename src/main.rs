extern "C" {
    pub fn version() -> *const u8;
}


fn main() {
    unsafe {
        let version = version();
        println!("the zlib version is {:?}", version);
    }
}
