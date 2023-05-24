#![feature(min_specialization)]
use core::ops::Deref;

pub use heapless;

#[derive(Debug)]
pub struct Layout {
    pub address: usize,
    pub size: usize,
}
pub trait GetLayout {
    fn get_layout<const N: usize>(&self, layout: &mut heapless::Vec<Layout, N>);
}

impl<T> GetLayout for T
where
    T: GetLayoutType,
{
    default fn get_layout<const N: usize>(&self, layout: &mut heapless::Vec<Layout, N>) {
        layout
            .push(Layout {
                address: self as *const _ as usize,
                size: core::mem::size_of_val(self.deref()),
            })
            .unwrap();

        T::get_layout_type(layout);
    }
}

pub trait GetLayoutType {
    fn get_layout_type<const N: usize>(layout: &mut heapless::Vec<Layout, N>);
}

impl<T> GetLayoutType for T {
    default fn get_layout_type<const N: usize>(_layout: &mut heapless::Vec<Layout, N>) {}
}

#[cfg(test)]
mod test {
    use crate::*;
    use heapless::Vec;

    // Notice here tests of Layout can only be done with respect
    // to the `size` field, as the `address` will for these
    // examples refer to data on stack (and thus change between runs)

    struct Proxy {}

    impl GetLayoutType for Proxy {
        fn get_layout_type<const N: usize>(layout: &mut Vec<Layout, N>) {
            println!("-- Proxy --");
            layout
                .push(Layout {
                    address: 1024,
                    size: 4,
                })
                .unwrap()
        }
    }

    #[test]
    fn test_u32() {
        let data: u32 = 32;
        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:?}", layout);

        assert!(layout[0].size == 4)
    }

    #[test]
    fn test_array_u32() {
        let data: [u32; 16] = [32; 16];

        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:?}", layout);

        assert!(layout[0].size == 64)
    }

    struct Simple {
        data: u32,
        data2: u64,
    }

    // this implementation should be generated by a custom derive
    impl GetLayout for Simple {
        fn get_layout<const N: usize>(&self, layout: &mut Vec<Layout, N>) {
            // get_layout is executed on each field
            self.data.get_layout(layout);
            self.data2.get_layout(layout);
        }
    }

    #[test]
    fn test_simple() {
        let data = Simple { data: 0, data2: 0 };
        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:?}", layout);

        assert!(layout[0].size == 4);
        assert!(layout[1].size == 8);
    }

    struct Complex {
        simple: Simple,
        data2: Proxy,
    }

    // this implementation should be generated by a custom derive
    impl GetLayout for Complex {
        fn get_layout<const N: usize>(&self, layout: &mut Vec<Layout, N>) {
            // get_layout is executed on each field
            self.simple.get_layout(layout);
            self.data2.get_layout(layout);
        }
    }

    #[test]
    fn test_complex() {
        let data = Complex {
            simple: Simple { data: 0, data2: 0 },
            data2: Proxy {},
        };

        let mut layout: Vec<Layout, 8> = Vec::new();
        data.get_layout(&mut layout);
        println!("{:?}", layout);

        assert!(layout[0].size == 4);
        assert!(layout[1].size == 8);
        assert!(layout[2].size == 0); // Proxy ZDT
        assert!(layout[3].size == 4); // Proxy
    }

    // enum Enum {
    //     A,
    //     B(u32),
    //     C(Custom),
    // }

    // // this implementation should be generated by a custom derive
    // impl GetLayout for Enum {
    //     fn get_layout<const N: usize>(&self, layout: &mut Vec<Layout, N>) {
    //         // get_layout is executed on each variant
    //         self.get_layout(layout);
    //     }
    // }

    // #[test]
    // fn test_enum() {
    //     let mut data = Enum::A;

    //     let mut layout: Vec<Layout, 8> = Vec::new();
    //     data.get_layout(&mut layout);
    //     println!("{:?}", layout);

    //     let mut layout: Vec<Layout, 8> = Vec::new();
    //     data = Enum::B(1);
    //     data.get_layout(&mut layout);
    //     println!("{:?}", layout);

    //     if let Enum::B(ref x) = data {
    //         println!("x {} {}", x, x as *const _ as usize);
    //     }

    //     let mut layout: Vec<Layout, 8> = Vec::new();
    //     data = Enum::C(Custom {});
    //     data.get_layout(&mut layout);
    //     println!("{:?}", layout);

    //     if let Enum::C(ref c) = data {
    //         println!("c {}", c as *const _ as usize);
    //     }
    // }
}
