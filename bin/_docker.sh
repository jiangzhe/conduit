#!/bin/sh
#
# docker
#

set -eu

. bin/_log.sh

# TODO this should be set to the canonical public docker regsitry; we can override this
# docker regsistry in, for instance, CI.
export DOCKER_REGISTRY="${DOCKER_REGISTRY:-gcr.io/runconduit}"

# When set, causes docker's build output to be emitted to stderr.
export DOCKER_TRACE="${DOCKER_TRACE:-}"

docker_repo() {
    repo="$1"

    name="$repo"
    if [ -n "${DOCKER_REGISTRY:-}" ]; then
        name="$DOCKER_REGISTRY/$name"
    fi

    echo "$name"
}

docker_tags() {
    image="$1"
    docker image ls "${image}" | sed 1d | awk '{print $2}'
}

docker_build() {
    repo=$(docker_repo "$1")
    shift

    tag="$1"
    shift

    file="$1"
    shift

    extra="$@"

    output="/dev/null"
    if [ -n "$DOCKER_TRACE" ]; then
        output="/dev/stderr"
    fi

    log_debug "  :; docker build . -t $repo:$tag -f $file $extra"
    docker build . \
        -t "$repo:$tag" \
        -f "$file" \
        $extra \
        > "$output"

    echo "$repo:$tag"
}

docker_pull() {
    repo=$(docker_repo "$1")
    tag="$2"
    log_debug "  :; docker pull $repo:$tag"
    docker pull "$repo:$tag"
}

docker_push() {
    repo=$(docker_repo "$1")
    tag="$2"
    log_debug "  :; docker push $repo:$tag"
    docker push "$repo:$tag"
}

docker_retag() {
    repo=$(docker_repo "$1")
    from="$2"
    to="$3"
    log_debug "  :; docker tag $repo:$from $repo:$to"
    docker tag "$repo:$from" "$repo:$to"
    echo "$repo:$to"
}
