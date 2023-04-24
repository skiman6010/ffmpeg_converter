use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tokio::task;

async fn process_files(
    input_directory: &str,
    output_directory: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();
    let mut video_list = Vec::new();
    for entry in fs::read_dir(input_directory)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "mp4" {
                let new_filename = format!("{}.mp4", path.file_stem().unwrap().to_string_lossy());
                let mut output_path = PathBuf::from(output_directory);
                output_path.push(&new_filename);

                // Spawn a new task for each ffmpeg process
                let task = task::spawn(async move {
                    let status = Command::new("ffmpeg")
                        .arg("-i")
                        .arg(path)
                        .arg("-c:v")
                        .arg("libx264")
                        .arg("-pix_fmt")
                        .arg("yuv420p")
                        .arg("-c:a")
                        .arg("copy")
                        .arg(output_path)
                        .status()
                        .expect("ffmpeg process failed to start");

                    if status.success() {
                        println!("Successfully converted {}", new_filename);
                        new_filename
                    } else {
                        eprintln!("Failed to convert {}", new_filename);
                        String::new()
                    }
                });

                tasks.push(task);
            }
        }
    }

    // Wait for all tasks to complete
    for task in tasks {
        match task.await {
            Ok(filename) => {
                if !filename.is_empty() {
                    video_list.push(filename);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // Save the list of videos to a file
    let video_list_path = PathBuf::from(output_directory).join("video_list.txt");
    let mut file = File::create(video_list_path)?;
    for video in video_list {
        writeln!(file, "{:#?}", video)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_directory> <output_directory>", args[0]);
        return;
    }

    let input_directory = &args[1];
    let output_directory = &args[2];

    // Create a new Tokio runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Run the process_files function inside the runtime
    match rt.block_on(process_files(input_directory, output_directory)) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
