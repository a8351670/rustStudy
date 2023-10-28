pub(crate) fn printb() {
    println!("1. a到Z之间到字符为：");
    for str in ('Z'..='a').rev() {
        println!("{}", str);
    }
}