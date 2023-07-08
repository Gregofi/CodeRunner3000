# The unnamed monorepo
When I first bought a raspberry pi, I thought to myself, "I must run something
cool on this bad boy!". At my job, we were doing plenty of containerized
applications which run in k8. I was semi-experienced with it, but I never
bootstraped such project myself. Also, I was writing my own dynamic language as
a side project. This led me to an idea to create my own tiny [Compiler
Explorer](https://godbolt.org/) where I can include my own language, so that
everybody can try it from their browsers. To create this, I have to learn many
things which I did not pick up in my school years.

Some of the things are certainly not done optimaly, and sometimes wheels are
reinvented. For example, I certainly did not have to implement HTTP server
in Rust. But I wanted to take the opportunity to learn this language.

## The architecture
There are/will be multiple components:
- website: The website which is accesible to the users
- evaluator: The evaluator of the code that users write into the website.
