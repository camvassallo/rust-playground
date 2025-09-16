use std::collections::LinkedList;

fn main() {}

#[test]
fn reverse_linked_list() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(3);
    list.push_back(2);
    list.push_back(1);
    println!("{:?}", list);

    reverse(&mut list);

    println!("{:?}", list);
}

fn reverse(list: &mut LinkedList<i32>){

    let mut temp: Vec<i32> = Vec::new();

    let list_len: i32 = list.len() as i32;
    for i in 0..list_len {
        temp.push(list.pop_back().unwrap());
    }

    list.clear();

    for num in temp {
        list.push_back(num);
    }
}

#[test]
fn merge_sorted_list_test() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    let mut list2: LinkedList<i32> = LinkedList::new();
    list2.push_back(1);
    list2.push_back(2);
    list2.push_back(3);
    list2.push_back(4);

    let list_merged: LinkedList<i32> = merge_sorted_linked_lists(&mut list,  &mut list2);

    println!("{:?}", list_merged);
}

fn merge_sorted_linked_lists(list1: &mut LinkedList<i32>, list2: &mut LinkedList<i32>) -> LinkedList<i32>{
    let mut result = LinkedList::new();

    while !list1.is_empty() || !list2.is_empty() {

        if list1.is_empty() {
            result.push_back(list2.pop_front().unwrap());
        }else if list2.is_empty() {
            result.push_back(list1.pop_front().unwrap());
        } else if list1.front() < list2.front() {
            result.push_back(list1.pop_front().unwrap());
        } else {
            result.push_back(list2.pop_front().unwrap());
        }
    }

    result
}
