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

	std::cout <<  launcherInstalledJson.dump();

	return 10;
}