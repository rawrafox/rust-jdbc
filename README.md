# jdbc #

A Rust library that allows you to use JDBC and JDBC drivers.

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
jdbc = "0.1"
```

Next, add this to your crate root:

```rust
extern crate jdbc;
```

Then you can in your main function do something like this:

```rust
let jvm_options = ["-Djava.class.path=./jars/postgresql-42.1.1.jar", "-Xcheck:jni"];
let _ = unsafe { jvm::JVM::from_options(&jvm_options) };

let url = "jdbc:postgresql://localhost/test";

let connection = DriverManager::get_connection(url).unwrap();
let statement = connection.create_statement().unwrap();
let result_set = statement.execute_query("SELECT * FROM customers").unwrap();
let metadata = result_set.get_meta_data().unwrap();
let columns = metadata.get_column_count().unwrap();

while result_set.next().unwrap() {
  print!("row:");
  for i in 1 .. columns + 1 {
    print!(" {:?}", result_set.get_string(i).unwrap());
  }
  println!("");
}
```

## What is jdbc? ##

The primary purpose of this crate is to allow you to use Java JDBC database
drivers from Rust in a convenient way with a relatively safe interface. It does
this via embedding a JVM in your process so be aware that if you use JNI in some
other way in your app you will need to tell jdbc to play nice.

## Platforms ##

I am testing on my Mac, but I would love to setup some CI to get this party
started.

## Contributing ##

Patches are welcome, don't forget to add yourself to the Authors list.

## Authors ##

 - Aurora <@rawrasaur, aurora@aventine.se>
