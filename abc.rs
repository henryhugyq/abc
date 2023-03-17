fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main(){
    let mut abc = vec![5, 7, 12, 3, 1, 2, 16, 8, 7,99,88];
    println!("排序前: {:?}",abc);
    bubble_sort(&mut abc);
    println!("排序后：{:?}",abc);
}