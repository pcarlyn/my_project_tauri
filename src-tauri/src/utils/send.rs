use std::process::Command;

pub fn open_mail_with_attachment() {
    let script = r#"tell application "Mail"
        activate
        set newMessage to make new outgoing message with properties {subject:"Archive", content:"See the attached file.", visible:true}
        tell newMessage
            make new attachment with properties {file name:"/Users/eboniequ/files/archive.zip"} at after last paragraph
        end tell
    end tell"#;

    let status = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .status();

    match status {
        Ok(_) => println!("Письмо создано в Apple Mail."),
        Err(e) => eprintln!("Ошибка: {}", e),
    }
}


// use lettre::{SmtpTransport, Transport, Message};
// use lettre::message::{SinglePart, MultiPart, Attachment};
// use std::fs::File;
// use std::io::Read;

// pub fn send_email_with_attachment() {
//     let attachment_path = "/Users/eboniequ/files/archive.zip"; // Укажите путь к файлу
//     let mut file = File::open(attachment_path).expect("Could not open file");
//     let mut file_content = Vec::new();
//     file.read_to_end(&mut file_content).expect("Could not read file");

//     let email = Message::builder()
//         .from("sender@example.com".parse().unwrap())
//         .to("receiver@example.com".parse().unwrap())
//         .subject("Test email with attachment")
//         .multipart(
//             MultiPart::mixed()
//                 .singlepart(SinglePart::plain("This is a test email with an attachment.".to_string())) // Текст письма
//                 .singlepart(
//                     Attachment::new("archive.zip".to_string()) // Имя файла (теперь String)
//                         .body(file_content, "application/zip".parse().unwrap()), // Тип MIME
//                 ),
//         )
//         .unwrap();

//     let mailer = SmtpTransport::builder_dangerous("localhost")
//         .port(1025)
//         .build();

//     match mailer.send(&email) {
//         Ok(_) => println!("Email sent successfully!"),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }




// use lettre::{SmtpTransport, Message, Transport};
// use lettre::message::{SinglePart};

// pub fn send_test_email() {
//     let email = Message::builder()
//         .from("sender@example.com".parse().unwrap())
//         .to("receiver@example.com".parse().unwrap())
//         .subject("Test email2")
//         .singlepart(SinglePart::plain(String::from("This is a test email"))) // Используем String
//         .unwrap();

//     let mailer = SmtpTransport::builder_dangerous("localhost")
//         .port(1025)
//         .build();

//     match mailer.send(&email) {
//         Ok(_) => println!("Email sent successfully!"),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }
