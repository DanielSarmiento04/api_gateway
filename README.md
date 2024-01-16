# *Api Gateway*

##  Resume

The objective of this repository is make simple api gateway using rust as management, ngnx as server proxy and some microserver bellow api gateway


## Table of code

1. [Introduction](#introduction)
2. [Setup](#setup)


### Introduction

A simple schema that the project try to follow is the next

<p align="center">
  <img src="docs/Diagrams.jpg" height ="300px">
</p>


Folder strcuture

- cars (javascript/bun)
- users (python/fastapi)
- rust_api (rust api gateway)
- ngnx (server proxy)


### SetUp

```
sudo openssl req -new -newkey rsa:2048 -nodes -keyout tudominio.key -out tudominio.csr
```

> Each component can build using docker, each folder can build separatly

> As suggestion, create a network and run the docker container over this network
 


docker-compose up -d --remove-orphans


## References



- Images
    - <a href="https://freepngimg.com/png/86146-icons-mobile-symbol-accessories-phone-computer-logo">Icons Mobile Symbol Accessories Phone Computer Logo from FreePNGImg.com</a>
    



