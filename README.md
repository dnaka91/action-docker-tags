# Docker Tags Action

Generate tags for Docker images based on the current ref-spec.

The new [Docker Build Push Action](https://github.com/docker/build-push-action) `v2` brings many
changes and removed the automatic tag generation based on the ref-spec from Git. This action
generates the Docker tags from the Git version so you don't have to include a huge bash script in
all of your workflow files that push Docker images.

## ⚠️ Deprecated ⚠️

This action has been deprecated in favor of the `docker/metadata-action` action which covers the
whole use case of this action and does even more.

## Migration to `docker/metadata-action`

The migration to the official Docker action is pretty straight forward and only requires you to
change the single step within your workflow.

This action didn't need any input to generate the tags as it was a fixed setup without much
configuration. Only if you needed tags for multiple registries you needed to set the `registries`
field.

The new official action requires a little bit of boilerplate. To get the same behavior you can use
the following settings:

```yaml
with:
  tags: |
    type=ref,event=branch
    type=semver,pattern={{version}}
    type=semver,pattern={{major}}.{{minor}}
    type=semver,pattern={{major}}
  flavor: |
    latest=true
```

This will create tags with the full **SemVer** version, only major and minor part, and lastly the
major part only. Also, a `latest` tag is created in addition to a new tag that is the branch name
that triggered the action.

**Note**: The new official action advises against using `type=semver,pattern={{major}}` if your
current version is not `1.0` yet, so if you you are still at `0.x.x`, don't add this part.

A more complete setup migration including multiple registries is provided below.

Previously with this action:

```yaml
name: ci
on: [push]
jobs:
  docker:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Generate tags
        id: docker_tags
        uses: dnaka91/action-docker-tags@v0.1
        with:
          registries: |
            docker.io
            ghcr.io
            quay.io
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v1
      - name: Login to DockerHub
        uses: docker/login-action@v1
        with:
          username: ${{ github.repository_owner }}
          password: ${{ secrets.DOCKER_PASSWORD }}
      - name: Login to GitHub Container Registry
        uses: docker/login-action@v1
        with:
          registry: ghcr.io
          username: ${{ github.repository_owner }}
          password: ${{ secrets.GITHUB_TOKEN }}
      - name: Login to Red Hat Quay.io
        uses: docker/login-action@v1
        with:
          registry: quay.io
          username: ${{ github.repository_owner }}
          password: ${{ secrets.QUAY_PASSWORD }}
      - name: Build and push
        uses: docker/build-push-action@v2
        with:
          push: true
          tags: ${{ steps.docker_tags.outputs.tags }}
```

Now with the new official action:

```yaml
name: ci
on: [push]
jobs:
  docker:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
      - name: Generate Docker metadata
        id: meta
        uses: docker/metadata-action@v3
        with:
          images: |
            docker.io/${{ github.repository }}
            ghcr.io/${{ github.repository }}
            quay.io/${{ github.repository }}
          tags: |
            type=ref,event=branch
            type=semver,pattern={{version}}
            type=semver,pattern={{major}}.{{minor}}
          flavor: |
            latest=true
      - name: Setup Docker Buildx
        uses: docker/setup-buildx-action@v1
      - name: Login to DockerHub
        uses: docker/login-action@v1
        with:
          username: ${{ github.repository_owner }}
          password: ${{ secrets.DOCKER_PASSWORD }}
      - name: Login to GitHub Container Registry
        uses: docker/login-action@v1
        with:
          registry: ghcr.io
          username: ${{ github.repository_owner }}
          password: ${{ secrets.GITHUB_TOKEN }}
      - name: Login to Red Hat Quay.io
        uses: docker/login-action@v1
        with:
          registry: quay.io
          username: ${{ github.repository_owner }}
          password: ${{ secrets.QUAY_PASSWORD }}
      - name: Build and push
        uses: docker/build-push-action@v2
        env:
          DOCKER_BUILDKIT: 1
        with:
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
```

## License

This project is licensed under the [AGPL-3.0 License](LICENSE) (or
<https://www.gnu.org/licenses/agpl-3.0.html>).
