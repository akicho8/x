fn main() {
    println!("{:?}", [3, 4].iter().cloned().collect::<Vec<_>>());
// ↓これと同じことらしい
println!("{:?}", [3, 4].iter().map(|&e| e).collect::<Vec<_>>());
}