use actix_web::{HttpResponse, Responder, web};
use mime_guess::from_path;
use rust_embed::Embed;

// 将静态文件嵌入到二进制程序中 默认是Cargo.toml的相对路径
#[derive(Embed)]
#[folder = "../web/dist/"]
struct Asset;

// pub fn get_program_icon() {
//     match Asset::get("favicon.ico") {
//         Some(content) => {
//             content.data.into_owned();
//         }
//         None => {}
//     }
// }

// 处理静态文件请求 返回对应的文件
fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

// 重定向请求“/”到index.html文件
#[actix_web::get("/")]
pub(crate) async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

// 静态文件请求url路径以web开头，需要和vite.config.js中的base配置匹配
#[actix_web::get("/web/{_:.*}")]
pub(crate) async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}
