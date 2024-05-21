all:
	cargo build
	rm -f ./secure_classifier
	mv target/debug/secure_classifier ./secure_classifier