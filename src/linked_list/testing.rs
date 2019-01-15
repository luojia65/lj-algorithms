#[macro_export]
macro_rules! test_one {
    ($module_name: ident, $ll_name: ident) => {

#[cfg(test)]
mod $module_name {
    use super::*;
    use core::iter::FromIterator;
    #[test]
    fn new_list() {
        let mut list: $ll_name<usize> = $ll_name::new();
        assert!(list.is_empty());
        assert!(!list.contains(&233usize));
        assert_eq!(list.len(), 0);
        assert_eq!(format!("{:?}", list), "[]");
        assert_eq!(list, list);
        assert_eq!(list.front(), None);
        assert_eq!(list.front_mut(), None);
        assert_eq!(list.back(), None);
        assert_eq!(list.back_mut(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        let second = list.split_off(0);
        assert_eq!(list, $ll_name::new());
        assert_eq!(second, $ll_name::new());
    }

    #[test]
    fn small_item() {
        let mut list = $ll_name::from_iter(vec![1, 2, 3]);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert_eq!(list, $ll_name::from_iter(vec![1, 2, 3]));
        assert_ne!(list, $ll_name::from_iter(vec![10, 2, 3]));
        list.push_back(4);
        assert_eq!(format!("{:?}", list), "[1, 2, 3, 4]");
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(format!("{:?}", list), "[2, 3, 4]");
        list.push_front(5);
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3)); 
        assert_eq!(list.pop_front(), None); 
        assert_eq!(list.pop_back(), None); 
        let mut list = $ll_name::from_iter(vec![6, 7, 8, 9, 10]);
        let second = list.split_off(3);
        assert_eq!(format!("{:?} {:?}", list, second), "[6, 7, 8] [9, 10]");
        assert!(((list < second)&&(second > list))||
                ((list > second)&&(second < list)));
    }

    #[test] 
    #[should_panic(expected = "Cannot split off a nonexistent index")]
    fn invalid_split_off() {
        let mut list = $ll_name::from_iter(&[10, 11, 12]);
        assert_eq!(list.split_off(0).len(), 3); // okay! []  [10, 11, 12]
        let mut list = $ll_name::from_iter(&[10, 11, 12]);
        assert_eq!(list.split_off(1).len(), 2); // okay! [10]    [11, 12]
        let mut list = $ll_name::from_iter(&[10, 11, 12]);
        assert_eq!(list.split_off(2).len(), 1); // okay! [10, 11]    [12]
        let mut list = $ll_name::from_iter(&[10, 11, 12]);
        assert_eq!(list.split_off(3).len(), 0); // okay! [10, 11, 12]  []
        let mut list = $ll_name::from_iter(&[10, 11, 12]);
        list.split_off(4); // panic!
    }

    #[test]
    fn large_item() {
        let mut vec = Vec::new();
        let qty = 10000;
        for i in 0..=qty {
            vec.push(i * 5 + 4);
        }
        let mut list = $ll_name::new();
        list.extend(vec);
        assert!(!list.is_empty());
        assert_eq!(list.len(), qty+1);
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_back(), Some(5*qty+4));
        assert!(list.contains(&2504));
        assert!(!list.contains(&2503));
        for i in 1..=qty-1 {
            assert_eq!(list.pop_front(), Some(i * 5 + 4));
        }
    }
}
    };
}
