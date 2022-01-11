use clap::{App, Arg};
use std::io;

fn get_args(flag: &mut bool) -> Option<String> {
    let matches = App::new("A Translation Tool")
        .version("0.1")
        .author("VainJoker <vainjoker@tuta.io>")
        .arg(
            Arg::new("online_translate")
                .short('N')
                .long("on")
                .takes_value(true),
        )
        .arg(
            Arg::new("offline_translate")
                .short('F')
                .long("off")
                .takes_value(true),
        )
        .get_matches();

    let mut arg_online_word: Option<String> = None;
    let mut arg_offline_word: Option<String> = None;
    // You can check the value provided by positional arguments, or option arguments
    if let Some(i) = matches.value_of("online_translate") {
        // println!("Value for input: {}", i);
        arg_online_word = Some(i.to_string());
    }

    if let Some(c) = matches.value_of("offline_translate") {
        // println!("Value for config: {}", c);
        arg_offline_word = Some(c.to_string());
        *flag = false;
    }
    if *flag {
        // println!("{:?}",arg_online_word);
        arg_online_word
    } else {
        // println!("{:?}",arg_offline_word);
        arg_offline_word
    }
}

#[tokio::main]
async fn online_translate(word: String) -> Result<(), Box<dyn std::error::Error>> {
    let params = [
        ("type", "AUTO"),
        ("i", &word),
        ("doctype", "json"),
        ("version", "2.1"),
        ("keyfrom", "fanyi.web"),
        ("ue", "UTF-8"),
        ("action", "FY_BY_CLICKBUTTON"),
        ("typoResult", "true"),
    ];

    let res = reqwest::Client::new()
        .post("http://fanyi.youdao.com/translate")
        .form(&params)
        .send()
        .await?;

    //半天时间不知道.await
    let result = res.text().await?;
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    //json对象套数组
    let your_input = &v["translateResult"][0][0]["src"];
    let your_output = &v["translateResult"][0][0]["tgt"];
    println!("输入的词为:{}", your_input);
    println!("翻译结果为:{}", your_output);
    Ok(())
}

fn offline_translate(word: String) {
    println!("offline_translate\n{}", word);
}

fn main() {
    let mut flag: bool = true;
    //通过函数改变本身的值用可变引用
    let mut word: Option<String> = get_args(&mut flag);
    if word == None {
        let mut buf = String::new();
        println!("输入你要翻译的词:");
        io::stdin().read_line(&mut buf).ok().expect("Error!");
        word = Some(buf.trim().to_string());
    }
    if flag {
        online_translate(word.unwrap());
    } else {
        offline_translate(word.unwrap());
    }
}
