
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Or<T, const LENGTH: usize>
where [Option<T>; LENGTH]: Serialize + DeserializeOwned {
    pub items: [Option<T>; LENGTH],
}

impl<T: Copy + Clone, const LENGTH: usize> Or<T, LENGTH>
where [Option<T>; LENGTH]: Serialize + DeserializeOwned {
    pub fn new(items: &[T]) -> Or<T, LENGTH> {
        if items.len() > LENGTH {
            panic!("input is too long")
        }
        else {
            let mut r = Or::<T, LENGTH>::empty();
            for i in 0 .. items.len() {
                r.items[i] = Some(items[i]);
            }
            return r;
        }
    }

    pub fn empty() -> Or<T, LENGTH> {
        return Or::<T, LENGTH> { items: [None; LENGTH] };
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct XOr<T, const LENGTH: usize>
where [Option<T>; LENGTH]: Serialize + DeserializeOwned {
    pub items: [Option<T>; LENGTH],
}

impl<T: Copy + Clone, const LENGTH: usize> XOr<T, LENGTH>
where [Option<T>; LENGTH]: Serialize + DeserializeOwned {
    pub fn new(items: &[T]) -> XOr<T, LENGTH> {
        if items.len() > LENGTH {
            panic!("input is too long")
        }
        else {
            let mut r = XOr::<T, LENGTH>::empty();
            for i in 0 .. items.len() {
                r.items[i] = Some(items[i]);
            }
            return r;
        }
    }
    pub fn empty() -> XOr<T, LENGTH> {
        return XOr::<T, LENGTH> { items: [None; LENGTH] };
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct And<T, const LENGTH: usize>
where [Option<T>; LENGTH]: Serialize + DeserializeOwned {
    pub items: [Option<T>; LENGTH],
}

impl<T: Copy + Clone, const LENGTH: usize> And<T, LENGTH>
where [Option<T>; LENGTH]: Serialize + DeserializeOwned {
    pub fn new(items: &[T]) -> And<T, LENGTH> {
        if items.len() > LENGTH {
            panic!("input is too long")
        }
        else {
            let mut r = And::<T, LENGTH>::empty();
            for i in 0 .. items.len() {
                r.items[i] = Some(items[i]);
            }
            return r;
        }
    }

    pub fn empty() -> And<T, LENGTH> {
        return And::<T, LENGTH> { items: [None; LENGTH] };
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct NAnd<T, const LENGTH: usize>
where [Option<T>; LENGTH]: Serialize + DeserializeOwned {
    pub items: [Option<T>; LENGTH],
}

impl<T: Copy + Clone, const LENGTH: usize> NAnd<T, LENGTH>
where [Option<T>; LENGTH]: Serialize + DeserializeOwned {
    pub fn new(items: &[T]) -> NAnd<T, LENGTH> {
        if items.len() > LENGTH {
            panic!("input is too long")
        }
        else {
            let mut r = NAnd::<T, LENGTH>::empty();
            for i in 0 .. items.len() {
                r.items[i] = Some(items[i]);
            }
            return r;
        }
    }

    pub fn empty() -> NAnd<T, LENGTH> {
        return NAnd::<T, LENGTH> { items: [None; LENGTH] };
    }
}
