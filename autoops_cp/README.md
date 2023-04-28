### AutoopsCopy

#### 功能
1. 单文件拷贝 目标文件存在先改名[xx.20230209105022]再拷贝，目标父级目录不存在创建再拷贝

执行命令: autoops_cp <source_file> <target> [tar]
```shell
autoops_cp c:\aa c:\bb        # 将aa拷贝bb
autoops_cp c:\aa c:\bb        # bb存在先改名bb.20230209105022 再将拷贝
```
2. tar解压 目标目录不存在解压，解压文件存在先改名再解压
执行命令: autoops_cp <source_file> <target> tar
```shell
autoops_cp c:\aa.tar.gz c:\ tar   # 将 aa.tar.gz解压到 c盘
autoops_cp c:\aa.tar.gz c:\ tar   # 将 aa.tar.gz解压到 c盘 存在的文件改名xx.20230209105022 再将拷贝
```


#### 构建发布
```shell
# windows 使用
cargo build --release
# 使用静态链接解决依赖库问题
cargo build --target=x86_64-unknown-linux-musl --release
# linux 通过strip命令 去掉符号和调式信息减少包大小
strip target/release/autoops_cp
```
