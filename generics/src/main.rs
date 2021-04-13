fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = get_largest(&char_list);
    println!("The largest char is {}", result);
}

fn get_largest<T: Copy + PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
