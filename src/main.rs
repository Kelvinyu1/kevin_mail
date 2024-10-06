use clap::Parser;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    email: String,
    message: String,
    filename: String,
}

fn main() {
    let args = Cli::parse();

    // Get the current working directory where the user is running the command
    let curr_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", curr_dir);

    // Join the filename to the current directory path
    let file_path: PathBuf = curr_dir.join(&args.filename);
    println!("Looking for file at: {:?}", file_path);

    // Read the file from the specified path
    let file_content = fs::read(&file_path).expect("Failed to read the file");

    // Set the content type for a plain text file
    let content_type = ContentType::TEXT_PLAIN;

    // Create the email message
    let email = Message::builder()
        .from("Kelvin Yu <kelvinyu86@gmail.com>".parse().unwrap())
        .to(args.email.parse().unwrap())
        .subject("Sending email with Rust")
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::plain(args.message.clone()))
                .singlepart(
                    Attachment::new(args.filename.clone()).body(file_content, content_type),
                ),
        )
        .unwrap();

    // Set up the credentials for sending the email
    let creds = Credentials::new(
        "kelvinyu86@gmail.com".to_string(),
        "xxxxxxxxxxxxxxxx".to_string(),
    );

    // Open a remote connection to Gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
