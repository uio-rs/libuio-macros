.PHONY: devel
devel: build volumes
	docker run --rm -it --name libuio-macros-devel --net host \
        -v "nvim-macros-cache:/root/.cache/nvim" \
        -v "nvim-macros-state:/root/.local/state" \
        -v "nvim-macros-share:/root/.local/share" \
        -v "${HOME}/.config/nvim:/root/.config/nvim" \
        -v "${HOME}/.ssh:/root/.ssh:ro" \
        -v "${HOME}/.gnupg:/root/.gnupg:ro" \
        -v "${PWD}:/opt/libuio-macros" \
        -v "${HOME}/.zshrc:/root/.zshrc:ro" \
        -v "${HOME}/.zshenv:/root/.zshenv:ro" \
        -v "${HOME}/.p10k.zsh:/root/.p10k.zsh:ro" \
        -v "${HOME}/.cargo/credentials.toml:/root/.cargo/credentials.toml:ro" \
        --privileged \
        --entrypoint nvim \
        libuio-macros-devel:latest /opt/libuio-macros

.PHONY: build
build:
	 docker build -t libuio-macros-devel:latest -f dist/Dockerfile .

.PHONY: volumes
volumes:
	docker volume create nvim-macros-cache
	docker volume create nvim-macros-state
	docker volume create nvim-macros-share
