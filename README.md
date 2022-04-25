# DEV blog source

https://dev.to/ken_matsui

# Usage

## Configuration

First, you need to install [`devto-cli`](https://github.com/ken-matsui/devto-cli) to manage articles.

```bash
$ cargo install devto-cli
```

You may need to export your dev.to token when using some scripts.

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

# Markdown Syntax

* https://github.com/adam-p/markdown-here/wiki/Markdown-Here-Cheatsheet
* https://github.com/maxime1992/dev.to#how-do-i-add-images-to-my-blog-posts
* `{% embed https://... %}`

# Note

* Committed new revisions, after being published, will be automatically applied by [`dev-to-git`](https://www.npmjs.com/package/dev-to-git)
