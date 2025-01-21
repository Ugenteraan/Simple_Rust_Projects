
fn sort_list(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32> {

    let mut i = 0;
    let mut j = 0;

    let mut result: Vec<i32> = Vec::new();

    while i < list1.len() && j < list2.len() {

        if list1[i] <= list2[j] {
            result.push(list1[i]);
            i += 1;
            continue;
        }

        result.push(list2[j]);
        j += 1;

    }

    while i < list1.len() {
            result.push(list1[i]);
            i += 1;
        }

    while j < list2.len() {
        result.push(list2[j]);
        j += 1;
    }

    result

}




fn merge_sort(x: Vec<i32>) -> Vec<i32> {

    //base case
    if x.len() <=1 {
        return x;
    }

    //break the vector.
    let (left, right) = x.split_at(x.len()/2);

    let list1 = merge_sort(left.to_vec());
    let list2 = merge_sort(right.to_vec());

    sort_list(list1, list2)
    
    
}





fn main() {
    
    let x: Vec<i32> = vec![5,2,10,19,10,1,20];

    let y  = merge_sort(x);
    println!("{:?}", y);

}
