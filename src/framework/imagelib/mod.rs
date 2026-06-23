// ImageLib — 图像加载库

pub struct ImageLib;

impl ImageLib {
    pub fn load_image(_path: &str) -> Option<*mut crate::framework::graphics::image::Image> {
        None
    }

    pub fn load_surface(_path: &str) -> Option<*mut crate::ffi::sdl2::SDL_Surface> {
        None
    }
}
