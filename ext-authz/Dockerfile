FROM rust:1.67

WORKDIR /app

COPY . .

RUN apt update && apt install -y protobuf-compiler

RUN cargo install --path .

CMD ["ext-authz"]
