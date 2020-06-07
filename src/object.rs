use material::MaterialBox;
use surface::Surface;

pub struct Object {
    pub surface: Box<dyn Surface>,
    pub material: MaterialBox,
}
