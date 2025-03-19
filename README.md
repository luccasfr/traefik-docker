# Traefik Boilerplate

## What's included?

- Base Traefik configuration with Docker Compose.
- Rust application to add hosts to Traefik. (All Rust sources are in `./src`)

## How to use it?

1. Build the Rust application (optional if using prebuilt)

```bash
cargo build --release
```

2. Replace your email in `docker-compose.yml`. Currently (youremail@email.com)

3. Run Traefik

```bash
docker-compose up -d
```

4. Run the Rust application

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

## How to add a new host with docker-compose?

```yaml
services:
  your-service:
    image: your-image
    labels:
      - "traefik.enable=true"
      # replace yourhost.com with your host and your-service with your service name
      - "traefik.http.routers.your-service.rule=Host(`yourhost.com`)"
      - "traefik.http.routers.your-service.tls=true"
      - "traefik.http.routers.your-service.tls.certresolver=acmeresolver"
      - "traefik.http.routers.your-service.entrypoints=websecure"
    # Don't forget to add the service to the web network
    networks:
      - web

# Don't forget to add the service to the web network
networks:
  web:
    external: true
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
