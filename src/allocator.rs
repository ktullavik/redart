

pub struct Allocator<T> {
    items: Vec<T>,
    freelist: Vec<usize>
}


impl<T> Allocator<T> {


    pub fn new(capacity: usize) -> Allocator<T> {
        Allocator {
            items: Vec::with_capacity(capacity),
            freelist: Vec::with_capacity((capacity >> 1) + 1)
        }
    }


    pub fn put(&mut self, itm: T) -> usize {
        
        if self.freelist.is_empty() {
            self.items.push(itm);
            return self.items.len() - 1;
        }

        let index = self.freelist.pop().unwrap();
        self.items[index] = itm;
        index
    }

    
    pub fn delete(&mut self, index: usize) {
        self.items.remove(index);
        self.freelist.push(index);
    }


    pub fn get(&self, index: usize) -> &T {
        &self.items[index]
    }

}
