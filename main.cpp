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
	std::string engineVersionName = argv[1];

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

	EngineInstall install;
	bool bFoundInstall = false;

	for (auto &_install : engineInstalls)
	{
		// Is this the engine verson the user is reffering to?
		if (_install.name == engineVersionName)
		{
			// Found installation - set it up for later processing
			install = _install;
			bFoundInstall = true;
			break;
		}
	}

	if (!bFoundInstall)
	{
		// Log an error and give a list of installations to choose from.
		std::cout << "Unable to find an Unreal Engine installation by the name of " << engineVersionName << "! Please choose one of the following:" << std::endl;
		for (auto &_install : engineInstalls)
		{
			std::cout << "- " << _install.name << " (" << _install.path << ")" << std::endl;
		}

		return -4;
	}

	// We've found the engine path, now create the path to the UATTool bat file.
	std::string UATToolPath = install.path + "/Engine/Build/BatchFiles/RunUAT.bat";

	// Build the UAT command based on the path we found and the arguments passed in after the engine name.
	std::stringstream UATCommandStream;

	// Wrap the path in "s so the system does not get confused by whitespaces/etc. NOTE: The command also has to be wrapped in quotation marks: https://stackoverflow.com/a/55652942
	UATCommandStream << "\"" << "\"" << UATToolPath << "\" ";

	// Use GetCommandLine to get the command used for this (that way the quotation marks are preserved), and strip the command name + first argument; based on: https://stackoverflow.com/a/14150434
	LPTSTR cmd = GetCommandLine();
	int l = strlen(argv[0]);// + strlen(" ") + strlen(argv[1]);
	std::string argv0WithQuotes = "\"" + std::string(argv[0]) + "\"";
	if (cmd == strstr(cmd, argv[0]))
	{
		cmd = cmd + l;
		while (*cmd && isspace(*cmd))
			++cmd;
	}
	// In case it is wrapped with quotes
	else if (cmd == strstr(cmd, argv0WithQuotes.c_str()))
	{
		cmd = cmd + l + 2;
		while (*cmd && isspace(*cmd))
			++cmd;
	}
	// Remove the second argument / the name one
	l = strlen(argv[1]);
	if (cmd == strstr(cmd, argv[1]))
	{
		cmd = cmd + l;
		while (*cmd && isspace(*cmd))
			++cmd;
	}

	std::string UATArguments = cmd;


	// Copy over the remaining arguments from the input stream (program name + 1 parameter for UATTool means that all the arguments after index 1 should be added -> start at i=2). Also add a closing quotation mark.
	UATCommandStream << UATArguments << "\"";

	std::string UATCommand = UATCommandStream.str();

	// Print the command in blue bright letters to help distinguish it from the rest, and then reset the color. Such a bummer escape sequences aren't properly supported on Win.
	HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
	CONSOLE_SCREEN_BUFFER_INFO Info;
	GetConsoleScreenBufferInfo(hConsole, &Info);
	SetConsoleTextAttribute(hConsole, 10);
	std::cout << "Running UAT command: " << UATCommand << std::endl << std::endl;
	SetConsoleTextAttribute(hConsole, Info.wAttributes);

	// Run the command and return whatever the bat file decides to return.
	return system(UATCommand.c_str());
}