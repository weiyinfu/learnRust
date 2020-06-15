crate有包、集装箱的含义，Rust将crate作为组件。  
crate有两种：库和二进制。
# 使用rustc来使用crate
crate 可以编译成二进制可执行文件（binary）或库文件（library）。默认情况 下，rustc 将从 crate 产生二进制可执行文件。这种行为可以通过 rustc 的选项 --crate-type 重载。

```plain
rustc --crate-type=lib haha.rs
```
会产生libhaha.rlib文件，生成的文件名默认为libxxx.rlib。。`rustc executable.rs --extern haha=libhaha.rlib`

# 使用cargo来使用crate
```sh
# 二进制可执行文件
cargo new foo

# 或者库
cargo new --lib foo
```
cargo支持来自网络的依赖
```plain
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # 来自 crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自网上的仓库
bar = { path = "../bar" } # 来自本地文件系统的路径
```

cargo的文件默认规则：
* main.rs为二进制文件入口
* lib.rs为库文件入口
* bin目录下可以放置多个rs文件
* 与src同级的tests文件夹存放集成测试代码，tests目录下每个文件都会编译成一个crate。如果多个crate之间有共用代码，只能以文件夹模块的方式来添加：`tests/common/mod.rs`

# 测试
默认情况下，测试时不会打印测试通过的case的输出。如果你希望也能看到通过的测试中打印的值，截获输出的行为可以通过`--nocapture`参数来禁用：
```shell script
$ cargo test -- --nocapture
```
    
