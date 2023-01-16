FROM fedora:latest
LABEL maintainer="Dmitrii Gusev<amidg4x4dima@gmail.com>"

# improve dnf performance
RUN echo "skip_if_unavailable=True" >> /etc/dnf/dnf.conf && \
    echo "fastestmirror=True" >> /etc/dnf/dnf.conf && \
    echo "max_parallel_downloads=10" >> /etc/dnf/dnf.conf && \
    echo "defaultyes=True" >> /etc/dnf/dnf.conf && \
    echo "keepcache=True" >> /etc/dnf/dnf.conf && \
    dnf autoremove -y && dnf clean all -y

# upgrade all latest
RUN dnf upgrade --refresh -y

# set RPM Fusion
RUN sudo dnf install \
  https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm -y && \
  sudo dnf install \
  https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm -y

# update and install missing tools
RUN dnf update -y && dnf upgrade -y && \
    dnf install -y git clang curl wget ffmpeg nasm pkgconf yasm unzip && \
    dnf swap --enablerepo=rpmfusion-free-updates-testing mesa-va-drivers mesa-va-drivers-freeworld && \
    dnf swap --enablerepo=rpmfusion-free-updates-testing mesa-vdpau-drivers mesa-vdpau-drivers-freeworld

# create new user
RUN useradd --create-home --home-dir /home/alvr --shell /bin/bash --user-group --groups adm,wheel alvr && \
    echo alvr:alvr | chpasswd && \
    echo "alvr ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# create a workdir for ALVR build
WORKDIR /home/alvr/build
RUN git clone https://github.com/alvr-org/ALVR.git -b v19.0.0

# build
RUN su - alvr -c "./build/ALVR/packaging/alvr_build_linux.sh all"