#![feature(min_specialization)]

use heapless::Vec;
use layout_trait::*;

struct Generic<T> {
    generic: T,
}

impl<T> layout_trait::GetLayout for Generic<T> {
    fn get_layout<const N: usize>(
        &self,
        layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>,
    ) {
        self.generic.get_layout(layout);
    }
}

impl<T> layout_trait::GetLayoutType for Generic<T> {
    fn get_layout_type<const N: usize>(
        layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>,
    ) {
        T::get_layout_type(layout);
    }
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();

    let a = Generic { generic: 0u32 };

    a.get_layout(&mut layout);
    println!("{:#x?}", layout);
}
