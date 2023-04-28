use sha2::Digest;
use autoops_cp;


#[test]
fn test_single_cp() {
    // 测试单文件拷贝
    let source_file = std::path::Path::new("D:\\aa");
    let target_file = std::path::Path::new("D:\\demo.txt");
    let file_sha1 = hex::encode(sha2::Sha256::digest(std::fs::read(&source_file).unwrap()));
    let file_sha2 = autoops_cp::single_cp(target_file, source_file);
    assert_eq!(file_sha1, file_sha2)
}

#[test]
fn test_tar_cp() {
    // 测试拷贝 tar 解决
    let source_file = std::path::Path::new(r"D:\aa.tar.gz");
    let target_path = std::path::Path::new(r"D:\bb");
    let file_sha1 = hex::encode(sha2::Sha256::digest(std::fs::read(&source_file).unwrap()));
    let file_sha2 = autoops_cp::tar_dir(target_path, &source_file);
    assert_eq!(file_sha1, file_sha2)
}
