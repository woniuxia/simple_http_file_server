use std::{fs, path::PathBuf};

use axum::{extract::Path, http::header, response::Response, routing::get, Router};
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let router = Router::new()
        .route("/", get(default_handler))
        .route("/*path", get(file_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, router).await?;

    Ok(())
}

async fn default_handler() -> Response<axum::body::Body> {
    let path = PathBuf::from("F:\\");
    process_file(path).await
}

async fn file_handler(Path(sub_path): Path<String>) -> Response<axum::body::Body> {
    let mut path = PathBuf::from("F:\\");
    path.push(sub_path);

    process_file(path).await
}

async fn process_file(path: PathBuf) -> Response<axum::body::Body> {
    if !path.exists() {
        let response = Response::builder()
            .header(header::CONTENT_DISPOSITION, "text/html")
            .body("path not exists".into())
            .unwrap();
        return response;
    }
    if path.is_file() {
        let mut content = Vec::new();
        let mut file = File::open(&path).await.unwrap();
        file.read_to_end(&mut content).await.unwrap();

        let response = Response::builder()
            .header(
                header::CONTENT_DISPOSITION,
                format!(
                    "attachment; filename=\"{}\"",
                    path.file_name().unwrap().to_str().unwrap()
                ),
            )
            .body(content.into())
            .unwrap();

        return response;
    }

    println!("path:{}", path.to_str().unwrap());

    let mut file_count = 0;
    let mut dir_count = 0;

    let mut html = String::new();
    html.push_str("<html><head><meta charset=\"utf-8\"></head><body>");
    html.push_str("<a href=\"..\">..</a><br>");

    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        let path_str = &path.to_str().unwrap()[3..];
        let path_str = path_str.replace("\\", "/");
        println!("{}", path_str);

        if path.is_dir() {
            dir_count += 1;
            html.push_str(&format!(
                "<a href=\"/{}\">DIR :\t{}</a><br>",
                path_str,
                entry.file_name().to_str().unwrap()
            ));
        } else {
            file_count += 1;
            html.push_str(&format!(
                "<a href=\"/{}\">FILE:\t{}</a><br>",
                path_str,
                entry.file_name().to_str().unwrap()
            ));
        }
    }

    html.push_str("<br>");
    html.push_str(&format!(
        "<p>dir count:{}<br>file count:{}</p>",
        dir_count, file_count
    ));
    html.push_str("</body></html>");
    Response::builder()
        .header(header::CONTENT_DISPOSITION, "text/html charset=utf-8")
        .body(html.into())
        .unwrap()
}
