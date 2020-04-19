mod calculate;

fn main() {
    let board_array: [&str; 3] = ["As", "Ks", "Jd"];
    let playerscards_array: [&str; 4] = ["8s", "7s", "?", "?"];
    println!("{:?}", calculate::calc_hand(&board_array,&playerscards_array));
}
