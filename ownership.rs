fn main(){
    let s1: String = String::from("Hello");
    let s2: &str = "World";
    println!("{}", add_string(s1, s2));
}

//sは変更したいのでミュータブルのString型で受け取る
//tは変更するう必要がないので参照だけを受け取る
fn add_string(s: String, t: &str) -> String{
    s + t
}
