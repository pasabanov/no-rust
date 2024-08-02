################################################################################
#                                                                              #
#  Makefile for Rust version of "no" program                                   #
#                                                                              #
#  The "no" program serves as the opposite of the "yes" program, continuously  #
#  outputting "n" or all arguments passed to it separated by spaces.           #
#                                                                              #
#  Author: Petr Alexandrovich Sabanov                                          #
#  Date: 2024-08-02                                                            #
#                                                                              #
#  License: GNU General Public License v3.0                                    #
#  See: https://www.gnu.org/licenses/gpl-3.0.html                              #
#                                                                              #
################################################################################

.PHONY: all \
 		build \
 			build-dev build-debug build-release build-relwithdebinfo build-minsize \
 		run-dev run-debug run-release run-relwithdebinfo run-minsize \
 		dev debug release relwithdebinfo minsize \
 		clean \
 			clean-dev clean-debug clean-release clean-relwithdebinfo clean-minsizerel

all: build

build: build-dev build-debug build-release build-relwithdebinfo build-minsize

build-dev: build-debug
build-debug:
	cargo build --package no-rust --bin no-rust --profile dev
build-release:
	cargo build --package no-rust --bin no-rust --profile release
build-relwithdebinfo:
	cargo build --package no-rust --bin no-rust --profile relwithdebinfo
build-minsize:
	cargo build --package no-rust --bin no-rust --profile minsize

run-dev: run-debug
run-debug: build-debug
	./target/debug/no-rust
run-release: build-release
	./target/release/no-rust
run-relwithdebinfo: build-relwithdebinfo
	./target/relwithdebinfo/no-rust
run-minsize: build-minsize
	./target/minsize/no-rust

dev: debug
debug: run-debug
release: run-release
relwithdebinfo: run-relwithdebinfo
minsize: run-minsize

clean: clean-dev clean-debug clean-release clean-relwithdebinfo clean-minsize

clean-dev: clean-debug
	rm -rf "./target/"
clean-debug:
	rm -rf "./target/debug/"
clean-release:
	rm -rf "./target/release/"
clean-relwithdebinfo:
	rm -rf "./target/relwithdebinfo/"
clean-minsize:
	rm -rf "./target/minsize/"