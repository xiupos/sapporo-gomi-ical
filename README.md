# sapporo-gomi-ical

札幌市の家庭ごみ収集日カレンダー (CSV) を iCalendar (`.ics`) に変換します。
[masaminh/sapporo_gc_calendar](https://github.com/masaminh/sapporo_gc_calendar) の Rust 実装です。

## Usage

Rust ([cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)) が必要です。CSV を[札幌市のオープンデータ](https://ckan.pf-sapporo.jp/dataset/garbage_collection_calendar)からダウンロードし、地区名を指定して実行します。地区名 (例: `北区1`) は[札幌市の案内](https://www.city.sapporo.jp/seiso/kaisyu/)で調べることができます。

```sh
cargo run --release -- -a 北区1 garvagecollectioncalendar202610.csv > kita1.ics
```

## References

- [masaminh/sapporo_gc_calendar](https://github.com/masaminh/sapporo_gc_calendar) (Python 実装)
- [札幌市家庭ごみ収集日カレンダー](https://ckan.pf-sapporo.jp/dataset/garbage_collection_calendar) (札幌市環境局環境事業部業務課、[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/deed.ja))
