use surface::Surface;
use material::MaterialBox;

pub struct Object<'a> {
    pub surface: &'a Surface,
    pub material: MaterialBox
}
