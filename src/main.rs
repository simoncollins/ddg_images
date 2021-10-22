use argh::FromArgs;
use std::error::Error;
use std::env;
use std::path::PathBuf;
use rs_ddg_images::{download_image, download_images, get_token, find_images, Response, SearchResult};
use std::path::Path;
use std::fs::{create_dir_all, create_dir};
use convert_case::{Case, Casing};

fn default_num_images() -> u32 {
    200
}

#[derive(FromArgs, PartialEq, Debug)]
/// Search and download images.
struct ImageSearch {
    #[argh(positional)]
    keyword: Vec<String>,

    /// base folder to save images in
    #[argh(option, short='f')]
    folder: Option<String>,

    /// image types to download
    #[argh(option, short='t')]
    image_types: Vec<String>,

    /// how many images to download
    #[argh(option, short='n', default="default_num_images()")]
    num_images: u32,

    /// check image file validity
    #[argh(switch, short='v')]
    validate: bool
}

impl ImageSearch {

    fn base_path(&self) -> PathBuf {
        let mut path = env::current_dir().unwrap();
        if let Some(folder) = &self.folder {
            path = path.join(folder);
        }
        path
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let settings: ImageSearch = argh::from_env();
    let base = settings.base_path();
    if !base.exists() {
        create_dir_all(&base).expect("Could not create base folder");
    }

    for keyword in settings.keyword.iter() {
        let token = get_token(keyword).await?;
        let keyword_folder = keyword.to_case(Case::Snake);
        let keyword_folder = base.join(keyword_folder);

        if !keyword_folder.exists() {
            create_dir(&keyword_folder).expect("Could not create keyword folder");
        }
        let images = find_images(keyword, &token).await?;
        println!("Fetching images for {}", keyword);
        download_images(images, keyword_folder).await;
    }
    Ok(())
}
