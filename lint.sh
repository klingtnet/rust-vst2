#!/bin/bash

hash rustup &>/dev/null
[[ $? -ne 0 ]]\
    && echo 'Please install rustup'\
    && exit 2:

rustup run nightly -- which cargo-clippy &>/dev/null
[[ $? -ne 0 ]]\
	&& rustup run nightly -- cargo install clippy\
    && exit 1

rustup run nightly -- cargo clippy
