use std::any;

pub trait TypeLayout {
    const LAYOUT: Layout;
}

#[repr(C)]
#[derive(Debug)]
pub struct Layout {
    pub name: &'static str,
    pub version: u32,
    pub fields: &'static Fields,
}

impl TypeLayout for i32 {
    const LAYOUT: Layout = Layout {
        name: any::type_name::<Self>(),
        version: 0,
        fields: &Fields::Unit,
    };
}

#[repr(C)]
#[derive(Debug)]
pub enum Fields {
    Struct(&'static [(&'static str, &'static Layout)]),
    Tuple(&'static[&'static Layout]),
    Unit,
}

pub trait SystemLayout<Args> {
    const LAYOUT: &'static [&'static Layout];
}

impl<F, T1> SystemLayout<(T1,)> for F
where
    F: FnMut(T1) -> (),
    T1: TypeLayout,
{
    const LAYOUT: &'static [&'static Layout] = &[&T1::LAYOUT];
}

impl<F, T1, T2, T3> SystemLayout<(T1, T2, T3)> for F
where
    F: FnMut(T1, T2, T3) -> (),
    T1: TypeLayout,
    T2: TypeLayout,
    T3: TypeLayout,
{
    const LAYOUT: &'static [&'static Layout] = &[&T1::LAYOUT, &T2::LAYOUT, &T3::LAYOUT];
}