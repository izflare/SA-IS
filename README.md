# SA-IS

### Description

This code is an implementation of a linear time/space suffix array construction algorithm, called SA-IS.
This algorithm is proposed in the following paper:

> Ge Nong, Sen Zhang, Wai Hong Chan: _Linear Suffix Array Construction by Almost Pure Induced-Sorting._ DCC 2009: 193-202.

### Download

To clone the repository, call

```
git clone https://github.com/izflare/SA-IS.git
```

### Compile

This code has been tested under linux compiling with rust (cargo) ver 1:1.30.1-1  
After download the repository, 

```
cd SA-IS
cargo build --release
```

### Run

After compiling,

```
cd target/release
./sais --input <input>
```

then the tool run. `<input>` is your input text data file.  
Elapsed time for running (just during constructing suffix array) and constructed suffix array will be displayed.

