一个包可以包含多个二进制crate项和一个可选的库crate

包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
Crates：一个模块树，可以产生一个库或可执行文件。
模块（Modules）和 use：允许你控制作用域和路径的私有性。
路径（path）：一个为例如结构体、函数或模块等项命名的方式。

## 包和crate
包是提供一系列功能的一个或多个crate的捆绑。 一个包会包含一个Cargo.toml文件

src/main.rs就是一个与包同名的二进制crate的crate根。
如果包目录中包含 src/lib.rs, 则包带有与其同名的库crate，且src/librs是crate根


## 定义模块来控制作用域与私有性
使模块公有并不使其内容也是公有的

模块上的 pub 关键字只允许其父模块引用它，而不允许访问内部代码。 这有什么用呢？