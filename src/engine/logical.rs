
pub trait Logical {

}

pub struct Or<T, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

pub struct XOr<T, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

pub struct And<T, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}

pub struct NAnd<T, const LENGTH: usize> {
    items: [Option<T>; LENGTH],
}
