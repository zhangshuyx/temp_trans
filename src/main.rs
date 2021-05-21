fn main() {
    let f: f64 = 100.0;
    println!("The C value of F: {} is {:.2}.", f, temp_trans(f));
}

fn temp_trans(f: f64) ->f64 {
    (f - 32.0) / 1.8
}
