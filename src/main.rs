mod anime;
mod fs_scan;
mod fs_store;
mod metadata;
mod config;

mod prelude {
    pub use crate::config::*;
    pub use crate::anime::*;
}

fn main() {
    let animes = fs_scan::scan();

    fs_store::save_to_data_folder(animes).unwrap();
    
    let animes = fs_store::get_anime_collection().unwrap();
    
    println!("{:?}", animes);
}

