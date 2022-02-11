// use kwindex::*;

// fn main() {
// let mut kwindex = KWIndex::new();
// kwindex = kwindex.extend_from_text("It ain't over untïl it ain't, over.");
// println!("{:?}", kwindex);
// }

//Discovering the terrors of the turbofish
fn main() {
    let (oh, woe, is, me) = ("the", "Turbofish", "remains", "undefeated");
    let x: (bool, bool) = (oh < woe, is > (me));
    println!("{:?}", x);
}
