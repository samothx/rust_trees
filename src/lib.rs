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

pub mod testing {

    /*   #[allow(dead_code)]
      fn test_mut_ref() {
          let mut data = String::from("data");
          let mut dref0: Option<&mut String> = Some(&mut data);
          let mut dref1: Option<&mut String> = None;
          if let Some(data) = dref0 {
              data.push('!');
              dref1 = Some(data);
          } else {
              dref0 = None;
          }
      }

    */
}
