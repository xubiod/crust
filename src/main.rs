#![allow(dead_code, unused_variables)]

macro_rules! rule1 {
    (mangle $x:ident into a $y:ty) => {
        $x as $y
    };
    (
        $(please)? make me a $struct_vis:vis struct called $name:ident with
        $($(and)? a field named $item_name:ident which is a $item_vis:vis $item_type:ty),*
    ) => {
        $struct_vis struct $name {
            $(
                $item_vis $item_name: $item_type,
            )*
        }
    };
    (
        $(please)? make me a $all_vis:vis enum called $name:ident with $($(and)?
        a variant called $item_name:ident $(that holds $($(and)? a $item_subtype:ty),*)?),*
    ) => {
        $all_vis enum $name {
            $(
                $item_name $(($($item_subtype,)*))? ,
            )*
        }
    };
    (
        $(please)? make me a union called $name:ident that can either be $($(or)?
        a field named $item_name:ident which is a $item_type:ty),*
    ) => {
        union $name {
            $(
                $item_name: $item_type,
            )*
        }
    };
}

rule1!{
    please make me a pub struct called StupidAss 
    with a field named x which is a pub i32, 
    and a field named y which is a i32, 
    and a field named z which is a pub(crate) i32
}

rule1!{
    make me a pub enum called StupidEnum
    with a variant called None, 
    a variant called Nada, 
    a variant called Nope,
    and a variant called SomeShit that holds a u8, a u16, and a i32
}

rule1!{
    make me a union called StupidUnion
    that can either be a field named i which is a i32,
    or a field named f which is a f32,
    or a field named u which is a u32
}

fn main() {
    let x = 0u32;
    let x = rule1!(mangle x into a i32);

    let y = StupidAss{x, y: 5, z: 10};
}
