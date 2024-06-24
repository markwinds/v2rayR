#!/bin/bash

script_dir=$(cd "$(dirname "$0")";pwd)
web_dir=$script_dir/../web
user_end_dir=$script_dir/../user-end

# 彩色输出
# 第一个参数颜色
# 第二个参数输出内容
show()
{
    color="37m"
    case $1 in
    black)
        color="30m"
        ;;
    r)
        color="31m" # 红色
        ;;
    g)
        color="32m" # 绿色
        ;;
    y)
        color="33m" # 黄色
        ;;
    b)
        color="34m" # 蓝色
        ;;
    m)
        color="35m" # 品红
        ;;
    c)
        color="36m" # 青色
        ;;
    w)
        color="37m" # 白色
        ;;
    esac
    echo -e "\033[1;${color}$2\033[0m" # 1;表示加粗
}

show g "-------------build web-------------"
cd $web_dir
yarn install
if [ $? -ne 0 ]; then
    show r "npm install failed"
    exit 1
fi
yarn run build
if [ $? -ne 0 ]; then
    show r "npm run build failed"
    exit 1
fi


show g "-------------build user-end-------------"
cd $user_end_dir 
cargo build --release --features "gui"
if [ $? -ne 0 ]; then
    show r "cargo build failed"
    exit 1
fi


show g "---------------compress-----------------"
cd $user_end_dir 
output_dir=v2ray-r
mkdir $output_dir
cp target/release/v2ray-r $output_dir/
cp target/release/v2ray-r.exe $output_dir/
gzip $output_dir/*