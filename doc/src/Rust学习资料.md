# 一、Rust官方教程
## 使用rustup查看本地文档
安装程序rustup还包括文档的副本存放在本地，可以方便地离线阅读。运行 `rustup doc` 在浏览器打开本地文档。(译注：自 `Rust Stable 1.14.0 `版本后，Rust 安装程序默认不再自带本地文档，需要添加本地文档的话，需要执行 `rustup component list` 查看 `rust-docs` 是否高亮，若无的话，则需要通过命令手动安装 `rustup component add rust-docs`，装好后 `rustup doc` 就可用了。)

遇到标准库提供的类型或函数不知道怎么用时，可在 API 文档找到相关的说明。 
## Rust教程
最佳教程：通过例子学习Rust。Rust中文社区有这两个文档（但是似乎没有翻译完已经停止更新了）：
* [Github:Rust例子](https://github.com/rust-lang-cn/rust-by-example-cn)   
* [Github:Rust之书](https://github.com/rust-lang-cn/book-cn) 另一个中文版Repo：[repo地址](https://github.com/KaiserY/trpl-zh-cn)   

官网文档阅读似乎有点卡，可以把代码克隆下来，使用mdbook自己构建，在本地学习rust。我在自己网站上部署了以上两个文档：
* [Rust之书](https://weiyinfu.cn/RustBook/)，也可以从[这里](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html) 阅读。
* [Rust例子](https://weiyinfu.cn/RustExample/)  

## API
如果想了解详细用法：[Rust API](https://doc.rust-lang.org/std/index.html)  
使用cargo查看库的API的文档：`cargo doc --open`

# 二、Rust第三方教程
Rust第三方教程质量较差，有的内容比较简略，有的从别处照搬照抄，有的直接机器翻译。建议先把官方教程浏览一遍之后，再看第三方教程。而官方教程浏览一遍之后，第三方教程就没有阅读的必要了。     
* [菜鸟网](https://www.runoob.com/rust/rust-tutorial.html)：这份教程的特点是简练笼统。
* [yibai教程](https://www.yiibai.com/rust/rust-slices.html)：这份教程是一位大佬的学习笔记，值得一看。  
* [极客学院](https://wiki.jikexueyuan.com/project/rust/dining-philosophers.html) ：这份教程是一份非常不走心的机器翻译教程，建议忽略。 
* [LearRust with Too Many LinkedList](https://weathfold.gitbooks.io/rust-too-many-lists-zhcn/content/first-layout.html)：通过链表来学习Rust，这个教程表述不够严谨，作者总是嘻嘻哈哈让人看得很累，建议忽略。
* [Rust Primer](https://rustcc.gitbooks.io/rustprimer/content/heap-stack/heap-stack.html)这份教程是国内Rust爱好者合作书写的，很多章节不够严谨，常常夹杂较多的作者偏见。内容跟官方教程有较多重叠，建议忽略。    
* [Rust学习笔记](https://photino.gitbooks.io/rust-notes/memory-safety.html)：内容比较少，比较提纲挈领，学完Rust Example之后再看会感觉比较容易理解。


# 三、Rust社区
* [知乎Rust专栏](https://www.zhihu.com/search?q=rust&type=column)
* [Rust中文社区](https://rustcc.cn/)
