DIAGRAMS_SRC := $(wildcard src/**/*.plantuml)
DIAGRAMS_SVG := $(addsuffix .svg, $(basename $(DIAGRAMS_SRC)))

svg: $(DIAGRAMS_SVG)

clean:
	rm -f $(DIAGRAMS_PNG) $(DIAGRAMS_SVG)

src/%.svg: src/%.plantuml
	# plantuml -tsvg $^

.PHONY: svg clean