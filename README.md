# gokaku

五格を計算します。

- 天格
- 人格
- 外格
- 地格
- 総格

## 使い方

```
$ cargo run -- -help
warning: unused manifest key: package.author
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/gokaku -help`
gokaku 0.1.0
Gokaku(五格) Calculator

USAGE:
    gokaku <LAST_NAME> <FIRST_NAME>

ARGS:
    <LAST_NAME>     
    <FIRST_NAME>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## 使用しているAPI

- https://apino.yukiyuriweb.com/kanji
- https://rapidapi.com/KanjiAlive/api/learn-to-read-and-write-japanese-kanji/
  - サポートされている漢字が少ない
  - 呼び出し部を用意しているが、実際には使っていない
