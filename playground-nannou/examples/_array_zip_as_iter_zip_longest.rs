fn main() {
    use itertools::Itertools;
println!("{:?}", [5, 6, 7, 8].iter().zip_longest(&[100, 200]).collect::<Vec<_>>());
println!("{:?}", [100, 200].iter().zip_longest(&[5, 6, 7, 8]).collect::<Vec<_>>());
}
