
pub struct Or<T: Copy, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

impl<T, const LENGTH: usize> Or<T, LENGTH> where Option<T>: Copy, T: Copy {
    pub fn empty() -> Or<T, LENGTH> {
        return Or::<T, LENGTH> {items: [None; LENGTH]};
    }
}

impl<T, const LENGTH: usize> Copy for Or<T, LENGTH> where Option<T>: Copy, T: Copy {}
impl<T, const LENGTH: usize> Clone for Or<T, LENGTH> where Option<T>: Copy, T: Copy {
    fn clone(&self) -> Self { todo!() }
}

pub struct XOr<T: Copy, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

impl<T, const LENGTH: usize> XOr<T, LENGTH> where Option<T>: Copy, T: Copy {
    pub fn empty() -> XOr<T, LENGTH> {
        return XOr::<T, LENGTH> {items: [None; LENGTH]};
    }
}

impl<T, const LENGTH: usize> Copy for XOr<T, LENGTH> where Option<T>: Copy, T: Copy {}
impl<T, const LENGTH: usize> Clone for XOr<T, LENGTH> where Option<T>: Copy, T: Copy {
    fn clone(&self) -> Self { todo!() }
}

pub struct And<T: Copy, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

impl<T, const LENGTH: usize> And<T, LENGTH> where Option<T>: Copy, T: Copy {
    pub fn empty() -> And<T, LENGTH> {
        return And::<T, LENGTH> {items: [None; LENGTH]};
    }
}

impl<T, const LENGTH: usize> Copy for And<T, LENGTH> where Option<T>: Copy, T: Copy {}
impl<T, const LENGTH: usize> Clone for And<T, LENGTH> where Option<T>: Copy, T: Copy {
    fn clone(&self) -> Self { todo!() }
}

pub struct NAnd<T: Copy, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

impl<T, const LENGTH: usize> NAnd<T, LENGTH> where Option<T>: Copy, T: Copy {
    pub fn empty() -> NAnd<T, LENGTH> {
        return NAnd::<T, LENGTH> {items: [None; LENGTH]};
    }
}

impl<T, const LENGTH: usize> Copy for NAnd<T, LENGTH> where Option<T>: Copy, T: Copy {}
impl<T, const LENGTH: usize> Clone for NAnd<T, LENGTH> where Option<T>: Copy, T: Copy {
    fn clone(&self) -> Self { todo!() }
}
