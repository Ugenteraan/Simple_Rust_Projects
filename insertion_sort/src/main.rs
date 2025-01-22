

// fn insertion_sort(x: &[i32]) -> Vec<i32> {

    
//     let total_length: usize = x.len();

//     let mut result: Vec<i32> = Vec::with_capacity(total_length);
//     result.push(x[0]);

//     for i in 1..total_length {

//         let mut insertion_index: usize = i;
//         let curr_elem = x[i];

//         for j in (0..i).rev() {

//             if curr_elem < x[j] {
//                 insertion_index -= 1
//             }
//         }
//         result.insert(insertion_index, curr_elem);
//     }

//     result

// }

fn insertion_sort(mut x: Vec<i32>) -> Vec<i32>{

    let total_size = x.len();
    

    for i in 1..total_size {

        let curr_elem = x[i];
        let mut j = i;

        while j > 0 && curr_elem < x[j-1] {
            x[j] = x[j-1];
            j -= 1;
        }

        x[j] = curr_elem;

    }

    x


}




fn main() {

    let x: Vec<i32> = vec![9,2,5,1,6,10];

    let result: Vec<i32> = insertion_sort(x);

    println!("{:?}", result);
}
