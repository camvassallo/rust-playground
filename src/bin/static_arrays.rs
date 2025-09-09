use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let mut test_list: Vec<i32> = Vec::new();

    test_list.push(1);
    test_list.push(2);
    test_list.push(2);
    test_list.push(3);
    test_list.push(3);
    test_list.push(7);
    test_list.push(3);
    test_list.push(3);
    println!("{:?}", test_list);

    remove_specific(&mut test_list, 7);
    println!("{:?}", test_list);
    remove_duplicates_fast(&mut test_list);

    println!("{:?}", test_list);
}

fn remove_duplicates(a_list: &mut Vec<i32>) {
    println!("func");

    let mut index = 0;

    while index < a_list.len() - 1 {
        if a_list[index] == a_list[index+1] {
            println!("{} appears twice, so we'll remove it", a_list[index]);
            a_list.remove(index);
            index -= 1;
        }

        index += 1;
    }
}

fn remove_duplicates_fast(a_list: &mut Vec<i32>){
    let mut map: HashSet<i32> = HashSet::new();
    let mut ret_list: Vec<i32> = Vec::new();

    for num in a_list.iter() {

        if !map.contains(num) {
            ret_list.push(*num);
            map.insert(*num);
        }

        println!("{}", *num);
    }

    a_list.clear();

    for num in ret_list.iter() {
        a_list.push(*num);
    }

    // return ret_list;
}

fn remove_specific(a_list: &mut Vec<i32>, k: i32) {

    let mut tmp_list: Vec<i32> = Vec::new();

    for num in a_list.iter() {
        if !(*num == k) {
            tmp_list.push(*num);
        }
    }

    a_list.clear();

    for num in tmp_list.iter() {
        a_list.push(*num);
    }
}
