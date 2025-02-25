##### README #####
# Please put this Dockerfile in the root directory of the project.
#
# To build the docker image, run the following command:
# docker buildx build . -t micro:1
#
# Then run the docker image with the following command:
# docker run -p 2222:22 -it micro:1
# Then you can connect with 'ssh root@localhost -p 2222' with password 'docker'
#
# After login, input the testcase "test.m"
# vim test.m
#
# Then run the compiler to get "a.mlir", "ast.dot", and "cst.dot"
# ./microc test.m
#
# To generate png based on ast.dot.and cst.dot
# dot -Tpng ast.dot -o ast.png
# dot -Tpng cst.dot -o cst.png
#
# Then generate LLVM IR using MLIR file
# mlir-opt a.mlir --convert-func-to-llvm | mlir-translate --mlir-to-llvmir -o a.ll
#
# Then compile the LLVM IR to get the assembly code
# llc -march=riscv64 -mcpu=generic-rv64 -mattr=+d -filetype=asm a.ll -o a.s
#
# Then compile the assembly code to get the executable
# riscv64-linux-gnu-g++ -shared -fPIC -o util4mlir.so ./src/util4mlir.cpp
# riscv64-linux-gnu-gcc a.s util4mlir.so -o a
#
# Finally, run the executable
# LD_LIBRARY_PATH="/usr/riscv64-linux-gnu/lib:/root" ./a
##################

FROM buildpack-deps:bookworm

# Change mirrors
RUN echo "deb http://mirrors.sustech.edu.cn/debian bookworm main contrib non-free non-free-firmware" > /etc/apt/sources.list
RUN echo "deb http://mirrors.sustech.edu.cn/debian bookworm-updates main contrib non-free non-free-firmware" >> /etc/apt/sources.list
RUN echo "deb http://mirrors.sustech.edu.cn/debian-security bookworm-security main contrib non-free non-free-firmware" >> /etc/apt/sources.list

# Install and Configure SSH
RUN apt update
RUN apt install -y openssh-server
RUN echo 'root:docker' | chpasswd # set password to 'docker'
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN mkdir /var/run/sshd
RUN chmod 0755 /var/run/sshd
# Install vim nano graphviz
RUN apt install -y vim nano graphviz

# Install LLVM
RUN apt install -y lsb-release wget software-properties-common gnupg
RUN wget https://apt.llvm.org/llvm.sh
RUN chmod +x llvm.sh
RUN ./llvm.sh 18 all
RUN apt install -y libmlir-18-dev mlir-18-tools
# Install QEMU
RUN apt install -y qemu-user-static
# Install gcc
RUN apt install -y gcc-riscv64-linux-gnu g++-riscv64-linux-gnu
RUN cp /usr/riscv64-linux-gnu/lib/ld-linux-riscv64-lp64d.so.1 /lib/ld-linux-riscv64-lp64d.so.1
# Install Rust
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static \
    RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.85.0
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $RUST_VERSION

# Add PATH
RUN echo "export PATH=$PATH:/usr/lib/llvm-18/bin:/usr/bin" >> /root/.bash_profile

WORKDIR /root
COPY . .

RUN cargo build
RUN cp ./target/debug/microc ./microc

EXPOSE 22

# Start SSH server
CMD ["/usr/sbin/sshd", "-D"]
