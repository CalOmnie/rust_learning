fn main() {
use walkdir::WalkDir;
    let mut res = 0;
    for entry in WalkDir::new("/dev").into_iter().filter_map(|e| e.ok()) {
        let _unwrapped = entry.path().display();
        res += 1
    }
    println!("{}", res)
}
