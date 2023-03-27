# chatbot

chat with ChatGPT, surport 10 conversation contexts.


run server:
```
export PORT=10240 // default is 7878
export OPENAI_API_KEY=xxxxxxx
cargo run --bin server // you can also "cargo build" and run "target/release/server"
```

run client demo:
```
cargo run --bin client 127.0.0.1 10240
```


also use nc:
```
nc 127.0.0.1 10240
```
and input what you want to ask.


Deploy to Railway By One Click:

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/template/MrDA8-?referralCode=hpNd9_)
