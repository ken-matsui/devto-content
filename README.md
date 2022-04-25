# DEV blog source

https://dev.to/ken_matsui

# Usage

## Configuration

First, you need to install [`devto-cli`](https://github.com/ken-matsui/devto-cli) to manage articles.

```bash
$ cargo install devto-cli
```

Next, export your dev.to token when using some scripts.

```bash
$ export DEVTO_TOKEN=your_token
```

Or you can pass the token as an option:

```bash
$ devto new your-new-article-title --devto-token your_token
```

## Create a new article

```bash
$ devto new your-new-article-title
```

## Publish an article

> You should [set secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets#creating-encrypted-secrets-for-a-repository) on your GitHub repository

1. Change the `publish` to true in front matter
2. Push to your GitHub repository

# Markdown Syntax

* https://dev.to/p/editor_guide
* https://github.com/adam-p/markdown-here/wiki/Markdown-Here-Cheatsheet
* https://github.com/maxime1992/dev.to#how-do-i-add-images-to-my-blog-posts
* `{% embed https://... %}`

# Note

* Committed new revisions, after being published, will be automatically applied by [`dev-to-git`](https://www.npmjs.com/package/dev-to-git)
