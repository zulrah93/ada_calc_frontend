    git clone https://github.com/zulrah93/ada_calc_frontend.git
    cd ada_calc_frontend
    cargo build --release
    echo "Installing ada_calc_frontend in sbin!"
    cp /target/release/ada_calc_frontend /sbin/ada_calc_frontend
    echo "Restart terminal console for the changes to take effect!"