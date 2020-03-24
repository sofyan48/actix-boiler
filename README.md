# Actix Boiler
## Required

- [Rustup](https://rustup.rs/)
- Stable Toolchain: `rustup default stable`
- Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
- PostgreSQL database server or use our docker-compose.yml (require docker)

## Getting Started

```sh
git clone https://github.com/sofyan48/actix-boiler.git
cd actix-boiler
docker-compose up
cp .env.example .env
diesel setup --database-url='postgres://postgres:admin:qazwsx123@localhost/actix-boiler'
diesel migration run
cargo run
```

## Build release

```sh
cargo build --release
cd target/release
./canduma
```

## Security

### Important security considerations

We use session cookies for authentication.

**Why not JWT authentication?**

[Stop Using JWT for sessions and why your solution doesn't work](http://cryto.net/~joepie91/blog/2016/06/19/stop-using-jwt-for-sessions-part-2-why-your-solution-doesnt-work/)

The use of JWT remains secure only if you use adequate storage.
This boilerplate is built for use in a micro-services architecture.

JWT can be use for representing claims to be transferred between two parties.

The private key should only be on this micro-service.
public key can be used on all other parties to decode the token.

This boilerplate provides a complete example, so we included JWT also.

### Generate RSA keys for JWT

In development mode you can keep the one in `/keys` folder.

```shell script
// private key
$ openssl genrsa -out rs256-4096-private.rsa 4096

// public key
$ openssl rsa -in rs256-4096-private.rsa -pubout > rs256-4096-public.pem
```
