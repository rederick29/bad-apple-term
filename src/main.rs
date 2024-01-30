use std::{time::{Instant, Duration}, path::Path};
use std::io::{stdout, Write};
use image::GenericImageView;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event},
    style::{self, Stylize, PrintStyledContent},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

// Video frame rate
const TARGET_FRAME_RATE: f32 = 30.0;

// the duration caclucaltion itself takes some time. adjust for that here.
const CORRECTION_OFFSET: f32 = 0.75;
const EVENT_POLL_TIMEOUT: f32 = 1.0/(TARGET_FRAME_RATE + CORRECTION_OFFSET);

fn main() {
    let mut args = std::env::args();
    // executable path
    args.next();

    let ext = match args.next().unwrap_or("bmp".to_string()).as_str() {
        "jpg" => "jpg",
        "jpeg" => "jpeg",
        "bmp" => "bmp",
        "png" => "png",
        s => {
            println!("Error: unknown file extension type {s}");
            return;
        }
    };

    let mut w = stdout();
    run(&mut w, ext).unwrap_or_else(|e| {
        terminal::disable_raw_mode().unwrap();
        w.execute(cursor::Show).unwrap();
        w.execute(LeaveAlternateScreen).unwrap();
        println!("Error encountered: {e}");
    });
}

pub fn run<W: Write>(stdout: &mut W, file_ext: &str) -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(ClearType::All))?.execute(cursor::Hide)?;

    let (mut size_x, mut size_y) = terminal::size()?;
    let Ok(img) = image::open(format!("./images/1.{file_ext}")) else {
        return Err("Could not open the first image.".to_string().into());
    };
    let (img_width, img_height) = img.dimensions();

    let number_of_images = get_image_no(Path::new("./images/"), file_ext)?;

    // Resize image to fit in terminal. Does not maintain original aspect ratio.
    let mut scale_x: f32 = img_width as f32 / size_x as f32;
    let mut scale_y: f32 = img_height as f32 / size_y as f32;

    for n in 1..=number_of_images {
        let frame_start = Instant::now();
        let image = image::open(format!("./images/{}.jpg", n))?;
        let screen = vec![vec!['█'; size_x.into()]; size_y.into()];

        let frame = screen.into_iter().enumerate().flat_map(|(y, v)| {
            v.into_iter().enumerate().map(|(x, _)| {
                let p = image.get_pixel(
                    (x as f32 * scale_x) as u32,
                    (y as f32 * scale_y) as u32
                );
                '█'.with(style::Color::Rgb { r: p[0], g: p[1], b: p[2]})
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        frame.iter().for_each(|pixel| {
            stdout.queue(PrintStyledContent(*pixel)).unwrap();
        });
        stdout.flush()?;

        if event::poll(Duration::from_secs_f32(EVENT_POLL_TIMEOUT).checked_sub(Instant::now() - frame_start).unwrap_or(Duration::ZERO))? {
            match event::read()? {
                Event::Key(key_ev) => if key_ev.code == event::KeyCode::Char('q') { break; },
                Event::Resize(x, y) => {
                    size_x = x;
                    size_y = y;
                    scale_x = img_width as f32 / size_x as f32;
                    scale_y = img_height as f32 / size_y as f32;
                },
                _ => {},
            }
        }
    }

    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}

fn get_image_no(image_dir: &Path, file_ext: &str) -> std::io::Result<u16> {
    let mut no_frames = 0u16;
    for entry in std::fs::read_dir(image_dir)? {
        let path = entry?;
        if let Some(ext) = path.path().extension() {
            if ext == file_ext {
                no_frames += 1;
            }
        }
    }

    Ok(no_frames)
}
