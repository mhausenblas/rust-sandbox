fn main() {
    let needle = 42;
    let haystack = [1, 1, 2, 4, 42, 423, 500];

    for item in haystack.iter() {
        if item == &needle {
            println!("Found ya: {}", item);
        }
    }
}
