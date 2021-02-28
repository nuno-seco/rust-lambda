FROM scratch
ADD "./target/x86_64-unknown-linux-musl/release/bootstrap" "/"
ADD "./tmp/cenas" "/cenas"
ENTRYPOINT [ "/bootstrap" ]
