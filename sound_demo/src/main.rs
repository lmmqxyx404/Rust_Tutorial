use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取音频主机
    let host = cpal::default_host();

    // 选择输入设备（麦克风）
    let device = host.default_input_device().expect("没有找到输入设备");
    println!("使用设备: {}", device.name()?);

    // 获取设备支持的默认输入格式
    let config = device.default_input_config()?;
    println!("输入配置: {:?}", config);

    // 使用 tokio 的 mpsc 通道来异步传输音频数据
    let (tx, mut rx) = mpsc::channel(32);
    let tx_clone = tx.clone();
    // 创建输入流，捕获音频数据
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // 将数据发送到异步通道
            let tx = tx_clone.clone();
            // 在回调中克隆数据，以避免生命周期问题
            let data_clone = data.to_vec();
            // 由于这里是在 cpal 的回调线程中，所以不能调用异步方法
            if let Err(err) = tx.blocking_send(data_clone) {
                eprintln!("发送音频数据时出错: {}", err);
            }
        },
        move |err| {
            eprintln!("录音过程中出错: {}", err);
        },
        None,
    )?;

    // 开始录音
    println!("开始录音...");
    stream.play()?;

    // WAV 文件格式参数
    let spec = WavSpec {
        channels: 1,         // 单声道
        sample_rate: 44100,  // 采样率 44.1kHz
        bits_per_sample: 32, // 32位浮点数
        sample_format: hound::SampleFormat::Float,
    };

    let mut wav_writer = WavWriter::create("output.wav", spec)?;

    // 异步写入 WAV 文件
    while let Some(data) = rx.recv().await {
        for sample in data {
            wav_writer.write_sample(sample)?;
        }
    }

    // 刷新并关闭 WAV 文件
    wav_writer.finalize()?;

    println!("录音数据已保存到 output.wav 文件");

    // 等待用户按下回车键停止录音
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    println!("录音结束");
    drop(stream); // 停止音频流

    Ok(())
}
