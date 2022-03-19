use std::env::{args, current_dir};
use std::fs::{File, read_dir};
use std::io::Error;
use zip::ZipWriter;

mod util;

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    let path = args.get(1);
    if path.is_none() {
        util::err_exit("No pack directory specified");
    }
    let path = path.unwrap();
    let entries = read_dir(path).unwrap()
        .map(|res| res.map(|e| util::path_buf_to_string(&e.path())))
        .collect::<Result<Vec<_>, Error>>().unwrap();
    if !entries.contains(&format!("{}\\pack.mcmeta", path)) {
        util::err_exit("File that was inputted was not a valid Java resource pack");
    }
    let file = File::create("out.zip").unwrap();
    let mut zip = ZipWriter::new(file);
    read_recursive(path, &mut zip)?;
    println!("Successfully ported resource pack!");
    Ok(())
}

fn read_recursive(path: &String, zip: &mut ZipWriter<File>) -> Result<(), Error> {
    for item in read_dir(path).unwrap() {
        let new_path = item.unwrap().path();
        if new_path.is_dir() {
            zip.add_directory(&util::path_buf_to_string(&new_path), Default::default()).unwrap();
            read_recursive(&util::path_buf_to_string(&new_path), zip)?;
        } else {
            // TODO: Handle java to bedrock texture pack file system formats
            // REFERENCE: renames - https://github.com/rtm516/ConvertJavaTextureToBedrockApi/blob/712783785006e17ffc23a91920ea246d6a6c4618/src/converter/RenameConverter.mjs#L27-L1046
            let vec: Vec<&str> = path.split("\\").collect::<Vec<&str>>();
            let base_dir = *vec.get(3).unwrap(); // Hacky, should use string logic
            match base_dir {
                "mcpatcher" => {
                    // ihdk lol
                },
                "texts" => {
                    // Fonts
                },
                "textures" => {
                    // Textures... duh
                },
                _ => {
                    util::err_exit("Unknown directory detected");
                }
            }
        }
    }
    Ok(())
}