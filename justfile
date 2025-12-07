benchmark:
  RUSTFLAGS='-C target-cpu=native' cargo criterion

readme:
	cargo readme > README.md
