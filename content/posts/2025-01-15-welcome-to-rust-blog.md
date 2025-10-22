---
title: "Welcome to Rust Blog"
date: 2025-01-15T12:00:00Z
draft: false
tags: ["rust", "blogging", "web"]
---

# Welcome to Rust Blog!

This is a blazingly fast blog powered by **Rust** and **Axum**. It features:

- Server-side rendering with Tera templates
- Syntax highlighting for code blocks
- RSS feed generation
- Client-side search functionality
- Docker deployment to Google Cloud Run

## Code Example

Here's some Rust code with syntax highlighting:

```rust
fn main() {
    println!("Hello, world!");

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();

    println!("Sum: {}", sum);
}
```

## Features

- **Fast**: Built with Rust for maximum performance
- **Simple**: Markdown-based content management
- **Flexible**: Easy to theme and customize
- **Modern**: Uses Axum, the modern async web framework

## Getting Started

To create a new post, use:

```bash
just new-post "My Post Title"
```

To run the server locally:

```bash
just run
```

That's it! Happy blogging!
