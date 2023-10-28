use crate::printa::printa;
use crate::printb::printb;
pub mod printa;
mod printb;

fn main() {

    //循环打印从’a’~’Z’ 之间的所有字符,由于ascii码编码顺序先大写后小写
    printb();

    //循环打印从’A’~’z’ 之间的所有字符
    printa();

}




