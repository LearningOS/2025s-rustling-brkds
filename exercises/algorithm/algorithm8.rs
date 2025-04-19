/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/
// I AM NOT DONE

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
    size: u8,
    tag: bool,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            size: 0,
            tag:true,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        if self.tag{
            self.q1.enqueue(elem);
        }
        else {self.q2.enqueue(elem);}
        self.size+=1;
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.size !=0 {
         if self.tag{
            for i in 1..self.q1.size(){
               if let Ok(n) = self.q1.dequeue()
                 {self.q2.enqueue(n);}
            }
                self.size-=1;
                self.tag = !self.tag;
                self.q1.dequeue()
            
         }
         else {
            for i in 1..self.q2.size(){
               if let Ok(n) = self.q2.dequeue()
                {self.q1.enqueue(n);}
            }
                self.size-=1;
                self.tag = !self.tag;
                self.q2.dequeue()
            
         }
        }
        else{
		Err("Stack is empty")}
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        if self.size==0{
        true
        }
        else{false}
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}