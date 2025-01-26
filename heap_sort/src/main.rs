/**
 * 1) Treat the input array as a binary tree.
 * 2) Start from the last non-leaf nodes and heapify them.
 * 3) The process has to be recursively applied to every other non-leaf nodes above it and every node below it.
 * **/


fn heap_sort(x: &mut [i32]) -> (){

    let n = x.len();

    //the position of the last non-leaf nodes starts from (n//2)-1 all the way to 0. 
    //rust automatically floors them since they are ints and not floats.
    for i in (0..(n/2)).rev() {
        heapify(x, i, n);
    }

    for j in (1..n).rev() {
        x.swap(j, 0);
        heapify(x, 0, j);
    }
}


fn heapify(x: &mut [i32], root_idx:usize, n:usize) -> () {

    let mut largest = root_idx;
    let left = 2*root_idx + 1;
    let right = 2*root_idx + 2;

    if left < n && x[left] > x[largest] {
        largest = left;
    }

    if right < n && x[right] > x[largest]{
        largest = right;
    }

    if largest != root_idx {
        x.swap(root_idx, largest);
        heapify(x, largest, n);
    }

}




fn main() {
    let mut x: Vec<i32> = vec![5,2,4,6,5,7,19,2,10,2,1,0,7];

    heap_sort(&mut x);

    println!("{:?}", x);


}
