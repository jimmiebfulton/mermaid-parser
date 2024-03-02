alias ut := test-ut
alias it := test-it
alias all := test-all

test-ut:
    cargo test --lib --bins

test-it:
    cargo test --test '*'
                                                  
test-all:
    cargo test

test TEST:
    cargo test --test {{ TEST }}

install-defaults := ''

install install-options=install-defaults:
    cargo install --path crates/example_service_bin/ {{ install-options }}