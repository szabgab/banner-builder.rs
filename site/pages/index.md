---
title: Banbu, the Banner Builder
timestamp: 2023-11-21T17:30:01
description: Generating images from a config file
---

This code was extracted from the [Code Maven SSG](https://ssg.code-maven.com/).

The goal is to create a library that will make it easy to create various simple banners for YouTube, Meetup, and also for the Code Maven SSG.

There is a way to use this as a command line tool:

## Examples

### Hello World

![](examples/hello_world.yaml)

```banbu site/examples/hello_world.yaml hello_world.png```

![](examples/hello_world.png)


### YouTube Thumbnail

![](examples/youtube_thumbnail_text_background.yaml)
![](examples/youtube_thumbnail_text_background.png)
