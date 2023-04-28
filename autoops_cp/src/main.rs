use std::path::Path;
use autoops_cp::{single_cp, tar_dir};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        print!("参数错误,命令格式: autoops_cp <source_file> <target> [tar]");
        std::process::exit(100)
    }
    let source_file = Path::new(args.get(1).unwrap());
    if !source_file.is_file() {
        print!("源文件不存在");
        std::process::exit(100)
    }
    let target = Path::new(args.get(2).unwrap());
    let hash_str = if args.len() == 4 && args.get(3).unwrap() == "tar" {
        tar_dir(target, source_file)
    } else {
        single_cp(target, source_file)
    };
    print!("{}", hash_str);
}
