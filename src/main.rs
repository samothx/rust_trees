use chrono::Utc;
use rust_tree::util::make_list_usize;
use rust_tree::{list::linked_list::LinkedList, util::make_list_string};

const SAMPLES: usize = 10_000;

fn main() {
    let values = make_list_usize(SAMPLES);

    let mut list = LinkedList::new();
    let start = Utc::now();
    for val in &values {
        list.push(val);
    }
    let duration = Utc::now() - start;
    eprintln!(
        "duration for push(usize:{}): {} seconds",
        SAMPLES,
        duration.num_nanoseconds().unwrap() as f64 / 1e9
    );

    let mut list = LinkedList::new();
    let start = Utc::now();
    for val in &values {
        list.push1(val);
    }
    let duration = Utc::now() - start;
    eprintln!(
        "duration for push1(usize:{}): {} seconds",
        SAMPLES,
        duration.num_nanoseconds().unwrap() as f64 / 1e9
    );

    let values = make_list_string(SAMPLES);

    let mut list = LinkedList::new();
    let start = Utc::now();
    for val in &values {
        list.push(val.clone());
    }
    let duration = Utc::now() - start;
    eprintln!(
        "duration for push(string:{}): {} seconds",
        SAMPLES,
        duration.num_nanoseconds().unwrap() as f64 / 1e9
    );

    let mut list = LinkedList::new();
    let start = Utc::now();
    for val in &values {
        list.push1(val.clone());
    }
    let duration = Utc::now() - start;
    eprintln!(
        "duration for push1(string:{}): {} seconds",
        SAMPLES,
        duration.num_nanoseconds().unwrap() as f64 / 1e9
    );

    let start = Utc::now();
    for val in &values {
        list.contains(val);
    }
    let duration = Utc::now() - start;
    eprintln!(
        "duration for push1(string:{}): {} seconds",
        SAMPLES,
        duration.num_nanoseconds().unwrap() as f64 / 1e9
    );
}
