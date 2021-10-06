use argh::FromArgs;
use std::error::Error;
use rs_ddg_images::{download_image, get_token, find_images, Response, SearchResult};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings: ImageSearch = argh::from_env();
    println!("Settings are: {:?}", settings);

    let search_term = &settings.keyword[0];
    let token = get_token(search_term).await;

    match token {
        Ok(token) => {
            println!("Found token: {}", token);
            match find_images(search_term, &token).await {
                Ok(images) => {
                    download_image(&images[0]).await;
                },
                Err(err) => println!("Error fetching image URLs: {:?}", err)
            };
            Ok(())
        },
        Err(err) => Result::Err(err)
    }
}
