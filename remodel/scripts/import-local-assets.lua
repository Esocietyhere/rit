local dir, path = ...
load(remodel.readFile(("%s/lib.lua"):format(dir)))():importLocalAssets(path)
