#include <nlohmann/json.hpp>
#include <iostream>
#include <string>
#include <sstream>
#include <fstream>

#include <windows.h>
#include <KnownFolders.h>
#include <shlobj.h>

// Abbreviate nlohmann::json to json
using json = nlohmann::json;

// Simple data structure to hold the name/path of an engine; will become more important when we allow the user to add custom paths (to source builds).
struct EngineInstall
{
	std::string name;
	std::string path;
};

int main(int argc, char *argv[])
{
	// We need at least 2 arguments (command name & the version to use)
	if (argc < 2)
	{
		std::cout << "Usage: ./uattool <engine_version> <UAT Command>" << std::endl;
		return -1;
	}

	// Get the engine version name (first argument after the app name -> second index -> 1)
	std::string engineVersion = argv[1];

	// Build the path to the LauncherInstalled.dat file -> this will contain a list of all installed engine versions + their paths.
	wchar_t* wpath = NULL;//new wchar_t[256];
	SHGetKnownFolderPath(FOLDERID_ProgramData, 0, NULL, &wpath);
	char path[256];
	wcstombs(path, wpath, 256);
	std::stringstream launcherInstalledPathStream;
	launcherInstalledPathStream << path << "/Epic/UnrealEngineLauncher/LauncherInstalled.dat";
	std::string launcherInstalledPath = launcherInstalledPathStream.str();
	
	// (Attempt to) open the path
	std::ifstream ifs(launcherInstalledPath);
	if (!ifs.is_open())
	{
		std::cout << "Unable to locate & open LauncherInstalled.dat! Please consider opening up an issue and performing a manual search using your File Manager to locate the proper path (include this in the issue)!" << std::endl;
		
		// Log the LauncherInstalled path
		std::cout << "Using LauncherInstalled path: " << launcherInstalledPath << std::endl;
		return -2;
	}


	// Attempt to parse the file.
	json launcherInstalledJson;
	try {
		ifs >> launcherInstalledJson;
	}
	catch (json::parse_error) {

		std::cout << "Unable to parse LauncherInstalled.dat! Please consider opening up an issue and attach the following file:" << std::endl;
		std::cout << "Using LauncherInstalled path: " << launcherInstalledPath << std::endl;
		return -3;
	}

	// NOTE: Though this doesn't really have any use at this point in time (an if would work just fine too), we will likely use this in the future when we allow the user to save custom names/paths.
	std::vector<EngineInstall> engineInstalls;

	json installationListJson = launcherInstalledJson["InstallationList"];
	for (json::iterator it = installationListJson.begin(); it != installationListJson.end(); ++it) {
		// Attempt to parse this item, and if there was an error, skip the item.
		try {
			std::string appName = (*it)["AppName"].get<std::string>();
			std::string appPath = (*it)["InstallLocation"].get<std::string>();

			std::string appNamePrefix = "UE_";

			// Ensure the prefix is in the name (this is the way we identify (luancher) builds over plugins & the like)
			size_t prefix_position = appName.find(appNamePrefix);
			if (prefix_position == std::string::npos)
			{
				continue;
			}

			// Erase the prefix from the string so we can compare it.
			appName.erase(prefix_position, appNamePrefix.length());

			// Add this entry to the list of installs.
			EngineInstall install;
			install.name = appName;
			install.path = appPath;
			engineInstalls.push_back(install);

		}
		catch (...)
		{
			continue;
		}
	}

	std::cout << engineInstalls.size() << " Unreal Engine installs found!" << std::endl;

	return 10;
}