# Docker Tags Action

Generate tags for Docker images based on the current ref-spec.

The new [Docker Build Push Action](https://github.com/docker/build-push-action) `v2` brings many
changes and removed the automatic tag generation based on the ref-spec from Git. This action
generates the Docker tags from the Git version so you don't have to include a huge bash script in
all of your workflow files that push Docker images.

Currently the auto-generation of tags works as follows:

- The image name is simply taken from the `GITHUB_REPOSITORY` that GitHub provides which is
  `<username>/<repo>`.
- If the ref-spec is a tag like `refs/tags/v1.0.0` then the `1.0.0` part is extracted, parsed as
  a semantic version and turned into 3 versions: `1.0.0`, `1.0` and `1`.
  - In case the version is a pre-release like `1.0.0-beta` then this version is taken as is and no
    further splitting into multiple versions is done.
  - Anything before the first number is stripped, not just `v`, so even `abc1.2.3` would be
    recognized as `1.2.3`.
- If the ref-spec is `refs/heads/main` or `refs/heads/master` the version is set to `latest`.
- In case the ref-spec doesn't match any of the above rules it's ignored and no tags are created.

## Inputs

Currently no imports are supported and the action works solely on the information it already gets
from the environment that Github provides.

Additional options may be added in the future to support other workflows. Please open a PRs if you
like this Github Action and need additional imports to make it fit to your projects.

## Outputs

### `tags`

The only output of this action is `tags` which is a comma separated list of image tags for Docker
that can be directly passed to the `docker/build-push-action@v2` action.

## Example usage

The usage is almost the same as in the `docker/build-push-action@v2` samples just that you change
the `Prepare` step to use this action instead.

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
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v1
      - name: Login to DockerHub
        uses: docker/login-action@v1
        with:
          username: ${{ secrets.DOCKERHUB_USERNAME }}
          password: ${{ secrets.DOCKERHUB_TOKEN }}
      - name: Build and push
        uses: docker/build-push-action@v2
        with:
          push: true
          tags: ${{ steps.docker_tags.outputs.tags }}
```

## License

This project is licensed under the [AGPL-3.0 License](LICENSE) (or
<https://www.gnu.org/licenses/agpl-3.0.html>).
