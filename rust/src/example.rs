fn main(
    let array = vec![1, 2, 3, 4, 5];
    let mut iter = array.iter();
    let mut sum = 0;
    while let Some(x) = iter.next() {
        sum += x;
    }
    println!("Sum: {}", sum);
)