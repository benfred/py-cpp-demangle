if [ ! -d ~/rust-installer ]; then 
    mkdir ~/rust-installer
    curl -sL https://static.rust-lang.org/rustup.sh -o ~/rust-installer/rustup.sh
    sh ~/rust-installer/rustup.sh --spec=nightly --disable-sudo -y
fi
