grpcurl -plaintext -import-path ../proto -proto hello.proto -d '{"name": "Tonic"}' localhost:50051 hello.MyService/Hello

