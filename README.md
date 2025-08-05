# rzip
Windows用のファイル・フォルダのzip圧縮ツール。

## ビルド
```cmd
git clone https://github.com/ragmilk/rzip.git
cd rzip
cargo build --release
```

## 使用方法
### ファイル・フォルダを圧縮
```cmd
rzip -e "C:\path\to\file"
rzip -e "C:\path\to\folder"
```

### zipファイルを解凍
```cmd
rzip -d "C:\path\to\archive.zip"
```

### 右クリックメニューに追加
```cmd
rzip --install
```

### 右クリックメニューから削除
```cmd
rzip --uninstall
```

## TODO
- 出力先のカスタム
- GUIの追加
  - パスワード付きzipへの対応