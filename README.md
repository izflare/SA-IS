# SA-IS

### Description

### Download

To clone the repository, call

```
git clone https://github.com/izflare/sais.git
```

### Compile

This code has been tested under linux compileing with rust (cargo) ver 1:1.30.1-1
After download the repository, 

```
cd sais
cargo build --release
```

### Run

After compileing,

```
cd target/release
./sais <filename>
```

then the tool run. <filename> is your input text data file.
Elapsed time for running (just during constructing suffix array) and constructed suffix array will be displayed.

