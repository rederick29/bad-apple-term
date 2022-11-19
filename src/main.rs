use std::env;
use std::io::{stdout, Write};
use image::GenericImageView;
use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event},
    style::{self, Stylize},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

// Time given between frames for checking for new terminal events. Can also be used to speed up/slow
// down playback.
const EVENTS_POLLING_TIMEOUT: f32 = 1.0/144.0;

fn main() {
    let mut w = stdout();
    run(&mut w, check_truecolour_support()).unwrap();
}

pub fn run<W>(stdout: &mut W, truecolour: bool) -> Result<()>
where
    W: Write,
{
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(ClearType::All))?.execute(cursor::Hide)?;

    let (mut size_x, mut size_y) = terminal::size()?;
    let (img_width, img_height) = image::open("./images/1.bmp").unwrap().dimensions();
    let number_of_images = get_image_no(std::path::Path::new("./images/")).unwrap();

    // Resize image to fit in terminal. Does not maintain original aspect ratio.
    let mut scale_x: f32 = img_width as f32 / size_x as f32;
    let mut scale_y: f32 = img_height as f32 / size_y as f32;

    for n in 1..=number_of_images {
        let image = image::open(format!("./images/{}.bmp", n)).unwrap().into_luma8();

        for y in 0..size_y {
            for x in 0..size_x {
                let pixel = image.get_pixel(
                    (x as f32 * scale_x) as u32,
                    (y as f32 * scale_y) as u32
                );
                let colour = pixel[0];

                // If terminal supports truecolour, use it. Otherwise keep it limited to only 2
                // colours.
                if truecolour {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("█".with(style::Color::Rgb { r: colour, g: colour, b: colour })))?;
                } else if colour >= 130 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print("█"))?;

                } else {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::Print(" "))?;
                }
            }
        }
        stdout.flush()?;

        if event::poll(std::time::Duration::from_secs_f32(EVENTS_POLLING_TIMEOUT))? {
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

fn get_image_no(image_dir: &std::path::Path) -> std::io::Result<u16> {
    let mut no_frames = 0u16;
    for entry in std::fs::read_dir(image_dir)? {
        let path = entry?;
        if let Some(ext) = path.path().extension() {
            if ext == "bmp" {
                no_frames += 1;
            }
        }
    }

    Ok(no_frames)
}

fn check_truecolour_support() -> bool {
    if let Ok(var) = env::var("COLORTERM") {
        if var == "truecolor" || var == "24bit" {
            return true;
        }
    }
    if env::var("WT_SESSION").is_ok() {
        return true;
    }
    false
}
