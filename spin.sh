#!/bin/bash
mkdir spin
for i in {000..500..20}
do
	cargo run $i > "spin/$i.ppm"
done
