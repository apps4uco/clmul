benchmark:
  cargo criterion --features morton

readme:
	cargo readme > README.md
