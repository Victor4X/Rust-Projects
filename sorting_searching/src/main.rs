fn main() {
    println!("Hello, world!");
}

fn bubble_sort(slice: &mut [isize]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..slice.len() {
            if slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    bubble_sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}