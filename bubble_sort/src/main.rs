

fn bubble_sort(mut x: Vec<i32>) -> Vec<i32> {

    let mut complete: bool = false;

    while !complete {

        complete = true;
        for i in 0..x.len()-1 {
            if x[i+1] < x[i] {
                complete = false;
                x.swap(i, i+1);
            }
        }
    }
    x
    
}


fn main() {
    let elems: Vec<i32> = vec![10, 9, 12, 15, 11];
    let res = bubble_sort(elems);
    println!("{:?}", res);
}
