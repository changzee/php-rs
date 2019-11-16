#### PHP-SYS

Bindings to php.

#### BUILD STEP

````
// 编译安装PHP
$ wget -c https://www.php.net/distributions/php-7.3.10.tar.gz
$ tar xf php-7.3.10.tar.gz
$ cd php-7.3.10
// 请确保使用CLANG进行make，不然将会报错（clang不支持asm goto指令）
$ ./configure --enable-debug --enable-embed=shared --enable-maintainer-zts --enable-libxml --disable-dom --enable-xml  --disable-simplexml --disable-xmlwriter --disable-xmlreader --without-pear  --disable-phar CC=clang CFLAGS="-O3 -march=native"
$ make && make install 
// BUILD环境变量
$ export PHP_LIB_DIR=... # php lib所在路径
$ export PHP_INCLUDE_DIR=... # php header所在路径
$ export PHP_LINK_STATIC=... # 导入PHP静态/动态运行库，true为静态，false为动态
````

