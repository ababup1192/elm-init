use clap::{App, Arg};
use sailfish::TemplateOnce;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(TemplateOnce)]
#[template(path = "sandbox.stpl")]
struct SandboxTmplate {}

#[derive(TemplateOnce)]
#[template(path = "element.stpl")]
struct ElementTmplate {
    flags: bool,
    ports: bool,
}

#[derive(TemplateOnce)]
#[template(path = "document.stpl")]
struct DocumentTmplate {
    flags: bool,
    ports: bool,
}

#[derive(TemplateOnce)]
#[template(path = "base_html.stpl")]
struct HtmlTmplate {
    flags: bool,
    ports: bool,
}

enum Mode {
    Sandbox,
    Element,
    Document,
    Application,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("elm-init")
        .version("0.0.1")
        .author("ABAB↑↓BA <@ababupdownba>")
        .about("elm-init CLI")
        .arg(Arg::with_name("mode").help("Elm app mode.\nsandbox — react to user input, like buttons and checkboxes\nelement — talk to the outside world, like HTTP and JS interop\ndocument — control the <title> and <body>\napplication — create single-page apps").required(true))
        .arg(Arg::with_name("flags").help("flags option").short("f").long("flags"))
        .arg(Arg::with_name("ports").help("ports option").short("p").long("ports"));

    let matches = app.get_matches();

    if let Some(mode) = matches.value_of("mode") {
        let mode = match mode {
            "sandbox" => Mode::Sandbox,
            "element" => Mode::Element,
            "document" => Mode::Document,
            "application" => Mode::Application,
            _ => panic!("Invalid mode."),
        };
        let flags = matches.is_present("flags");
        let ports = matches.is_present("ports");

        let elm_buf: Vec<u8>;
        let html_buf: Vec<u8>;
        let do_output: bool;
        match mode {
            Mode::Sandbox => {
                let elm_ctx = SandboxTmplate {};
                elm_buf = elm_ctx.render_once().unwrap().into_bytes();
                let html_ctx = HtmlTmplate {
                    flags: false,
                    ports: false,
                };
                html_buf = html_ctx.render_once().unwrap().into_bytes();
                do_output = true;
            }
            Mode::Element => {
                let elm_ctx = ElementTmplate {
                    flags: flags,
                    ports: ports,
                };
                elm_buf = elm_ctx.render_once().unwrap().into_bytes();
                let html_ctx = HtmlTmplate {
                    flags: flags,
                    ports: ports,
                };
                html_buf = html_ctx.render_once().unwrap().into_bytes();
                do_output = true;
            }
            Mode::Document => {
                let elm_ctx = DocumentTmplate {
                    flags: flags,
                    ports: ports,
                };
                elm_buf = elm_ctx.render_once().unwrap().into_bytes();
                let html_ctx = HtmlTmplate {
                    flags: flags,
                    ports: ports,
                };
                html_buf = html_ctx.render_once().unwrap().into_bytes();
                do_output = true;
            }
            Mode::Application => {
                elm_buf = "".to_string().into_bytes();
                html_buf = "".to_string().into_bytes();
                do_output = false;
                println!("You should use elm-spa( https://www.elm-spa.dev/ ).")
            }
        }
        if do_output {
            fs::remove_dir_all("output").unwrap_or(());
            fs::create_dir("output")?;
            fs::create_dir("output/src")?;
            let mut elm_file = File::create("output/src/Main.elm")?;
            elm_file.write_all(&elm_buf)?;
            elm_file.flush()?;
            let mut html_file = File::create("output/index.html")?;
            html_file.write_all(&html_buf)?;
            html_file.flush()?;
        }
    }

    Ok(())
}
