# send email

* help
```shell
$send-mail --help
Usage: send-mail [OPTIONS] --subject <SUBJECT> --body <BODY>

Options:
  -s, --smtp-server <SMTP_SERVER>
          smtp server host
  -u, --username <USERNAME>
          smtp username
  -p, --password <PASSWORD>
          smtp password
  -f, --from <FROM>
          from [name:email] (name:xxx@xxx.com)
  -t, --to <TO>
          to [name:email|name:email|..] (name1:xxx1@xxx.com|name2:xxx2@xxx.com|...)
      --subject <SUBJECT>

      --body <BODY>
          body
      --body-html <BODY_HTML>
          html body
  -a, --attachment-file <ATTACHMENT_FILE>
          attachment file path
  -h, --help
          Print help
```

* env
```shell
export SMTP_SERVER=smtp.xxx.xx
export MAIL_USERNAME=xxxxxx
export MAIL_PASSWORD=xxxxxx
export MAIL_FROM=name:xxx@xxx.com
export MAIL_TO=name1:xxx1@xxx.com|name2:xxx2@xxx.com|...

send-mail --subject "subject" --body "body" --body-html "<h1>body html</h1>" --attachment-file attachment-file 
```