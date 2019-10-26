#![no_std]

//!
//! Get const value from enum handle
//! 
//! # Examples
//!
//! ```
//! use const_enum_map::{ConstKey,const_enum_map};
//!
//! fn item0() -> &'static str {
//!     "item0"
//! }

//! fn item1() -> &'static str {
//!     "item1"
//! }

//! const_enum_map! {
//!     Foo => fn()->&'static str,
//!     Item0 => item0,
//!     Item1 => item1
//! }
//!
//! fn main() {
//!     assert_eq!(Foo::Item0.get()(), "item0");
//!     assert_eq!(Foo::Item1.get(), Foo::Item1.get());
//!     assert_eq!(Foo::value_list(), Foo::value_list());
//! }
//! ```


pub trait ConstKey<T>: Copy + Clone {
    fn value_list() -> &'static [T];
    fn get_id(self) -> usize;
    fn get(self) -> &'static T {
        Self::value_list().get(self.get_id()).unwrap()
    }
}

#[macro_export]
macro_rules! const_enum_map {
    ( $key_type:ident => $item_type:ty , $( $enum_value:ident => $item_value:expr ,)+ ) => {
        #[derive(Copy,Clone)]
        enum $key_type {
            $(
                $enum_value ,
            )+
        }
        impl ConstKey<$item_type> for $key_type {
            fn value_list()->&'static [$item_type] {
                const VALUES:&'static [$item_type] = &[
                    $( $item_value ),+
                ];
                VALUES
            }
            fn get_id(self) -> usize {
                self as usize
            }
        }
    };
    ( $key_type:ident => $item_type:ty , $( $enum_value:ident => $item_value:expr ),+ ) => {
        const_enum_map! {
            $key_type => $item_type ,
            $( $enum_value => $item_value ,)+
        }
    }
}