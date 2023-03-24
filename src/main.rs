use anyhow::{anyhow, bail, Context};
use clap::Parser;
use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut opt = MailOpt::parse();

    if opt.smtp_server.is_none() {
        opt.smtp_server =
            Some(std::env::var("SMTP_SERVER").map_err(|err| anyhow!("SMTP_SERVER:{err}"))?);
    }

    if opt.username.is_none() {
        opt.username =
            Some(std::env::var("MAIL_USERNAME").map_err(|err| anyhow!("MAIL_USERNAME:{err}"))?);
    }

    if opt.password.is_none() {
        opt.password =
            Some(std::env::var("MAIL_PASSWORD").map_err(|err| anyhow!("MAIL_PASSWORD:{err}"))?);
    }

    if opt.from.is_none() {
        opt.from = Some(std::env::var("MAIL_FROM").map_err(|err| anyhow!("MAIL_FROM:{err}"))?);
    }

    if opt.to.is_none() {
        opt.to = Some(std::env::var("MAIL_TO").map_err(|err| anyhow!("MAIL_TO:{err}"))?);
    }

    let mut message = MessageBuilder::new();
    let from_str = opt.from.unwrap();
    if from_str.split(':').count() == 2 {
        let mut from = from_str.split(':');
        message = message.from((from.next().unwrap(), from.next().unwrap()));
    } else {
        message = message.from(from_str);
    }

    let to_str = opt.to.unwrap();
    let to = to_str
        .split('|')
        .map(|to| {
            if to.split(':').count() == 2 {
                let mut to_sp = to.split(':');
                (to_sp.next().unwrap(), to_sp.next().unwrap())
            } else {
                (to, to)
            }
        })
        .collect::<Vec<_>>();

    message = message.to(to).subject(opt.subject).text_body(opt.body);

    if let Some(ref body_html) = opt.body_html {
        message = message.html_body(body_html);
    }

    if let Some(ref attachment_file) = opt.attachment_file {
        if attachment_file.is_file() && attachment_file.exists() {
            let data = std::fs::read(attachment_file)?;
            let filename = attachment_file.file_name().unwrap().to_str().unwrap();
            message = message.binary_attachment("application/octet-stream", filename, data);
        } else {
            bail!("attachment_file is error");
        }
    }

    //application/octet-stream
    //message.binary_attachment()

    SmtpClientBuilder::new(opt.smtp_server.context("smtp server is none")?, 25)
        .implicit_tls(false)
        .credentials((
            opt.username.map_or("".to_string(), |x| x),
            opt.password.map_or("".to_string(), |x| x),
        ))
        .connect()
        .await?
        .send(message)
        .await?;
    Ok(())
}

#[derive(Parser, Debug)]
pub struct MailOpt {
    /// smtp server host
    #[arg(short, long, value_parser)]
    pub smtp_server: Option<String>,
    /// smtp username
    #[arg(short, long, value_parser)]
    pub username: Option<String>,
    /// smtp password
    #[arg(short, long, value_parser)]
    pub password: Option<String>,
    /// from [name:email] (name:xxx@xxx.com)
    #[arg(short, long, value_parser)]
    pub from: Option<String>,
    /// to [name:email|name:email|..] (name1:xxx1@xxx.com|name2:xxx2@xxx.com|...)
    #[arg(short, long, value_parser)]
    pub to: Option<String>,
    #[arg(long, value_parser)]
    pub subject: String,
    /// body
    #[arg(long, value_parser)]
    pub body: String,
    /// html body
    #[arg(long, value_parser)]
    pub body_html: Option<String>,
    /// attachment file path
    #[arg(short, long, value_parser)]
    pub attachment_file: Option<PathBuf>,
}
