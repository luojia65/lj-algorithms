#[macro_use]
mod testing;

pub mod singly;
mod singly_db; 
// pub mod circular; // bugs

pub use self::singly::SinglyLinkedList;
// pub use self::circular::CircularLinkedList;

test_one!(singly_list_tests, SinglyLinkedList);
// test_one!(circular_list_tests, CircularLinkedList);
