use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inp.txt").unwrap(); // @return Result<>
    let buf_reader = BufReader::new(file);
    let mut stats = HashMap::<String, (f64, f64, usize, f64)>::new(); // min, sum, count, max
    for line in buf_reader.lines() {
        match line {
            Ok(data) => {
                if let Some((name, tmp_)) = data.split_once(';') {
                    let tmp: f64 = tmp_.parse().expect("");
                    let ptr = match stats.get_mut(name) {
                        Some(data) => data,
                        None => {
                            stats
                                .entry(name.to_string())
                                .or_insert((f64::MAX, 0.0, 0, f64::MIN))
                        }
                    };
                    ptr.0 = ptr.0.min(tmp);
                    ptr.1 += tmp;
                    ptr.2 += 1;
                    ptr.3 = ptr.3.max(tmp);
                } else {
                    println!("Split Error: {:?} ", data);
                }
            }
            Err(err) => {
                print!("Issue in Buffer Reading {err}");
            }
        }
    }
    display(&stats);
}
fn display(stats: &HashMap<String, (f64, f64, usize, f64)>) {
    let stats = BTreeMap::from_iter(stats);
    let mut iter = stats.into_iter().peekable();
    print!("{{");
    while let Some((station, (min_tmp, sum, count, max_tmp))) = iter.next() {
        print!(
            "{station}={min_tmp:.1}/{:.1}/{max_tmp:.1}",
            sum / (*count as f64)
        );
        if iter.peek().is_some() {
            print!(", ");
        }
    }
    print!("}}");
}
