# UAT-Tool
UAT-Tool helps you access the Unreal Build Tool more easily: copying paths is now a thing of the past!

![UATTool Screenshot](https://i.imgur.com/96xHZ07.png)

UATTool will automatically detect launcher builds of the engine, and substitute the RunUAT.bat path into the command for you.


# How do I use it?
UATTool is currently still an experiment that we use internally here at HowToCompute. We may expand upon it's functionality in the future to allow compatibility with source builds and to improve it's user-friendlyness.

Once you have installed UATTool, start a new command prompt and run ```uattool```. This will show you the command's syntax. This can also be found in the snipet below:
```
Usage: ./uattool <engine_version> <UAT Command>
```

To list the engine versions that UATTool has detected on your system, run ```uattool 4```. This will list all of the available engines with their respective names and paths:
```
...
- 4.21 (D:\Program Files\Epic Games\UE_4.21)
- 4.22 (D:\Program Files\Epic Games\UE_4.22)
...
```

To run a UAT command, such as the `BuildPlugin` command, simply run uattool with the name of the version you'd like to use, and the rest of the UAT command you'd like to run. An example that we use internally can be found below:
```
uattool 4.24 BuildPlugin -Package="D:/PluginBuilds/TwitchWorks" -Plugin="D:\Unreal Projects\TwitchworksProject\Plugins\TwitchWorks\TwitchWorks.uplugin" 
```

# How do I install it?
The easiest way to install UATTool is opening up a powershell prompt, and running the following line of code:
```
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/How2Compute/UAT-Tool/master/install_uattool.ps1'))
```
This will download and run the installation script.

Alternatively, you can download the installation scipt manually and run it using powershell. You can download the latest version of the powershell script [here](https://raw.githubusercontent.com/How2Compute/UAT-Tool/master/install_uattool.ps1).

Lastly, you can download the latest [release](https://github.com/How2Compute/UAT-Tool/releases), extract it, and add the directory that contains the `uattool.exe` file to your system's path yourself. We will unfortunately not be able to cover this within the scope of this README. 

# I think I've found a bug! What do I do?
Please consider opening up a GitHub Issue so we can investigate your issue. If possible, include the steps you took to hit the issue, the command you ran as well as a screenshot of the error.

# Compiling UATTool
UATTool's project files are generated using CMake. Please ensure that you have both CMake and Visual Studio installed. Please also ensure that you have the [nlohmann JSON library](https://github.com/nlohmann/json) installed.

To generate the Visual Studio project files for UATTool, open up the command prompt to where you downloaded this GitHub repository, and run the following command:
```
cmake .
```
This will generate the visual studio project files that will allow you to compile the project. To produce the uattool executable. open up the `uattool.sln` solution file, select your desired configuration and use right click -> Build on the `uattool` solution. The generated binaries will be placed in the `Debug/` or `Release/` directory depending on your build configuration. 

# Disclaimer
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
