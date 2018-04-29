extern crate web_view;

use web_view::*;

fn main() {
    let size = (700, 400);
    let resizable = true;
    let debug = false;
    let titlebar_transparent = true;
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();

    let html = format!(r#"
    <html>
        <head>
        <link href="https://fonts.googleapis.com/css?family=PT+Sans" rel="stylesheet">
        <style>{css}</style>
        </head>
        <body>
        <script>{js}</script>
        </body>
    </html>
    "#,
    css = r#"body { background: #1d1f21; }"#,
    js = include_str!("../www/dist.js"));

    run(
        "",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        titlebar_transparent,
        move |mut webview| {
            webview.set_background_color(0.11, 0.12, 0.13, 1.0);
        },
        frontend_cb,
        userdata
    );
}
