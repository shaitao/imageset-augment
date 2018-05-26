extern crate clap;
extern crate image;

use clap::{App, Arg, ArgGroup};

use image::{GenericImage, imageops,ImageResult,DecodingResult};

use std::io;
use std::path::{Path, PathBuf};
use std::fs;




fn main() {
    println!("Hello, world!");
    let matches = App::new("Image Set Enhance")
        .version("0.0.1")
        .author("Sa Haitao")
        .about("截取, 翻转图片, 生成更多的图片.")
        .args(&[
            Arg::with_name("input_file")
                .help("the input file")
                .short("f")
                .long("file")
                .takes_value(true),
            Arg::with_name("input_directory")
                .help("the input directory")
                .short("d")
                .long("directory")
                .takes_value(true)
        ])
        .group(
            ArgGroup::with_name("input")
                .required(true)
                .args(&["input_file", "input_directory"])
        )
        .arg(
            Arg::with_name("cols")
                .help("generate rows x cols image from source image")
                .short("c")
                .long("cols")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("rows")
                .help("generate rows x cols image from source image")
                .short("r")
                .long("rows")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("width")
                .help("width of generated image ")
                .short("w")
                .long("width")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("height")
                .help("height of generated image ")
                .short("h")
                .long("height")
                .required(true)
                .takes_value(true)
        )
        .get_matches();

    let (r, c, w, h) = (
        matches.value_of("rows").map(|x| { x.parse::<u32>().expect("please input a integer") }).unwrap(),
        matches.value_of("cols").map(|x| { x.parse::<u32>().expect("please input a integer") }).unwrap(),
        matches.value_of("width").map(|x| { x.parse::<u32>().expect("please input a integer") }).unwrap(),
        matches.value_of("height").map(|x| { x.parse::<u32>().expect("please input a integer") }).unwrap()
    );

    println!("generator {}x{} ({}x{}) images form a source image", r, c, w, h);

    if let Some(file_name) = matches.value_of("input_file") {
        //处理单个文件
        progress_image(&file_name, r, c, w, h).unwrap();

    } else if let Some(dir) = matches.value_of("input_directory") {
        println!("Got input directory:{}", dir);
        //处理目录下所有文件
        progress_dir(dir, r, c, w, h).unwrap(); //打不开目录直接报错
    }
}

///处理单张图片, 从原始图片中挖出 rows x cols 张图片来
/// file: 图片路径
/// rows, cols :u32 横向,纵向产生的个数
///  width_s, height_s:u32 小图片的宽度,高度
fn progress_image<P: AsRef<Path>>(file: &P, rows: u32, cols: u32, width_s: u32, height_s: u32) -> Result<(),image::ImageError> {
    //file
    println!("progressing image {:?}.", file.as_ref());

    match image::open(file) {
        Ok(mut img) => {

            let (wdith, height) = img.dimensions();

            //小子图的中心坐标
            let mut w_c = wdith/(cols+1);

            let mut h_c = height/(rows+1);
            let mut w_stride = w_c;
            let mut h_stride = h_c;

            // 如果小图剪切范围太大
            //          |****|
            //      |****|
            //  |****|
            //    |*******|
            //需要缩放
            //      |****|
            //     |****|
            //    |****|
            //    |*******|
            //建立目录
            let  path = PathBuf::from("./results");
            if ! path.exists() {
                let dir_builder = fs::DirBuilder::new();
                dir_builder.create(&path).expect("failed to create directory.");
            }

            //修剪掉文件类型, rust的安全性真可怕,每一步会出错的地方都要处理...
            let file_name = file.as_ref().file_name().unwrap()
                .to_string_lossy()
                .split('.').next().unwrap()
                .to_string();

            let w_c_base = w_c;

            for r in 0..rows{

                w_c = w_c_base; //不这样 w_c 会一共加了 r*c 个 stride
                for c in 0..cols{
                    let subimg = imageops::crop(&mut img,w_c - width_s/2, h_c - height_s /2, width_s, height_s);

                    let mut path = path.clone();
                    path.push(file_name.clone() + &format!("_{}_{}.png",r,c));

                    subimg.to_image().save(&path).unwrap_or_else(|err|{
                        println!("failed to save file {:?} :Error: {}",&path,err)
                    });

                    w_c += w_stride;
                }
                h_c += h_stride;
            }
        Ok(())
        },

        Err(e) => {
            println!("open image error:{}",e);
            Result::Err(e)
        }
    }
}
///处理目录下的所有图片,不包含子目录的
/// dir: 目录
/// rows, cols :u32 横向,纵向产生的个数
///  width_s, height_s:u32 小图片的宽度,高度
fn progress_dir(dir: &str, r: u32, c: u32, width_s: u32, height_s: u32) -> io::Result<()> {
    println!("progressing directory. {}", dir);
    let mut files: Vec<PathBuf> = Vec::with_capacity(500);

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    };
    println!("{:?}", &files);
    Ok(())
}
