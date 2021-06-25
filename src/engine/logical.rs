#[derive(Copy, Clone)]
pub struct Or<T: Copy, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

impl<T: Copy, const LENGTH: usize> Or<T, LENGTH> {
    pub fn empty() -> Or<T, LENGTH> {
        return Or::<T, LENGTH> { items: [None; LENGTH] };
    }
}

#[derive(Copy, Clone)]
pub struct XOr<T: Copy, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

impl<T: Copy, const LENGTH: usize> XOr<T, LENGTH> {
    pub fn empty() -> XOr<T, LENGTH> {
        return XOr::<T, LENGTH> { items: [None; LENGTH] };
    }
}

#[derive(Copy, Clone)]
pub struct And<T: Copy, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

impl<T: Copy, const LENGTH: usize> And<T, LENGTH> {
    pub fn empty() -> And<T, LENGTH> {
        return And::<T, LENGTH> { items: [None; LENGTH] };
    }
}

#[derive(Copy, Clone)]
pub struct NAnd<T: Copy, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

impl<T: Copy, const LENGTH: usize> NAnd<T, LENGTH> {
    pub fn empty() -> NAnd<T, LENGTH> {
        return NAnd::<T, LENGTH> { items: [None; LENGTH] };
    }
}
