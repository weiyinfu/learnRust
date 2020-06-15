# 一、安装
国内安装需要设置镜像，新建一个sh文件，输入以下内容并运行，即可安装成功。
```sh
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
```

使用`cargo new hello-world --bin`可以创建一个二进制项目。在[Cargo之书](https://doc.rust-lang.org/cargo/index.html)中可以查看详细选项。  
`cargo build --release`以生产方式编译程序。    
## 补充：windows下安装Rust
首先下载`rustup-init.exe`，这一步毫无难度。  
然后执行此程序，一路yes和enter就可以。安装结束后产生两个目录：
* `~/.cargo`
* `~/.rustup`

Rust依赖Visual Studio构建工具，VisualStudio构建工具必须以环境变量的方式显式指明：
```plain
LIB=
C:\Program Files (x86)\Microsoft Visual Studio\2017\WDExpress\VC\Tools\MSVC\14.16.27023\lib\x64
C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64
C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\ucrt\x64
PATH=C:\Program Files (x86)\Microsoft Visual Studio\2017\WDExpress\VC\Tools\MSVC\14.16.27023\bin\Hostx86\x86
```

# 二、更新包索引
更新包索引也需要设置国内镜像，否则会很慢。~/.cargo/config
```plain
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```
更新包索引的命令为：`cargo metadata --verbose --format-version 1 --all-features`

