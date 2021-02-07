use reqwest;
use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::{Class, Name};
use url::Url;
use std::thread;

fn main() {
    let bbs_url = "https://torrentsir31.com/bbs/search.php?search.php&stx=%EB%8F%99%EC%83%81%EC%9D%B4%EB%AA%BD2";
    let doc = get_doc(bbs_url);
    let urls = get_bbs_urls(&doc);
    let mut handles = vec![];
    for url in urls {
        let handle = thread::spawn(move || {
            let d = get_doc(&url);
            println!("{}", get_magnet(&d));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn get_bbs_urls(doc: &Document) -> Vec<String> {
    let mut urls = vec![];
    let base = Url::parse("https://torrentsir31.com/bbs/").unwrap();
    for node in  doc.find(Class("media-heading")) {
        let link = node
                .find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();
        let url =  base.join(link).unwrap().as_str().to_owned();
        urls.push(url);
    }
    urls
}

fn get_doc(url: &str) -> Document {
    let client = reqwest::blocking::Client::new();
    let res = client.get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_2_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36")
            .send().unwrap();
    let doc = Document::from_read(res).unwrap();
    doc
}

fn get_magnet<'a> (doc: &'a Document) -> &'a str {
    let mut magnet = "";
    for node in  doc.find(Class("list-group")) {
        let m = node
                .find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();
        if m.contains("magnet") {
            magnet = m;
        }
    }
    magnet
}

#[test]
fn test_get_magnet() {
    let url = "https://torrentsir31.com/bbs/board.php?bo_table=movie&wr_id=15835";
    let doc = get_doc(url);
    let got = get_magnet(&doc);
    let expected = "magnet:?xt=urn:btih:77c904927c0067cb3aadedae461e20c08eb11164";
    assert_eq!(got, expected);
}