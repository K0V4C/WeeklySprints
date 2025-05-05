use std::{alloc::Layout, ptr::NonNull};
struct SimpleVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize
}

unsafe impl<T: Send> Send for SimpleVec<T>{}
unsafe impl<T: Sync> Sync for SimpleVec<T>{}

impl<T> SimpleVec<T> {
    
    /*
    
        PUBLIC API
    
    */
    
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "We are not ready to handle ZSTs");
       
       SimpleVec { ptr: NonNull::dangling(), cap: 0, len: 0 } 
    }
    
    pub fn push(&mut self, item: T) {
        if self.len() == self.cap {
            self.grow();
        }
        
        unsafe {
            std::ptr::write(self.ptr.as_ptr().add(self.len), item);
        }
        self.len += 1;
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            // Notice how we use new_len to index the vector 
            self.len -= 1;
            unsafe {
                Some(
                    std::ptr::read(self.ptr.as_ptr().add(self.len))
                )
            }
        }
        
    }
    
    pub fn len(&self) -> usize {
       self.len 
    }
    
    /*
   
        GUTS AND INTERNALS
   
    */
    
    fn grow(&mut self) {
        
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            
            // This can't overflow since self.cap <= isize::MAX
            let new_cap = self.cap * 2;
            
            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };
        
        // Ensure new allocaiton is not bigger then isize::MAX
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");
        
        let new_ptr = if self.cap == 0 {
            unsafe {
                std::alloc::alloc(new_layout)
            }
        }   else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {
                std::alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };
        
        // If allocation fails new_ptr will be null in that case we abort
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => std::alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
    
}


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn check_vec_creation() {
        let new_vec: SimpleVec<i32> = SimpleVec::new();
        assert!(new_vec.len() == 0, "Creating new vector failed, size is not 0")
    }
    
    
    
}
