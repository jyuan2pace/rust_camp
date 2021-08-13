#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

type Link = Option <Box<Node>>;

impl BST {
    pub fn new() -> Self {
        //TODO
    }
    
    /// Insert an element into the BST. Return true
    /// if successful, or false if the element was already in the BST.
    pub fn insert(&mut self, elem: i32) -> bool {
        //TODO
    }

    /// Search for an element in the BST. Return true
    /// if the element was found.
    pub fn search(&self, elem: i32) -> bool {
        //TODO
    }
    
    /// Do I need to write comments on what height means? -Jun
    pub fn height (&self) -> i32 {
        //TODO
    }

    /// transform a binary tree into its mirror tree.
    /// for instance
    ///        4              4
    ///      /  \           /  \ 
    ///     2    7   =>    7    2
    ///         /           \
    ///        5             5
    ///
    /// Note: clone/creating new node is not allowed. 
    /// The point of the exercise is exactly pointer swing
    /// No return type as the mirroring happens in place
    pub fn mirrorize(&mut self) -> () {
        //TODO
    }

    fn beauty_print_helper(cur_link: &Link, space: usize) -> String {
        match cur_link {
            None => "".to_string(),
            Some(boxed_node) => {
                let rs = BST::beauty_print_helper(&boxed_node.right, space+10);
                let mid = " ".repeat(space);
                let data = boxed_node.elem.to_string();
                let ls = BST::beauty_print_helper(&boxed_node.left, space+10);
                format!("{}{}{}{}{}{}", rs, "\n", mid, data, "\n", ls)
            }
        }
      }

    pub fn beauty_string(& self) -> String {
        //oh well, I am doing toString but this is a pretty one. XD
        //You can use this to debug if you are yet used to gdb -Jun
        BST::beauty_print_helper(& self.root, 0)
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
        let mut tree = BST::new();

        // Check empty list behaves right
        assert_eq!(tree.search(1), false);

        // Populate list
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        // Check search
        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(2), true);
        assert_eq!(tree.search(3), true);
        assert_eq!(tree.search(4), false);

        // Check repeatly insertion
        assert_eq!(tree.insert(1), false);
    }

    #[test]
    fn height() {
        let mut tree = BST::new();

        assert_eq!(tree.height(), -1);

        // Populate list
        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.height(), 2);
        
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn mirror1() {
        let mut tree = BST::new();
        assert_eq!(tree.height(), -1);

        // Populate list
        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(1), true); 
        //println!("{}",tree.beauty_string());
        tree.mirrorize();
        let expected_str = "
                    1

          2

4
";
            
        assert_eq!(tree.beauty_string(), expected_str);
        //println!("{}",tree.beauty_string());
   }

    #[test]
    fn mirror2() {
        let mut tree = BST::new();

        assert_eq!(tree.height(), -1);

        // Populate list
        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.insert(7), true);
//        println!("{}",tree.beauty_string());
        tree.mirrorize();
        let str = "
                    1

          2

                    3

4

                    5

          6

                    7
";
        assert_eq!(tree.beauty_string(), str);
//        println!("{}",tree.beauty_string());
    }
}
