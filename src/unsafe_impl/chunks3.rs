use std::marker;


pub fn chunks3<'a, C>(memory: &'a [C]) -> Chunks3<'a, C>
    where
        C: Copy + 'a
{
    assert_eq!(memory.len() % 3, 0);

    unsafe {
        Chunks3 {
            ptr: memory.as_ptr(),
            end: memory.as_ptr().offset(memory.len() as isize),
            _marker: marker::PhantomData,
        }
    }
}

pub struct Chunks3<'a, C>
    where
        C: Copy + 'a
{
    ptr: *const C,
    end: *const C,
    _marker: marker::PhantomData<&'a C>,
}

impl<'a, C> Iterator for Chunks3<'a, C>
    where
        C: Copy + 'a
{
    type Item = (C, C, C);
    
    fn next(&mut self) -> Option<(C, C, C)> {
        if self.ptr == self.end {
            return None;
        }
        let data;
        unsafe {
            data = (*self.ptr.offset(0), *self.ptr.offset(1), *self.ptr.offset(2));
            self.ptr = self.ptr.offset(3);
        }
        Some(data)
    }
}


pub fn chunks3_mut<'a, C>(memory: &'a mut [C]) -> Chunks3Mut<'a, C>
    where
        C: Copy + 'a
{
    assert_eq!(memory.len() % 3, 0);

    unsafe {
        Chunks3Mut {
            ptr: memory.as_mut_ptr(),
            end: memory.as_mut_ptr().offset(memory.len() as isize),
            _marker: marker::PhantomData,
        }
    }
}

pub struct Chunks3Mut<'a, C>
    where
        C: Copy + 'a
{
    ptr: *mut C,
    end: *mut C,
    _marker: marker::PhantomData<&'a C>,
}

impl<'a, C> Iterator for Chunks3Mut<'a, C>
    where
        C: Copy + 'a
{
    type Item = (&'a mut C, &'a mut C, &'a mut C);
    
    fn next(&mut self) -> Option<(&'a mut C, &'a mut C, &'a mut C)> {
        if self.ptr == self.end {
            return None;
        }

        let data;
        unsafe {
            data = (
                &mut *self.ptr.offset(0),
                &mut *self.ptr.offset(1),
                &mut *self.ptr.offset(2));
            self.ptr = self.ptr.offset(3);
        }
        Some(data)
    }
}
