#[cfg(feature = "video")]
use ffmpeg_next::frame;

#[cfg(feature = "video")]
use ffmpeg_next::{self, software::scaling::flag::Flags, software::scaling};

#[cfg(feature = "video")]
use ffmpeg_next::media::Type;

use image::GrayImage;

use crate::args::Args;
use crate::braille::convert_size;
use crate::image_processing::{binarize, preprocess_image};
use crate::size::Size;
use crate::measure_time;

#[cfg(feature = "video")]
pub struct VideoData {
    pub frames: Vec<GrayImage>,
    pub size: Size,
    pub ratio: f32,
    pub fps: f32,
}

#[cfg(feature = "video")]
pub fn load_frames(
    path: &str,
    args: Args,
) -> Result<VideoData, Box<dyn std::error::Error>> {

    let mut ictx = ffmpeg_next::format::input(&path)?;
    let video_stream_index = ictx
        .streams()
        .best(Type::Video)
        .ok_or("No video stream")?
        .index();

    let context_decoder = ictx.stream(video_stream_index).ok_or("Invalid stream")?;
    let codec_params = context_decoder.parameters();
    let decoder = ffmpeg_next::codec::context::Context::from_parameters(codec_params)?;
    let mut decoder = decoder.decoder().video()?;
    // get frame per second
    let fps = get_fps(&ictx)?;

    let (w, h) = (decoder.width(), decoder.height());
    let (cols, rows) = convert_size(w, h, args.size.0, args.size.1);
    let size = Size(cols * 2, rows * 4);

    let mut scaler = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg_next::format::Pixel::GRAY8,
        cols * 2,
        rows * 4,
        Flags::BILINEAR,
    )?;

    let mut frames = Vec::new();
    let mut decoded = frame::Video::empty();
    let mut first_frame_saved = false;

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;

            while decoder.receive_frame(&mut decoded).is_ok() {

                let mut gray_frame = frame::Video::empty();
                scaler.run(&decoded, &mut gray_frame)?;

                let width = gray_frame.width() as usize;
                let height = gray_frame.height() as usize;
                let data = gray_frame.data(0);
                let linesize = data.len() / height;
                let mut img_buf = Vec::with_capacity(width * height);
                for y in 0..height {
                    let start = y * linesize;
                    let end = start + width;
                    img_buf.extend_from_slice(&data[start..end]);
                }
                // frame to image
                // let img = GrayImage::from_raw(
                //     size.0,
                //     size.1,
                //     gray_frame.data(0).to_vec()
                let img = GrayImage::from_raw(width as u32, height as u32, img_buf)
                    .ok_or("Failed to create image")?;

                // let img = GrayImage::from_raw(cols * 2, rows * 4, gray_frame.data(0).to_vec())
                //     .ok_or("Failed to create image")?;

                // Save the first frame to tmp.png
                if !first_frame_saved {
                    img.save("tmp.png")?;
                    first_frame_saved = true;
                }

                let img = measure_time!(preprocess_image(&img, args.contrast, args.invert));
                // ピクセルを二値化する
                let img = measure_time!(binarize(&img, args.binarize));

                frames.push(img);
            }
        }
    }

    decoder.send_eof()?;

    Ok(VideoData {
        frames,
        size,
        ratio: w as f32 / h as f32 * 2f32,
        fps,
    })
}

/// Get the FPS of a video file
#[cfg(feature = "video")]
pub fn get_fps(ictx: &ffmpeg_next::format::context::Input) -> Result<f32, String> {
    let video_stream = ictx.streams().best(Type::Video).ok_or("No video stream")?;
    let avg_frame_rate = video_stream.avg_frame_rate();
    Ok(avg_frame_rate.0 as f32 / avg_frame_rate.1 as f32)
}