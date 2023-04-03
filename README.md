# rust-cloudrun

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io#https://github.com/narumincho/rust-cloudrun)

GitHub でプライベートのメールアドレスを使っている場合のメール設定コマンドの例
(GitPod)

GitPod のバグのため. 報告されている (
https://github.com/gitpod-io/gitpod/issues/13323#issuecomment-1258232249 )

```bash
git config --global user.email 16481886+narumincho@users.noreply.github.com
```

## 実行方法

wasm-pack のインストールが必要です

```sh
cargo run -p client_build
cargo run -p server
```
