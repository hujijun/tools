use sha2::Digest;
use std::fs::File;
use std::path::Path;
use tar::Archive;

pub fn rename(file: &Path) {
    /* 文件改名 */
    let now_time = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let new_name = format!("{}.{}", file.to_string_lossy().to_string(), now_time);
    std::fs::rename(&file, new_name).unwrap();
}

pub fn tar_dir(target_path: &Path, source_file: &Path) -> String {
    /* tar包解压 */
    let mut _source_file = File::open(&source_file).unwrap();
    let mut archive = Archive::new(flate2::read::GzDecoder::new(&_source_file));
    if target_path.is_dir() {
        for _file in archive.entries().unwrap() {
            let mut _file = _file.unwrap();
            let target_file = target_path.join(_file.path().unwrap());
            let _parent_path: &Path = &target_file.parent().unwrap();
            if !_parent_path.exists() {
                // 上级目录不存创建
                std::fs::create_dir_all(_parent_path).unwrap();
            } else if target_file.exists() {
                // 文件存在执行改名
                rename(&target_file);
            }
            _file.unpack(target_file).unwrap();
        }
    } else {
        if target_path.is_file() {
            rename(&target_path);
        }
        // 目标目录不存在直接将tar包解压到目标目录
        archive.unpack(target_path).unwrap();
    }
    hex::encode(sha2::Sha256::digest(std::fs::read(source_file).unwrap()))
}


pub fn single_cp(target_file: &Path, source_file: &Path) -> String {
    /* 单文件拷贝 */
    if target_file.exists() {
        // 目标文件存在改名
        rename(&target_file);
    } else {
        // 父级目录不存在创建目录
        let target_path = target_file.parent().unwrap();
        if !target_path.exists() {
            std::fs::create_dir_all(&target_path).unwrap();
        }
    }
    std::fs::copy(&source_file, &target_file).unwrap();
    // 返回文件sha256
    hex::encode(sha2::Sha256::digest(std::fs::read(target_file).unwrap()))
}
