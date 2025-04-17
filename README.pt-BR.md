# Traefik Docker

[<img src="https://flagcdn.com/w20/us.png" alt="United States Flag"> English Version](./README.md)

## Requisitos

- Sistema operacional Linux
- Docker e Docker Compose instalados e funcionando

## O que está incluído?

- Configuração base do Traefik com Docker Compose.
- Aplicação Rust para adicionar hosts ao Traefik. (Todo o código-fonte Rust está em `./src`)

## Como iniciar o serviço Traefik?

1. Crie uma rede Docker chamada `web`

```bash
docker network create web
```

2. Substitua seu email no arquivo `docker-compose.yml`.

```
--certificatesresolvers.acmeresolver.acme.email=seuemail@email.com
```

3. Execute o Traefik

```bash
docker compose up -d
```

## Como adicionar um novo host usando a aplicação Rust?

1. Baixe a versão mais recente do GitHub

Acesse a [página de Releases](https://github.com/luccasfr/traefik-boilerplate/releases) e baixe a versão mais recente.

Alternativamente, você pode compilar a partir do código-fonte:

```bash
cargo build --release
```

2. Execute a aplicação Rust

Se você compilou a aplicação a partir do código-fonte:

```bash
chmod +x ./target/release/add-host
./target/release/add-host
```

Se você baixou uma versão pré-compilada:

```bash
chmod +x ./add-host*
./add-host*
```

Após seguir os passos, a aplicação irá guiá-lo pelo processo de adicionar um novo host. A aplicação adicionará o host ao arquivo `./config/services/nome-do-servico.yml`, que é um volume Docker.

3. Reinicie o Traefik

```bash
chmod +x ./restart
./restart
```

## Como adicionar um novo host com Docker Compose?

1. Configure estas labels e redes no arquivo Docker Compose do seu serviço:

```yaml
services:
  seu-servico:
    image: sua-imagem
    labels:
      - "traefik.enable=true"
      # Substitua seuhost.com pelo seu host e seu-servico pelo nome do seu serviço
      - "traefik.http.routers.seu-servico.rule=Host(`seuhost.com`)"
      - "traefik.http.routers.seu-servico.tls=true"
      - "traefik.http.routers.seu-servico.tls.certresolver=acmeresolver"
      - "traefik.http.routers.seu-servico.entrypoints=websecure"
    # Não se esqueça de adicionar o serviço à rede web
    networks:
      - web

# Não se esqueça de adicionar o serviço à rede web
networks:
  web:
    external: true
```

O Traefik detectará automaticamente a última porta exposta do serviço e a adicionará ao roteador. Se você quiser usar uma porta diferente, pode adicionar o seguinte label:

```yaml
- "traefik.http.services.seu-servico.loadbalancer.server.port=8080"
```

2. Reinicie o Traefik

```bash
chmod +x ./restart
./restart
```

## Autores

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