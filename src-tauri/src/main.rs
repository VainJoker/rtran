#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::mpsc;

use tauri::async_runtime::spawn;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rstranslate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn rstranslate(word: &str) -> String {
    translate_o(word.to_string())
}

fn translate_o(word: String) -> String {
    let staticword = Box::leak(word.into_boxed_str());
    let params = [
        ("type", "AUTO"),
        ("i", staticword),
        ("doctype", "json"),
        ("version", "2.1"),
        ("keyfrom", "fanyi.web"),
        ("ue", "UTF-8"),
        ("action", "FY_BY_CLICKBUTTON"),
        ("typoResult", "true"),
    ];
    let (tx, rx) = mpsc::channel();
    spawn(async move {
        // println!("{:#?}",params);
        let res = reqwest::Client::new()
            .post("http://fanyi.youdao.com/translate?smartresult=dict&smartresult=rule")
            .form(&params)
            .send()
            .await
            .unwrap();
        //半天时间不知道.await
        // println!("{:#?}",res);
        let val = res.text().await.unwrap();
        tx.send(val).unwrap();
    });
    let result = rx.recv().unwrap();
    let v: serde_json::Value = serde_json::from_str(&result).unwrap_or_default();
    // json对象套数组
    // let your_input = &v["translateResult"][0][0]["src"];
    let your_output = &v["translateResult"][0][0]["tgt"];
    // println!("输入的词为:{}", your_input);
    // println!("翻译结果为:{}", your_output);
    your_output.to_string()
}
