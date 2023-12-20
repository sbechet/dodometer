use pcg::Pcg;

fn main() {
    let mut pcg: Pcg = Pcg::default();

    for _ in 0..100 {
        println!("u64: {}", pcg.genu64());
    }
}
