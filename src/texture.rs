use glium::texture::Texture2d;
use glium::backend::Facade;
use std::rc::{Rc, Weak};
use std::path::PathBuf;
use texture_flags::*;
use std::fs::PathExt;
use image;

fn get_string_hash(s: &str) -> u64 {
    use std::hash::{SipHasher, Hash, Hasher};

    let mut hasher = SipHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug)]
pub struct Texture {
    hash: u64,
    texture: Texture2d,
    surface_flags: SurfaceFlags,
}

// TODO: make this take a list of root directories
//       -- also, allow sub-builders so that builders
//          for different maps can share cache information
//       -- use enum with root (Vec<pathbuf>, facade, cache) or
//          inherit(texturebuilder, pathbuf)
pub struct TextureBuilder<'a, T: Facade + 'a> {
    roots: Vec<PathBuf>,
    facade: &'a T,
    cache: Vec<Weak<Texture>>,
}

impl<'a, T: Facade + 'a> TextureBuilder<'a, T> {
    pub fn new<A: Into<PathBuf>, I: IntoIterator<Item=A>>(
        a: I, facade: &'a T
    ) -> TextureBuilder<'a, T> {
        TextureBuilder {
            roots: a.into_iter().map(|e| e.into()).collect::<Vec<_>>(),
            facade: facade,
            cache: vec![]
        }
    }

    fn get_real_path_and_ext(
        &self,
        path: &str
    ) -> Option<(image::ImageFormat, PathBuf)> {
        use image::ImageFormat;
        use image::ImageFormat::*;

        fn get_extensions(i: &ImageFormat) -> &'static [&'static str] {
            static PNG_EXT:  [&'static str; 1] = ["png"];
            static JPEG_EXT: [&'static str; 2] = ["jpeg", "jpg"];
            static GIF_EXT:  [&'static str; 1] = ["gif"];
            static WEBP_EXT: [&'static str; 1] = ["webp"];
            static PPM_EXT:  [&'static str; 1] = ["ppm"];
            static TIFF_EXT: [&'static str; 1] = ["tiff"];
            static TGA_EXT:  [&'static str; 1] = ["tga"];
            static BMP_EXT:  [&'static str; 1] = ["bmp"];
            static ICO_EXT:  [&'static str; 1] = ["ico"];

            match *i {
                PNG  => &PNG_EXT,
                JPEG => &JPEG_EXT,
                GIF  => &GIF_EXT,
                WEBP => &WEBP_EXT,
                PPM  => &PPM_EXT,
                TIFF => &TIFF_EXT,
                TGA  => &TGA_EXT,
                BMP  => &BMP_EXT,
                ICO  => &ICO_EXT,
            }
        }

        for root in &self.roots {
            let root: PathBuf = root.join(path);
            let file_name: String =
                if let Some(Some(f)) = root.file_name().map(|o| o.to_str()) {
                    f.into()
                } else {
                    return None
                };
            for ex in [PNG, JPEG, GIF, WEBP, PPM, TIFF, TGA, BMP, ICO].into_iter() {
                let extensions = get_extensions(&ex);

                for str_ex in extensions {
                    let out = root.with_file_name(format!("{}.{}", file_name, str_ex));

                    if out.is_file() { return Some((*ex, out.to_path_buf())); }
                }
            }
        }

        None
    }

    pub fn load(
        &mut self, path: &str, surface_flags: SurfaceFlags
    ) -> Option<Rc<Texture>> {
        use std::io::BufReader;
        use std::fs::File;
        use glium::texture::RawImage2d;
        use image;

        let str_hash = get_string_hash(path);
        if let Some(t) = self.cache.iter()
            .filter_map(|weak| weak.upgrade())
            .find(
                |t| t.hash == str_hash
            )
        {
            return Some(t);
        }

        let (ext, real_path) =
            if let Some(tup) = self.get_real_path_and_ext(path) {
                tup
            } else {
                println!("{} not found", path);
                return None
            };

        let f = if let Ok(a) = File::open(&real_path) {
                a
            } else {
                println!("Cannot open {:?}", &real_path);
                return None
            };
        let reader = BufReader::new(f);

        let raw = if let Ok(a) = image::load(
                reader,
                ext
            ) {
                a.to_rgba()
            } else {
                println!("Cannot interpret {:?}", &real_path);
                return None
            };
        let image_dimensions = raw.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(
                raw.into_raw(), image_dimensions
            );
        Texture2d::new(self.facade, image).ok()
            .map(|t| {
                let out = Rc::new(
                    Texture {
                        hash: get_string_hash(path),
                        texture: t,
                        surface_flags: surface_flags
                    }
                );
                self.cache.push(Rc::downgrade(&out));
                out
            })
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool { self.hash == other.hash }
}

impl Eq for Texture {}

