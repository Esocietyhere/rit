---@diagnostic disable: undefined-global
local libdir = ...
libdir = libdir:match("[^/]+")

local CONFIG_PATH = ("%s/%s"):format(libdir, "config.json")
local NO_PLACES_ERROR = "Config does not have any places"

local DEFAULT_PROJECT_FILE = "default.project.json"
local DEFAULT_PROJECT_CONTENT = {
	["name"] = "Untitled Project",
	["tree"] = {
		["$className"] = "DataModel",
		["Lighting"] = {},
		["Workspace"] = {
			["$className"] = "Workspace",
			["$ignoreUnknownInstances"] = true,
		},
	},
}

local function isFile(path)
	local success, result = pcall(remodel.isFile, path)
	return success and result
end

local function jsonDecode(str)
	local success, result = pcall(json.fromString, str)
	if success then
		return result
	end
end

local maps = isFile(CONFIG_PATH) and jsonDecode(remodel.readFile(CONFIG_PATH)) or {}

function maps.getProjectFileName(name)
	return ("%s.project.json"):format(name)
end

function maps.removeUnwantedItems(root)
	local trash = {}

	print(("Scanning %s for unwanted instances..."):format(root.Name))

	for _, instance in ipairs(root:GetDescendants()) do -- remodel does not support roblox's :IsA method yet
		if
			instance.ClassName == "Script"
			or instance.ClassName == "LocalScript"
			or instance.ClassName == "ModuleScript"
			or instance.ClassName == "CoreScript" -- this may never happen but better safe than sorry
			or instance.ClassName == "Camera" -- very annoying when rojo jerks your camera around
		then
			table.insert(trash, instance)
			print(("Marked %s '%s' for deletion"):format(instance.ClassName, instance.Name))
		end
	end

	if #trash > 0 then
		-- for some reason remodel can destroy things in the future?
		-- i have no fucking clue but i have to do it this way
		for _, instance in ipairs(trash) do
			pcall(instance.Destroy, instance)
		end
	end

	return root
end

function maps:_getGame(options)
	self.usedNames = {}

	if options.name then
		assert(self.import.places, NO_PLACES_ERROR)

		local placeId = self.import.places[options.name]
		if placeId then
			return remodel.readPlaceAsset(placeId)
		end
	end

	if options.id then
		return remodel.readPlaceAsset(options.id)
	end

	if options.path then
		return remodel.readPlaceFile(options.path)
	end

	error("No valid options passed to maps:_getGame")
end

function maps:loadPlace(options)
	return self.removeUnwantedItems(self:_getGame(options))
end

function maps:writeModelFile(model, path)
	local sanitizedName = model.Name:gsub("[^%w_]", "")

	if self.usedNames[sanitizedName] then
		self.usedNames[sanitizedName] = self.usedNames[sanitizedName] + 1
		sanitizedName = ("%s%d"):format(sanitizedName, tostring(self.usedNames[sanitizedName]))
	else
		self.usedNames[sanitizedName] = 0
	end

	local filePath = ("%s/%s.rbxm"):format(path, sanitizedName)

	print(("Importing %s"):format(filePath))

	remodel.createDirAll(path)
	remodel.writeModelFile(model, filePath)
end

function maps:writeModelsIn(root, curDir)
	for _, instance in ipairs(root:GetChildren()) do
		if instance.ClassName == "Folder" then
			self:writeModelsIn(instance, ("%s/%s"):format(curDir, instance.Name))
		else
			self:writeModelFile(instance, curDir)
		end
	end
end

function maps:_importGameAssets(options)
	local game = self:loadPlace(options)
	local Assets = game.ReplicatedStorage:FindFirstChild("Assets")

	if Assets then
		local models = Assets:FindFirstChild("models")

		if models then
			self:writeModelsIn(models, "assets/models")
		else
			print("No models folder in Assets folder")
		end
	else
		print("No Assets folder found")
	end
end

function maps:importAssets()
	assert(type(self.import.assetsPlaceId) == "number", "Config does not have a valid assetsPlaceId")
	self:_importGameAssets({ id = self.import.assetsPlaceId })
end

function maps:importLocalAssets(path)
	self:_importGameAssets({ path = path })
end

function maps.getMapDir(name)
	return ("maps/%s"):format(name)
end

local function deserializeProjectFile(projectFile)
	local project = jsonDecode(remodel.readFile(projectFile))

	project.tree = project.tree or DEFAULT_PROJECT_CONTENT.tree
	project.tree["$className"] = project.tree["$className"] or "DataModel"
	project.tree.Workspace = project.tree.Workspace or {}
	project.tree.Lighting = project.tree.Lighting or {}

	return project
end

function maps:_defaultProjectFileContent(name)
	local projectFileName = self.getProjectFileName(name)

	if isFile(projectFileName) then
		return deserializeProjectFile(projectFileName)
	end

	if isFile(DEFAULT_PROJECT_FILE) then
		return deserializeProjectFile(DEFAULT_PROJECT_FILE)
	end

	return DEFAULT_PROJECT_CONTENT
end

function maps:generateProjectFile(name)
	local dir = self.getMapDir(name)
	local workspacePath = ("%s/Workspace"):format(dir)
	local lightingPath = ("%s/Lighting.rbxm"):format(dir)
	local projectFileName = self.getProjectFileName(name)
	local content = self:_defaultProjectFileContent(name)
	local changed = false

	content.name = name

	if content.tree.Workspace["$path"] ~= workspacePath then
		content.tree.Workspace["$path"] = workspacePath
		changed = true
	end

	if content.tree.Lighting["$path"] ~= lightingPath then
		content.tree.Lighting["$path"] = lightingPath
		changed = true
	end

	if changed then
		print(("Updated %s"):format(projectFileName))
		remodel.writeFile(projectFileName, json.toStringPretty(content))
	end
end

function maps:refreshProjectFile(name)
	local projectFileName = self.getProjectFileName(name)
	pcall(os.execute, ("rm '%s'"):format(projectFileName))
	self:generateProjectFile(name)
end

function maps:_importGameMap(game, name)
	assert(name and name ~= "", "name must not be nil or empty")

	local dir = self.getMapDir(name)

	if pcall(remodel.isDir, dir) then
		-- we want a completely new copy of the imported map
		print(("Deleting old '%s' directory..."):format(name))
		os.execute(("rm -rd '%s'"):format(dir))
	end

	self:writeModelFile(game.Lighting, dir)

	local workspaceDir = ("%s/Workspace"):format(dir)
	for _, instance in ipairs(game.Workspace:GetChildren()) do
		self:writeModelFile(instance, workspaceDir)
	end

	self:generateProjectFile(name)
end

function maps:importLocalMap(path, name)
	if name == "" then
		name = nil
	end

	self:_importGameMap(self:loadPlace({ path = path }), name or "file_import")
end

function maps:importMap(name)
	self:_importGameMap(self:loadPlace({ name = name }), name)
end

function maps:importAllMaps()
	assert(self.import.places, NO_PLACES_ERROR)
	for name in pairs(self.import.places) do
		self:importMap(name)
	end
end

return maps:{{method}}({{args}})
