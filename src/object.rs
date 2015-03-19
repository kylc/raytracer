use surface::Surface;
use material::MaterialBox;

pub struct Object {
    pub surface: Box<Surface>,
    pub material: MaterialBox
}
