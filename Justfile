major := "0"
minor := major + ".2"
patch := minor + ".0"
image := "ghcr.io/dnaka91/action-docker-tags"

# list available recipes
default:
    @just --list -u

# update Git tags
tag:
    git tag -s v{{major}} -m "Version {{patch}}" -f
    git tag -s v{{minor}} -m "Version {{patch}}" -f
    git tag -s v{{patch}} -m "Version {{patch}}" -f

# build and tag the Docker image
build:
    docker build -t {{image}}:latest -t {{image}}:{{major}} -t {{image}}:{{minor}} -t {{image}}:{{patch}} .

# publish Git tags and Docker image
publish: tag build
    git push --tags -f
    docker push {{image}}:latest
    docker push {{image}}:{{major}}
    docker push {{image}}:{{minor}}
    docker push {{image}}:{{patch}}
