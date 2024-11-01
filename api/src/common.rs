use serde::{Deserialize,Serialize};
use std::path::PathBuf;

#[derive(Deserialize,Serialize)]
pub struct Image{
    image:Option<PathBuf> // get a path of image
}
