default:
    just --list

build:
    cargo build --release --locked

install: build
    install -Dm0755 target/release/flatrun /usr/bin/flatrun
    install -Dm0644 data/org.ryanabx.flatrun.desktop /usr/share/applications/org.ryanabx.flatrun.desktop
    install -Dm0644 data/org.ryanabx.flatrun.metainfo.xml /usr/share/metainfo/org.ryanabx.flatrun.metainfo.xml
    install -Dm0644 logo.svg /usr/share/icons/hicolor/scalable/apps/org.ryanabx.flatrun.svg

install-user: build
    install -Dm0755 target/release/flatrun $HOME/.local/bin/flatrun
    install -Dm0644 data/org.ryanabx.flatrun.desktop $HOME/.local/share/applications/org.ryanabx.flatrun.desktop
    install -Dm0644 data/org.ryanabx.flatrun.metainfo.xml $HOME/.local/share/metainfo/org.ryanabx.flatrun.metainfo.xml
    install -Dm0644 logo.svg $HOME/.local/share/icons/hicolor/scalable/apps/org.ryanabx.flatrun.svg

uninstall:
    rm -f /usr/share/icons/hicolor/scalable/apps/org.ryanabx.flatrun.svg
    rm -f /usr/share/applications/org.ryanabx.flatrun.desktop
    rm -f /usr/share/metainfo/org.ryanabx.flatrun.metainfo.xml
    rm -f /usr/bin/flatrun

uninstall-user:
    rm -f $HOME/.local/share/icons/hicolor/scalable/apps/org.ryanabx.flatrun.svg
    rm -f $HOME/.local/share/applications/org.ryanabx.flatrun.desktop
    rm -f $HOME/.local/share/metainfo/org.ryanabx.flatrun.metainfo.xml
    rm -f $HOME/.local/bin/flatrun