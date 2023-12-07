#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        println!("[{}:{}:{}]{}", $lvl, file!(), line!(), format!($($arg)*))
    };
}


pub enum ROO<T, E>{
    R(Result<T, E>),
    O(Option<T>),
}

pub trait UOR<T,E> {
    fn wrap(self) -> ROO<T,E>;
}

impl<T,E> UOR<T,E> for Result<T,E> {
    fn wrap(self) -> ROO<T,E>{
        ROO::R(self)
    }
}

impl<T> UOR<T,()> for Option<T> {
    fn wrap(self) -> ROO<T,()>{
        ROO::O(self)
    }
}


#[macro_export]
macro_rules! unwrap_or_return {
    ($lvl:expr) => {
        match crate::macros::UOR::wrap($lvl) {
            crate::macros::ROO::R(result) => {
                let Ok(thing) = result else {
                    return
                };
                thing
            },
            crate::macros::ROO::O(option) => {
                let Some(thing) = option else {
                    return
                };
                thing
            }
        }
    };
   ($a:expr, $b:expr) => {
        match crate::macros::UOR::wrap($a) {
            crate::macros::ROO::R(result) => {
                let Ok(thing) = result else {
                    return $b
                };
                thing
            },
            crate::macros::ROO::O(option) => {
                let Some(thing) = option else {
                    return $b
                };
                thing
            }
        }
    };
    ($a:expr , $($s:stmt);+ , $b:expr) => {
        match crate::macros::UOR::wrap($a) {
            crate::macros::ROO::R(result) => {
                let Ok(thing) = result else {
                    $($s)+
                    return $b
                };
                thing
            },
            crate::macros::ROO::O(option) => {
                let Some(thing) = option else {
                    $($s)+
                    return $b
                };
                thing
            }
        }
    };
}