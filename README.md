# imageset-augment
对图片集进行扩充, 通过窗口裁剪, 旋转等方式.

目录处理采用rayon并行

#### 使用方法

./imageset-augment -f <文件名>  -o <输出目录> -c <列数> -r <行数> -w <小窗宽度> -h <小窗高度>

./imageset-augment -d <目录名> -o <输出目录> -c <列数> -r <行数> -w <小窗宽度> -h <小窗高度>

输出目录不提供则保存到 ./results 下面