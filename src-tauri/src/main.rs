// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::command;
use base64::{decode, encode};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use std::{io::Cursor, vec};


use log::{debug, error, info, trace, warn};
use fern::Dispatch;
use chrono::Local;


static mut brightness_split: u8 =  125;

fn setup_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug) // 设置全局日志级别
        .chain(std::io::stdout())       // 输出到控制台
        .chain(fern::log_file("output.log")?) // 输出到文件
        .apply()?;
    Ok(())
}


#[tauri::command]
fn light_split(interval: Vec<u32>) -> u32 {
    0
}

#[tauri::command]
fn process_image(image_data: String) -> Result<String, String> {
    info!("image str process image");
    // 移除 Data URL 前缀
    let base64_data = image_data
        .split(',')
        .nth(1)
        .ok_or("Invalid image data")?;
    

    // 解码 Base64 数据
    let decoded_data = decode(base64_data).map_err(|e| e.to_string())?;

    // 加载图像
    let img = image::load_from_memory(&decoded_data).map_err(|e| e.to_string())?;

    // 处理图像
    let processed_img = analyze_brightness(img)?;

    // 将图像转换为 PNG 格式字节数组
    let mut buffer = Vec::new();
    processed_img
        .write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png)
        .map_err(|e| e.to_string())?;

    // 将处理后的图像编码为 Base64
    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);
    info!("Data process done...");

    Ok(data_url)
}

fn analyze_brightness(img: DynamicImage) -> Result<DynamicImage, String> {
    let (width, height) = img.dimensions();

    // 创建一个新的图像缓冲区
    let mut imgbuf = ImageBuffer::new(width, height);

    // 遍历每个像素
    for (x, y, pixel) in img.pixels() {
        // 获取像素的亮度（灰度值）
        let brightness = pixel.to_luma_alpha().0[0];

        // 根据亮度划分为三档
        let new_pixel = if brightness < 85 {
            // 低亮度，显示为蓝色
            Rgba([0, 0, 255, 255])
        } else if brightness < 170 {
            // 中亮度，显示为绿色
            Rgba([0, 255, 0, 255])
        } else {
            // 高亮度，显示为红色
            Rgba([255, 0, 0, 255])
        };

        imgbuf.put_pixel(x, y, new_pixel);
    }

    Ok(DynamicImage::ImageRgba8(imgbuf))
}

fn main() {
    setup_logger().expect("Failed to initialize logger.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_image, light_split])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}