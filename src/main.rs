// アプリケーションの状態は、同じスコープ内のすべてのルートとリソースで共有されます。
// 状態には、web :: Data <T>エクストラクタを使用してアクセスできます。
// ここで、Tは状態のタイプです。ミドルウェアでも状態にアクセスできます。

// 簡単なアプリケーションを作成し、アプリケーション名を次の状態で保存しましょう。

use actix_web::{ get, web, App, HttpServer };

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {}!", app_name)
}


// アプリを初期化するときに状態を渡し、アプリケーションを起動します。

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .data(AppState {
            app_name: String::from("Actix-Web"),
        })
        .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// アプリケーション内には、任意の数の状態タイプを登録できます。