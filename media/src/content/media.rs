fn main() {
    let mut trial = vec![1, 2, 3];

    let trying = &trial[0];

    print!("Trying: {}, ", trying);
    let last = trial.push(4);
    println!("Last: {:?}", last);
}
