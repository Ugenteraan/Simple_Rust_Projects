

fn selection_sort(x: &mut [i32]) -> () {

    if x.len() <= 1 {
        return;
    }

    let mut smallest_value = x[0]; //temporary first smallest number assignment.
    let mut smallest_value_index = 0;

    for i in 1..x.len() {

        if x[i] < smallest_value {
            smallest_value_index = i;
            smallest_value = x[i];
        }
    }
    x.swap(0, smallest_value_index);

    selection_sort(&mut x[1..]);

}




fn main() {
    
    let mut x: Vec<i32> = vec![5,2,4,6,5,7,19,2,10,2,1,0,7];

    selection_sort(&mut x);

    println!("{:?}", x);


}
