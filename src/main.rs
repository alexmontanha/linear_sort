
fn main() {
    let list = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let item = 5;
    let result = linear_search(&list, &item);
    println!("Result: {:?}", result);
}

fn linear_search<T: PartialEq>(list: &[T], item: &T) -> Option<usize> {
    for (i, val) in list.iter().enumerate() {
        if val == item {
            return Some(i);
        }
    }
    None
}
