fn main() {
let text="loveviosam!!";
let ch= text.chars().next().unwrap();
for i in 0..text.len() {
    let echar =text.chars().nth(i).unwrap();
    if echar == 'o'{
    println!("{}",i);

    }

}
}
