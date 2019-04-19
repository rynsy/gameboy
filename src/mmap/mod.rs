pub struct MMap {
    data: Vec<u16>,
}

impl Default for MMap {
    fn default() -> MMap {
        MMap{
            data: Vec::with_capacity(0xFFFF),
        }
    }
}
