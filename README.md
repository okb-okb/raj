# raj

## About
raj (rust atcoder judge) is a wrapper of [oj](https://github.com/online-judge-tools/oj) specialized for [atcoder](https://atcoder.jp/).
It can make a directory for a contest, execute a local test, and make a submission.

We confirm that the tool works under the following environment.

- Ubuntu 20.04.1 LTS
- rustc 1.65.0 (897e37553 2022-11-02)
- cargo 1.65.0 (4bc8f24d3 2022-10-20)

## Tutorial
In this tutotiral, you will download and build raj, do local test, and submit a file.

```
$ git clone git@github.com:okb-okb/raj.git
$ cd raj
$ cargo build
$ cp target/debug/raj raj
$
$ ./raj make abc150
$ cp tutorial/abc150_a.rs abc150/abc150_a.rs
$ ./raj test abc150 a --file ./abc150/abc150_a.rs
$ ./raj submit abc150 a --file ./abc150/abc150_a.rs
```

## Usage
### `make`
The command to make a directory for a contest. By default, this command makes files named `<CONTEST>_[a-h].rs`.

```
Usage: raj make [OPTIONS] <CONTEST>

Arguments:
  <CONTEST>

Options:
  -n, --number <NUMBER>  The number of problems the contest has
  -h, --help             Print help information
```

Example:
```
raj make -n 6 abc150 # make files named abc150_[a-f].rs
```

### `test`
The command to run a local test for the designated problem. By default, the command make a test with a file `./<CONTEST>_<PROBLEM>.rs` and a problem `<CONTEST>_<PROBLEM>` (For example, `CONTEST = abc150` and `PROBLEM = a`).

```
Usage: raj test [OPTIONS] <CONTEST> <PROBLEM>

Arguments:
  <CONTEST>
  <PROBLEM>

Options:
  -f, --file <FILE>        File name to test
  -e, --error <TOLERANCE>  Tolerance of error for problems handle floating point number (1e-{TOLERANCE})
  -h, --help               Print help information
```

Example:
```
raj test abc150 a --file ./abc150/abc150_a.rs --error 5 # test if a file abc150_a.rs can pass test cases in ABC 150 prolem A with a tolerance 1e-5. NOTICE: this command does not work since this problem should output "Yes" or "No". 
```

### `submit`
The command to submit a file. By default, the command submit a file named `<CONTEST>_<PROBLEM>.rs`.

```
Usage: raj submit [OPTIONS] <CONTEST> <PROBLEM>

Arguments:
  <CONTEST>
  <PROBLEM>

Options:
  -f, --file <FILE>  File name to submit
  -h, --help         Print help information
```

Example
```
raj submit --file abc150_a.rs abc150 a # submit a file abc150_a.rs to ABC 150 problem A
```

### Environment variable
You can set environment variable to custom how raj works.

- `RAJ_EXTENSION` - an extension for files to make, test or submit (ex: `cpp`, `py`). (default: `rs`)
- `RAJ_TEMPLATE_FILE` - a template file to copy when you use `make` command. (default: `template.rs`)
- `RAJ_COMPILE` - a command before file name to compile your file. (default: `rustc -o a.out`)