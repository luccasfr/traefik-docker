# Traefik Boilerplate

## What's included?

- Base Traefik configuration with Docker Compose.
- Rust application to add hosts to Traefik. (All Rust sources are in `./src`)

## How to start the Traefik service?

1. Create a Docker network named `web`

```bash
docker network create web
```

2. Replace your email in `docker-compose.yml`.

```
--certificatesresolvers.acmeresolver.acme.email=youremail@email.com
```

3. Run Traefik

```bash
docker compose up -d
```

## How to add a new host using the Rust application?

1. Build the Rust application (optional if using prebuilt)

```bash
cargo build --release
```

2. Run the Rust application

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

After following the steps, the application will guide you through the process of adding a new host. The application will add the host to `./config/services/service-name.yml`, which is a Docker volume.

3. Restart Traefik

```bash
chmod +x ./restart
./restart
```

## How to add a new host with Docker Compose?

1. Configure these labels and networks in your service's Docker Compose file:

```yaml
services:
  your-service:
    image: your-image
    labels:
      - "traefik.enable=true"
      # Replace yourhost.com with your host and your-service with your service name
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

Traefik will automatically detect the last exposed port of the service and add it to the router. If you want to use a different port, you can add the following label:

```yaml
- "traefik.http.services.your-service.loadbalancer.server.port=8080"
```

2. Restart Traefik

```bash
chmod +x ./restart
./restart
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
