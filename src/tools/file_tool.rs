use std::{
    fs::{self, File},
    path::Path,
};

pub fn create_dir(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn create_file(path: &str) -> std::io::Result<File> {
    let handle = File::create(path)?;
    Ok(handle)
}

pub fn get_path(path: &str) -> std::option::Option<&Path> {
    let sss = Path::new(path).parent()?;
    Some(sss)
}

pub fn super_create(path: &str) -> File {
    //判断是否带有目录
    if path.contains("/") {
        let path_new = get_path(path).unwrap();
        create_dir(path_new.as_os_str().to_str().unwrap()).unwrap();
        return create_file(path).unwrap();
    } else {
        return create_file(path).unwrap();
    }
}
