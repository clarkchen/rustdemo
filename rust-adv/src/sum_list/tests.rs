use crate::sum_list;

#[test]
fn test_sum_list(){
    let acceptable_list = [1,2,3,4];
    let exceptional_list = [1, u32::MAX];
    assert_eq!(sum_list::sum_list(&acceptable_list), Some(10));
    assert_eq!(sum_list::sum_list(&exceptional_list), None);
}