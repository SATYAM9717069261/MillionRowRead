use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    os::fd::AsRawFd,
};

fn main() {
    let file = File::open("inp.txt").unwrap(); // @return Result<>
    let map = mmap(&file);
    let mut stats = HashMap::<Vec<u8>, (f64, f64, usize, f64)>::new(); // min, sum, count, max
    for line in map.split(|data| *data == b'\n') {
        if line.is_empty() {
            break;
        }
        let mut fields = line.rsplitn(2, |c| *c == b';');
        let _tmp = fields.next().unwrap();
        let name = fields.next().unwrap();

        let tmp: f64 = unsafe { std::str::from_utf8_unchecked(_tmp) }
            .parse()
            .unwrap();
        let ptr = match stats.get_mut(name) {
            Some(data) => data,
            None => stats
                .entry(name.to_vec())
                .or_insert((f64::MAX, 0.0, 0, f64::MIN)),
        };
        ptr.0 = ptr.0.min(tmp);
        ptr.1 += tmp;
        ptr.2 += 1;
        ptr.3 = ptr.3.max(tmp);
    }
    display(&stats);
}
fn display(stats: &HashMap<Vec<u8>, (f64, f64, usize, f64)>) {
    let stats = BTreeMap::from_iter(stats);
    let mut iter = stats.into_iter().peekable();
    print!("{{");
    while let Some((station, (min_tmp, sum, count, max_tmp))) = iter.next() {
        print!(
            "{:?}={min_tmp:.1}/{:.1}/{max_tmp:.1}",
            station,
            sum / (*count as f64)
        );
        if iter.peek().is_some() {
            print!(", ");
        }
    }
    print!("}}");
}

fn mmap(f: &File) -> &'_ [u8] {
    let len = f.metadata().unwrap().len();
    unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            len as libc::size_t,
            libc::PROT_READ,
            libc::MAP_SHARED,
            f.as_raw_fd(),
            0,
        );
        if ptr == libc::MAP_FAILED {
            panic!("{:?}", std::io::Error::last_os_error())
        } else {
            if libc::madvise(ptr, len as libc::size_t, libc::MADV_SEQUENTIAL) != 0 {
                panic!("{:?}", std::io::Error::last_os_error())
            }
            return std::slice::from_raw_parts(ptr as *const u8, len as usize);
        }
    }
}
