fn main() {
    println!("Hello, world!");

    let mut a: Vec<i32> = Vec::new();

    a.push(22);
    a.push(21);
    a.push(20);
    a.push(1);
    println!("{:?}", a);

    let b = get_concatenation(& a);
    println!("{:?}", b);
}

// my version
fn get_concatenation(nums: & Vec<i32>) -> Vec<i32> {

    let mut ret_list: Vec<i32> = Vec::new();

    for num in nums.iter() {
        ret_list.push(*num);
    }

    for num in nums.iter() {
        ret_list.push(*num);
    }

    ret_list
}

// optimized version
fn get_concatenation_clean(nums: &Vec<i32>) -> Vec<i32> {
    let mut ret_list = nums.clone();
    ret_list.extend(nums);
    ret_list
}
