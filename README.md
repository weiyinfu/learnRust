# 一、安装
国内安装需要设置镜像，新建一个sh文件，输入以下内容并运行，即可安装成功。
```sh
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
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

# 三、Rust官方教程
## 使用rustup查看本地文档
安装程序还包括文档的副本存放在本地，可以方便地离线阅读。运行 `rustup doc` 让浏览器上打开本地文档。(译注：自 `Rust Stable 1.14.0 `版本后，Rust 安装程序默认不再自带本地文档，需要添加本地文档的话，需要执行 `rustup component list` 查看 `rust-docs` 是否高亮，若无的话，则需要通过命令手动安装 `rustup component add rust-docs`，装好后 `rustup doc` 就可用了。)

遇到标准库提供的类型或函数不知道怎么用时，可在 API 文档找到相关的说明。 
## Rust教程
最佳教程：通过例子学习Rust。Rust中文社区有这两个文档（但是似乎没有翻译完已经停止更新了）：
* [Rust例子](https://github.com/rust-lang-cn/rust-by-example-cn)  
* [Rust之书](https://github.com/rust-lang-cn/book-cn)
* [Rust API](https://doc.rust-lang.org/std/index.html)

如果想看中文，这里有一份翻译比较完全的文档：
* [Rust之书](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html) [repo地址](https://github.com/KaiserY/trpl-zh-cn) 

虽然可以直接从官网上学习Rust例子，但是官网网站似乎有点卡，可以把代码克隆下来，使用mdbook自己构建，在本地学习rust。我在自己网站上部署了以上两个文档：
* [Rust之书](https://weiyinfu.cn/RustBook/)  
* [Rust例子](https://weiyinfu.cn/RustExample/)  



# 四、Rust第三方教程
[菜鸟网](https://www.runoob.com/rust/rust-tutorial.html)  
[一百教程](https://www.yiibai.com/rust/rust-slices.html) 
[极客学院](https://wiki.jikexueyuan.com/project/rust/dining-philosophers.html) 

# 五、Rust社区
[知乎Rust专栏](https://www.zhihu.com/search?q=rust&type=column)
[Rust中文社区](https://rustcc.cn/)

# 六、使用cargo查看库的API的文档
`cargo doc --open`

# 七、寻找好玩的库
`https://crates.io/`

# 八、Rust调试
Rust回溯Rust回溯是调用的所有函数的列表，用于了解“导致错误的原因”。 需要设置RUST_BACKTRACE环境变量来获取回溯。

# 九、安全与速度不可兼得
数组下标越界之后，程序是否应该继续执行？C和C++给出的答案是应该继续执行，它没有数组下标越界的检查，这就使得每次访问数组速度较快。  

# 十、学习路线
* 看文档，练习使用
* 学习API，学习标准库和常用库