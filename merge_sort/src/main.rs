
fn sort_list(list1: &[i32], list2: &[i32]) -> Vec<i32> {

    let mut result = Vec::with_capacity(list1.len() + list2.len());

    let mut iter1 = list1.iter();
    let mut iter2 = list2.iter();

    let mut a = iter1.next();
    let mut b = iter2.next();



    while let (Some(&val1), Some(&val2)) = (a, b) {

        if val1 <= val2 {
            result.push(val1);
            a = iter1.next();
        }else{
            result.push(val2);
            b = iter2.next();
        }
    }


    result.extend(a.into_iter().chain(iter1));
    result.extend(b.into_iter().chain(iter2));
    result

}




fn merge_sort(x: &[i32]) -> Vec<i32> {

    //base case
    if x.len() <=1 {
        return x.to_vec();
    }

    //break the vector.
    let half = x.len()/2;
    
    let left = &x[..half];
    let right = &x[half..];

    let sorted_left = merge_sort(left);
    let sorted_right = merge_sort(right);

    sort_list(&sorted_left, &sorted_right)
    
}





fn main() {
    
    let x: Vec<i32> = vec![5,2,10,19,10,1,20];

    let y  = merge_sort(&x);
    println!("{:?}", y);

}
