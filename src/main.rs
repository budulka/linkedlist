use std::ops::{Index, IndexMut};
struct List<T> {
    len : usize,
    head : Option<Box<Node<T>>>
}


struct Node<T> {
    value : T,
    next : Option<Box<Node<T>>>
}

impl<T> List<T> {
    fn new() -> Self{
        Self {
            len : 1,
            head : None
        }
    }
    fn append(&mut self, val : T) {
        match &mut self.head {
            None => self.head = Some(Box::new(Node::new(val))),
            Some(node) => {List::append_rec(val, node); self.len += 1;}
        }
    }

   
    fn append_rec(val : T, current : &mut Box<Node<T>>) {
        match &mut current.next {
            None => current.next = Some(Box::new(Node::new(val))),
            Some (node) => List::append_rec(val, node)
        }
    }

    fn from_vector(vector : Vec<T>) -> Self {
        let mut list_from_vector = List::new();
        for item in vector {
            list_from_vector.append(item);
        }
        list_from_vector
    }

}

impl<T> Iterator for List<T> where T : Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head.as_mut() {
            None => None,
            Some (node) => {
                let value = node.value;
                self.head = node.next.take();
                Some(value)
            }
        }
    }
}
 
impl<T> Index<usize> for List<T> {
    type Output = T; 
    fn index(&self, index: usize) -> &T {
        if index > self.len {
            panic!("Out of range");
        }
        let mut count : usize = 0;
        let mut cur = &self.head;
        loop {
            match cur {
                None => panic!(),
                Some(node) if index == count => return &node.value,
                Some (node) => {cur = &node.next; count += 1}

            }
        }
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > self.len {
            panic!("Out of range");
        }
        let mut count : usize = 0;
        let mut cur = &mut self.head;
        loop {
            match cur.as_mut() {
                None => panic!(),
                Some(node) if index == count => return &mut node.value,
                Some (node) => {cur = &mut node.next; count += 1}

            }
        }
    }
}

impl<T> Node<T> {
    fn new(val : T) -> Self {
        Self {
            value : val,
            next : None
        }
    }
    
}



fn main() {
    let mut list = List::<u16>::new(); 
    list.append(4);
    list.append(0);
    list.append(3);
    list.append(16);
    println!("First list of length {}:", list.len);
    for item in list.into_iter() {
        print!("{item} ");
    }
    let v = vec!(1, 2, 3, 5,4, 5,);
    let mut list = List::from_vector(v);
    println!("\nSecond list has the length of {}", list.len);
    println!("\nSecond list");
    println!("The 2nd pos is {}", list[2]);
    list[2] = 6;
    for item in list.into_iter() {
        print!("{item} ");
    }
    
}
