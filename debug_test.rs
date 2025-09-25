// Temporary debug test
use bgi::*;

fn main() {
    let mut driver = 9; // VGA
    let mut mode = 999; // Invalid mode

    println!("Before initgraph: driver={}, mode={}", driver, mode);
    initgraph(&mut driver, &mut mode, "");
    println!("After initgraph: driver={}, mode={}", driver, mode);

    let result = graphresult();
    println!("graphresult(): {:?}", result);

    closegraph();
}
