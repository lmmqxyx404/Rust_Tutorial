# Rust_Tutorial introduction
Write down my learning note for rustlang. The tutorial could be better for people who knows a little program language.
The most significant advantage for rust is safe.

# Before the code(Preparing for the dev env)
Download visual studio and visual studio code if your computer system is windows.
It could be a better choice to edit the config file in the directory named .cargo for a better experience.

# Features
## 1. control the code by oneself and control each variable and so on.


# The project is temporarily composed of three parts
## 1. easy
### 0.hello
The first taste for rust.

### 1.variable
Add all kinds of variable and tried to convert them.

### 2.loop
The significant difference is the label.

## 2. medium


## 3. hard
### concurrenccy
Introduce some advanced traits in rust besides concurrency.

# about grpc_rust
## requisition
cmake tools could not be omitted.

## can import remote package

# about tokio
## When not to use use tokio
  - Reading a lot of files. Operating system generally doesn't provide asynchronous file APIs.
  - Just sending a single web request
  - Speeding up CPU-bound computation

## common c/s programm routes
### server 
use bind() to bind a specific port.

### client
use connect() to connect a remote socket.
