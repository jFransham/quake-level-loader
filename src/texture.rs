// TODO: Make TextureBuilder completely ignorant of the Texture struct,
//       load Texture2d's instead.

use glium::backend::Facade;
use glium::texture::{Texture2d, RawImage2d};
use std::sync::{Arc, Weak};
use std::collections::HashMap;
use std::path::PathBuf;
use texture_flags::*;
use std::fs::PathExt;
use image;

#[derive(Debug, Clone)]
pub struct Texture {
    texture: Arc<Texture2d>,
    surface_flags: SurfaceFlags,
}

pub trait CreateTexture {
    fn create_texture(self, flags: SurfaceFlags) -> Texture;
}

impl CreateTexture for Arc<Texture2d> {
    fn create_texture(self, flags: SurfaceFlags) -> Texture {
        Texture {
            texture: self,
            surface_flags: flags,
        }
    }
}

impl CreateTexture for Texture2d {
    fn create_texture(self, flags: SurfaceFlags) -> Texture {
        Arc::new(self).create_texture(flags)
    }
}

// TODO: make this take a list of root directories
//       -- also, allow sub-builders so that builders
//          for different maps can share cache information
//       -- use enum with root (Vec<pathbuf>, facade, cache) or
//          inherit(texturebuilder, pathbuf)
pub struct TextureBuilder<'a, T: Facade + 'a> {
    roots: Arc<Vec<PathBuf>>,
    missing: Option<String>,
    facade: &'a T,
    cache: HashMap<String, Weak<Texture2d>>,
}

impl<'a, T: Facade + 'a> TextureBuilder<'a, T> {
    pub fn new<A: Into<PathBuf>, I: IntoIterator<Item=A>>(
        a: I, facade: &'a T, ms: Option<String>
    ) -> TextureBuilder<'a, T> {
        TextureBuilder {
            roots: Arc::new(
                a.into_iter().map(|e| e.into()).collect::<Vec<_>>()
            ),
            missing: ms.clone(),
            facade: facade,
            cache: HashMap::new()
        }
    }

    fn get_real_path_and_ext(
        roots: &[PathBuf],
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

        for root in roots {
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

    fn find_in_cache(&self, hash: String) -> Option<Arc<Texture2d>> {
        self.cache.get(&hash).and_then(|weak| weak.upgrade())
    }

    pub fn load(
        &mut self, path: &str
    ) -> Option<Arc<Texture2d>> {
        self.find_in_cache(path.into()).or_else(||
            Self::load_inner(self.roots.clone(), path).and_then(|image|
                Texture2d::new(self.facade, image).ok()
                    .map(|t| {
                        let out = Arc::new(t);
                        self.cache.insert(path.into(), Arc::downgrade(&out));
                        out
                    })
            )
        )
    }

    pub fn load_async(
        &mut self, many: Vec<String>
    ) -> Vec<Option<Arc<Texture2d>>> {
        use eventual::*;
        use itertools::*;

        let cached = many.iter()
            .enumerate()
            .map(|(i, path)|
                (i, path.clone(), self.find_in_cache(path.clone()))
            )
            .collect::<Vec<_>>();
        let promises =
            cached.iter()
            .cloned()
            .filter_map(|(n, path, opt)|
                if opt.is_none() {
                    let rclone = self.roots.clone();
                    let msclone = self.missing.clone();
                    Some(Future::spawn(move || {
                        let load = Self::load_inner(rclone.clone(), &path)
                            .or_else(||
                                msclone.and_then(|inner|
                                    Self::load_inner(rclone, &inner)
                                )
                            );
                        (
                            n,
                            path,
                            load,
                        )
                    }))
                } else {
                    None
                }
            )
            .collect::<Vec<_>>();
        let textures = join(
            promises
        ).await()
        .unwrap()
        .into_iter()
        .map(|(n, path, raw)|
            (
                n,
                raw.and_then(|image|
                    Texture2d::new(self.facade, image).ok()
                        .map(|t| {
                            let out = Arc::new(t);
                            self.cache.insert(path.into(), Arc::downgrade(&out));
                            out
                        })
                ),
            )
        )
        .collect::<Vec<_>>();

        cached.into_iter()
            .filter_map(|(n, _, maybe_tex)|
                maybe_tex.map(|t| (n, Some(t)))
            )
            .chain(textures.into_iter())
            .sorted_by(|a, b| a.0.cmp(&b.0))
            .into_iter()
            .zip(0..)
            .map(|((n, t), i)| {
                 debug_assert!(n==i);
                 t
            })
            .collect::<Vec<_>>()
    }

    fn load_inner<'b: 'a>(
        roots: Arc<Vec<PathBuf>>, path: &str
    ) -> Option<RawImage2d<'b, u8>> {
        use std::io::BufReader;
        use std::fs::File;

        let (ext, real_path) =
            if let Some(tup) = Self::get_real_path_and_ext(&*roots, path) {
                tup
            } else {
                return None
            };

        let f = if let Ok(a) = File::open(&real_path) {
                a
            } else {
                return None
            };
        let reader = BufReader::new(f);

        let raw = if let Ok(a) = image::load(
                reader,
                ext
            ) {
                a.to_rgba()
            } else {
                return None
            };
        let image_dimensions = raw.dimensions();
        Some(
            RawImage2d::from_raw_rgba_reversed(
                raw.into_raw(), image_dimensions
            )
        )
    }
}
