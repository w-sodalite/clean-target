## clean-target

一款小工具，用来迭代目录，删除程序的编译目标文件夹`target`。

注意：只会删除和`src`同级的`target`目录

### 编译

```shell
cargo build --release
```

### 使用方式

```shell
clean-target [directory]
```