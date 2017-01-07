default:
	go build floyd.go
	ocamlbuild -pkg Str floyd.byte
	javac floyd.java
