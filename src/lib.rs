pub mod list;
pub mod tree;
pub mod util {
    pub fn make_list_string(len: usize) -> Vec<String> {
        let mut res = Vec::new();
        res.reserve(len);
        for idx in 1..=len {
            res.push(idx.to_string())
        }
        res
    }
    pub fn make_list_usize(len: usize) -> Vec<usize> {
        let mut res = Vec::new();
        res.reserve(len);
        for idx in 1..=len {
            res.push(idx)
        }
        res
    }
}
