/*!
```rudra-poc
[target]
crate = "av-data"
version = "0.2.1"

[test]
analyzers = ["Manual", "UnsafeDataflow"]
bug_classes = ["Other"]

[report]
issue_url = "https://github.com/rust-av/rust-av/issues/136"
issue_date = 2021-01-07
```
!*/
#![forbid(unsafe_code)]
use av_data::frame::*;
use av_data::pixel::*;

fn main() {
    let yuv420: Formaton = *formats::YUV420;
    let fm = std::sync::Arc::new(yuv420);
    let video_info = VideoInfo {
        pic_type: PictureType::I,
        width: 42,
        height: 42,
        format: fm,
    };
    let mut frame = new_default_frame(MediaKind::Video(video_info), None);

    frame.copy_from_raw_parts(
        vec![0 as usize as *const u8; 2].into_iter(),
        vec![40; 2].into_iter(),
    );

    println!("Program segfaults before reaching this point");
}
