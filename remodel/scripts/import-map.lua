local dir, name = ...
load(remodel.readFile(("%s/lib.lua"):format(dir)))():importMap(name)