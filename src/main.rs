mod sorting;

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];

    // sorting::quicksort(&mut numbers);
    // println!("quicksort {:?}", numbers);

    // sorting::bubble_sort(&mut numbers);
    // println!("bubblesort {:?}", numbers);

    sorting::merge_sort(&mut numbers);
    println!("mergesort {:?}", numbers);
}
