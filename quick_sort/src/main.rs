// use rand::Rng;
// use std::env;

// fn quicksort(mut x: Vec<i32>, mut start: usize, pivot:usize) -> Vec<i32> {

//     if pivot <= 1 || start == pivot{
//         return x;
//     }

//     let mut swap_ptr = start;
//     for curr_index in start..=pivot {

//         if x[curr_index] > x[pivot] {
//             continue;
//         }else{
//             if x[curr_index] < x[swap_ptr]{
//                 x.swap(curr_index, swap_ptr);
//                 swap_ptr += 1;
//             }

//             if curr_index == pivot {
//                 swap_ptr += 1;
//                 x.swap(curr_index, swap_ptr);
//             }

//         }

//     }

//     let left_half = quicksort(x, 0 as usize, swap_ptr);
//     quicksort(left_half, swap_ptr, pivot)

// }

fn quicksort(x: &mut [i32]) -> () {

    if x.len() <= 1 {
        return ;
    }

    let pivot_index = partition(x); //we can pass x directly here since it's already a &mut.

    quicksort(&mut x[..pivot_index]); //here, we need to specify &mut because x[..] is slicing it and it's non mutable by default.
    quicksort(&mut x[pivot_index+1..]);

}



fn partition(x: &mut [i32]) -> usize{



    let pivot_index: usize = x.len() - 1;
    let pivot_element: i32 = x[pivot_index];

    let mut i: usize = 0;

    for j in 0..=pivot_index {

        if x[j] < pivot_element {
            x.swap(j, i);
            i += 1;
        }
    }

    x.swap(i, pivot_index);

    return i;
}





fn main() {


    let mut x: Vec<i32> = vec![8,2,5,0,1,8,7,6,9,4];


    quicksort(&mut x);

    println!("{:?}", x);
}
