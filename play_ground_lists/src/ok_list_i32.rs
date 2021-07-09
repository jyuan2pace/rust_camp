#![allow(dead_code)]
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn search(&self, key: i32) -> Option<i32> {
    //TODO
        unimplemented!();
    }

    pub fn append(&mut self, elem: i32) { 
    //TODO
        unimplemented!();
    }

    pub fn get_at(&self, idx: i32) -> Option<&i32> {
    //TODO
        unimplemented!();
    }

    pub fn insert_at(&mut self, idx: i32, elem: i32)-> Result<String, String> {
    //TODO
        unimplemented!();
    }

    pub fn delete_at(&mut self, idx: i32) -> Option<i32> {
    //TODO
        unimplemented!();
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&i32> {
        let r = self.head.as_ref();
        r.map(|node| { &node.elem})
    }

    pub fn peek_mut(&mut self) -> Option<&mut i32> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter(List);

impl Iterator for IntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a> {
    next: Option<&'a mut Node>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn append() {
        let mut list = List::new();

        list.append(3); 
        list.append(2); 
        list.append(1);
        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    
    }
    
    #[test]
    fn search() {
        let mut list = List::new();

        list.append(13); 
        list.append(12); 
        list.append(11);
        
        // Check normal get
        assert_eq!(list.search(11), Some(2));
        assert_eq!(list.search(12), Some(1));
        assert_eq!(list.search(13), Some(0));
    
        //check negative case

        assert_eq!(list.search(5), None);
    }

    #[test]
    fn get_at() {
        let mut list = List::new();

        // Check negative case
        assert_eq!(list.get_at(0), None);
        list.append(13); 
        list.append(12); 
        list.append(11);
        
        // Check normal get
        assert_eq!(list.get_at(0).map(|x|*x), Some(13));
        assert_eq!(list.get_at(1).map(|x|*x), Some(12));
        assert_eq!(list.get_at(2).map(|x|*x), Some(11));
    
        //check negative case

        assert_eq!(list.get_at(-3), None);
        assert_eq!(list.get_at(3), None);
    }

 
    #[test]
    fn insert_at() {
         let mut list = List::new();
        // Check negative case
        assert!(matches!(list.insert_at(0,13), Ok(_))); 
        assert!(matches!(list.insert_at(1,11), Ok(_))); 
        assert!(matches!(list.insert_at(2,9), Ok(_)));
        
        // Check normal get
        assert_eq!(list.get_at(0).map(|x|*x), Some(13));
        assert_eq!(list.get_at(1).map(|x|*x), Some(11));
        assert_eq!(list.get_at(2).map(|x|*x), Some(9));
    
        //check negative case

        assert!(matches!(list.insert_at(1,12), Ok(_)));
        assert!(matches!(list.insert_at(3,10), Ok(_)));
        assert_eq!(list.get_at(0).map(|x|*x), Some(13));
        assert_eq!(list.get_at(1).map(|x|*x), Some(12));
        assert_eq!(list.get_at(2).map(|x|*x), Some(11));
        assert_eq!(list.get_at(3).map(|x|*x), Some(10));
        assert_eq!(list.get_at(4).map(|x|*x), Some(9));

        assert!(matches!(list.insert_at(8,9), Err(_)));
    
    }

    #[test]
    fn delete_at() {
         let mut list = List::new();
        // Check negative case
        let mut i = 0;
        loop {
            if i == 10 {break;}
            assert!(matches!(list.insert_at(i, i), Ok(_)));
            i = i+1; 
        }

        let mut j = 8;
        loop {
            if j < 0 {break;}
            assert_eq!(list.delete_at(j), Some(j));
            j = j-2;
        }


        // Check normal get
        assert_eq!(list.get_at(0).map(|x|*x), Some(1));
        assert_eq!(list.get_at(1).map(|x|*x), Some(3));
        assert_eq!(list.get_at(2).map(|x|*x), Some(5));
        assert_eq!(list.get_at(3).map(|x|*x), Some(7));
        assert_eq!(list.get_at(4).map(|x|*x), Some(9));
        
        assert_eq!(list.delete_at(4), Some(9));
    
        //check negative case
        assert_eq!(list.delete_at(4), None);
        assert_eq!(list.delete_at(-3), None);
    
    }
 

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

}


