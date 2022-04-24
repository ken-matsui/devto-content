#!/bin/bash

err () {
    echo >&2 "$@"
    exit 1
}

check_cmd() {
    command -v "$1" >/dev/null 2>&1
    return $?
}

need_cmd() {
    if ! check_cmd "$1"; then
        err "need '$1' (command not found)"
    fi
}

[ "$#" -eq 1 ] || err "1 argument required, $# provided"
: ${DEVTO_TOKEN:?"Need to export DEVTO_TOKEN"}

need_cmd mkdir
need_cmd cat
need_cmd curl
need_cmd jq

DIR=blog-posts/$1
FILE=blog-posts/$1/$1.md

[ -d "$DIR" ] && err "Directory '$DIR' exists."
[ -f "$FILE" ] && err "File '$FILE' exists."

# Save as draft
POST_ID=$(curl -s -X POST -H 'Content-Type: application/json' -H "api-key: $DEVTO_TOKEN" -d '{"article":{"title":"'"$1"'","body_markdown":"","published":false,"tags":[]}}' https://dev.to/api/articles | jq '.id')
: ${POST_ID:?"Could not create post"}
[ "$POST_ID" = "null" ] && err "Could not create post"
echo "Created post with id: $POST_ID"

# Update `dev-to-git.json`
jq ". += [{\"id\": $POST_ID, \"relativePathToArticle\": \"./$FILE\"}]" dev-to-git.json > dev-to-git.json.tmp
rm dev-to-git.json
mv dev-to-git.json.tmp dev-to-git.json

# Generate template file
mkdir -p $DIR
cat <<EOF > $FILE
---
title: '$1'
published: false
description: ''
tags:
---
EOF
