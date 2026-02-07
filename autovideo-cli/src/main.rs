use std::fs;
use std::path::PathBuf;
use autovideo_core::{process_videos, Mode};
use clap::Parser;

/// CLI application to automatically make textures, .esp and .nif files for a VotW mod.
/// 
/// To start make sure you have a video ready, run the application with a mod name of your choice and the name of your video.
/// To add additional videos to an esp, use the --esp (and --desp) flag,
/// not doing this will create a new esp (and overwrite the old one if it's still in the output directory)
/// An esp can only support up to 10 videos, trying to add more will still make the textures and meshes, but the esp won't be able to update anymore.
/// Make sure you have ffmpeg installed.
#[derive(Parser)]
#[command(version, verbatim_doc_comment)]
struct Args {
    /// Name of the mod. At most 10 character!
    mod_name: String,

    /// Path to video or folder of videos to convert.
    /// 
    /// Names of video files will be used to name the holotapes. In case of single video file, name can be overwritten using "-n".
    #[arg(short, long)]
    input: PathBuf,

    /// Name to use for this video, overwrites name of input video. At most 10 character!
    /// 
    /// This option is ignored when "--input" is a folder.
    #[arg(short = 'n', long)]
    video_name: Option<String>,

    /// Path to existing esp to append to that one
    /// 
    /// This will create a copy in the output folder and not directly edit given one
    #[arg(long = "esp", value_name = "ESP FILE")]
    input_esp: Option<PathBuf>,

    /// Path to existing driveIn esp to append to that one
    /// 
    /// This will create a copy in the output folder and not directly edit given one
    #[arg(long = "desp", value_name = "DRIVEIN ESP FILE")]
    input_esp_drive_in: Option<PathBuf>,

    /// Size of output frames
    /// 
    /// Determines video resolution in-game. Switch to 256 in case you want to preserve drive space.
    #[arg(short, long, default_value_t = 512)]
    size: u32,
    
    /// Will automatically refit input to 4:3 aspect ratio. (Which fits FO4 TVs better)
    #[arg(short, long)]
    keep_aspect_ratio: bool,
    
    /// Enable to not give a warning for names being too long and to automatically cut them shorter
    #[arg(long)]
    short_names: bool,
    
    /// For advanced users. Generates a FO4Edit script to add video records to existing esp. No esps will be generated
    /// 
    /// Useful for when you already have an existing VotW esp, either a full one made by autovideo or one you made yourself
    #[arg(short, long)]
    generate_script: bool,
    
    /// Say YES to all warnings causing them to be ignored, e.g. too many videos and video too long warnings
    #[arg(short, long)]
    yes: bool,
    
    /// Framerate at which to play the videos in-game
    /// 
    /// Alternatively you can put the wanted framerate in the video filename like this: video.30fps.mp4
    #[arg(short = 'r', long, default_value_t = 10)]
    framerate: u32,

    /// Enable High Quality
    ///
    /// High Quality will result in better visuals but double the filesize and take longer to process
    #[arg(short, long)]
    quality: bool,
}


fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut inputs = vec![];
    if args.input.exists() {
        if args.input.is_file() {
            inputs.push(args.input);
        } else if args.input.is_dir() {
            for input in fs::read_dir(args.input).unwrap().flatten() {
                let path = input.path();
                if path.is_file() {
                    inputs.push(path);
                }
            }
        }
    } else {
        return Err(format!("File or folder does not exist: {}", &args.input.to_str().unwrap()));
    }
    
    process_videos(
        inputs,
        args.input_esp,
        args.input_esp_drive_in,
        args.mod_name,
        args.framerate,
        args.short_names,
        args.video_name,
        args.size,
        args.keep_aspect_ratio,
        args.generate_script,
        None,
        if args.yes { Mode::YES } else { Mode::NO },
        || {},
        args.quality
    )?;
    
    Ok(())
}
