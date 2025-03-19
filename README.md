# Traefik Boilerplate

## What's included?

- Base Traefik configuration with Docker Compose.
- Rust application to add hosts to Traefik. (All Rust sources are in `./src`)

## How to use it?

1. Build the Rust application

```bash
cargo build --release
```

2. Run Traefik

```bash
docker-compose up -d
```

3. Run the Rust application

```bash
./target/release/traefik
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
