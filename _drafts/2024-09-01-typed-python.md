---
layout: post
title: Typed Python
tags: lessons-learned
category: 
---

big django code base
lots of unit tests
refactor is hard because of too many unit tests
refactor is hard because tests do not cover everything
over time, code becomes hard to read and developing new features become more expensive
we dont write tests for small tweaks and we send simple (typo) bugs to production

solution
typed python
code is easier to read
no more simple bugs in production. can ship things faster
unit tests become less important
more integration tests
refactor becomes easier
we can refactor with confidence

downside
type safe is good
but typed python is a frankenstein
typed languages (rust, haskell) may be more complex than python. typed python is more complex the natively typed languages.
this complexity can be reduced choosing typing compatible libraries

result
management is happy, because we can deliver more features with less bugs


