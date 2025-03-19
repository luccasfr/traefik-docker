# Traefik Boilerplate

## What's included?

- Base Traefik configuration with Docker Compose.
- Rust application to add hosts to Traefik. (All Rust sources are in `./src`)

## How to use it?

1. Build the Rust application (optional if using prebuilt)

```bash
cargo build --release
```

2. Run Traefik

```bash
docker-compose up -d
```

3. Run the Rust application

If you built the application:

```bash
./target/release/traefik
```

If you are using the prebuilt application:

For Windows:

```bash
./prebuilt/x86_64-pc-windows-msvc/traefik.exe
```

For Linux:

```bash
./prebuilt/x86_64-unknown-linux-musl/traefik
```

## Authors

<table>
  <tbody>
    <tr>
      <td align="center">
        <a href="https://github.com/luccasfr">
          <img src="https://github.com/luccasfr.png?size=100" alt="Lucas Ferreira" />
          <p>Lucas Ferreira</p>
        </a>
      </td>
    </tr>
  </tbody>
</table>
