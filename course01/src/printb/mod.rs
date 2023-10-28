
pub(crate) fn printb(){

    let mut i = 0;
    for   str in 'a'..='Z' {
        println!("{}", str);
        i = 1;
    }

    if i == 0 {
        println!("2. a到Z之间无字符");
    }
}