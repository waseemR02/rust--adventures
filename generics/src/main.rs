fn max<T: std::cmp::PartialOrd>(number_list: &[T]) -> &T {
    // bounds with PartialOrd
    let mut max_num = &number_list[0];

    for num in number_list {
        if num >= max_num {
            max_num = num;
        }
    }

    max_num
}
fn main() {
    let number_list = vec![4, 2, 1, 3, 5];

    println!(
        "Largest Number in the series {:?} is {}",
        number_list,
        max(&number_list)
    );
}
