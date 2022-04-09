release:
	cargo build --release
	cp ./target/release/uupdump .

docker: release
	docker build -t btwiuse/uupdump .

index:
	curl -sL https://uupdump.net > index.html
