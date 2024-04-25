##chacha20poly1305使用方法
1. 在./features/目录下生成key和nonce
    * cargo run -- text generate --key-path "./features/"
2. 执行 encrypt操作， 使用到第一步生成的key和nonce文件，输出打印到控制台
    * cargo run -- text encrypt  -k "./features/key.key" --nonce "./features/nonce.key"
3.  执行 decrypt操作， 使用到第一步生成的key和nonce文件，输出打印到控制台
    * cargo run -- text decrypt  -k "./features/key.key" --nonce "./features/nonce.key"
