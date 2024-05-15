# Generated by rust2rpm 26
%bcond_without check

%global ver ###
%global commit ###
%global date ###

# prevent library files from being installed
%global cargo_install_lib 0

Name:           portapak
Version:        %{ver}~git%{date}.%{sub %{commit} 1 7}
Release:        %autorelease
Summary:        Run flatpak applications without installation

SourceLicense:  MIT
# (MIT OR Apache-2.0) AND Unicode-DFS-2016
# Apache-2.0
# Apache-2.0 OR MIT
# MIT
# MIT OR Apache-2.0
# Unlicense OR MIT
License:        (MIT OR Apache-2.0) AND Unicode-DFS-2016 AND Apache-2.0 AND Apache-2.0 OR MIT AND MIT AND MIT OR Apache-2.0 AND Unlicense OR MIT
# LICENSE.dependencies contains a full license breakdown

URL:            https://github.com/ryanabx/portapak

# To create this source:
# * git clone the repository
# * tar -pcJf $name-$commit.tar.xz
Source:         %{name}-%{commit}.tar.xz

BuildRequires:  cargo-rpm-macros >= 26
BuildRequires:  rustc
BuildRequires:  cargo
BuildRequires:  flatpak-devel

Requires:       flatpak

%global _description %{expand:
%{summary}.}

%description %{_description}

%prep
%autosetup -n %{name}-%{commit} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
%cargo_install
install -Dm0644 data/io.github.ryanabx.portapak.desktop %{buildroot}/%{_datadir}/applications/io.github.ryanabx.portapak.desktop
install -Dm0644 data/io.github.ryanabx.portapak.svg %{buildroot}/%{_datadir}/icons/hicolor/scalable/apps/io.github.ryanabx.portapak.svg

%if %{with check}
%check
%cargo_test
%endif

%files
%license LICENSE
%license LICENSE.dependencies
%doc README.md
%{_bindir}/portapak
%{_datadir}/applications/io.github.ryanabx.portapak.desktop
%{_datadir}/icons/hicolor/scalable/apps/io.github.ryanabx.portapak.svg

%changelog
%autochangelog
