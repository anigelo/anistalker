mod anime;
mod fs_scan;
mod fs_write;
mod metadata;
mod config;

mod prelude {
    pub use crate::config::*;
    pub use crate::anime::*;
}

use prelude::*;

fn main() {
    let animes = fs_scan::scan(&get_media_path());
    
    fs_write::save_to_data_folder(animes).unwrap();
}

