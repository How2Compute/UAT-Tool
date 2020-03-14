# Script that will install UATTool - a tool that simplifies running the Unreal Automation Tool.
# NOTE: This script could be potentially destructive to your system. We cannot accept any responsibility for any damage caused by (parts of) this software, and advise you to review the code before running.
# TODO: Look in to https://gist.github.com/MarkTiedemann/c0adc1701f3f5c215fc2c2d5b1d5efd3!

# Get the downloads path (https://www.reddit.com/r/PowerShell/comments/4x2cph/problem_with_envuserprofiledownloads/)
$downloads_path = Get-ItemPropertyValue 'HKCU:\software\microsoft\windows\currentversion\explorer\shell folders\' -Name '{374DE290-123F-4565-9164-39C4925E467B}';
$zip_path = $downloads_path + '/uattool.zip';
$destination = $downloads_path + '/uattool';

# Confirm the paths/ourplans with the user
Write-Output "UATTool Installation Utility.\nThis utility will download the latest UATTool release from github and save it at $($zip_path).\nFrom there, it will extract the zip file to $($destination).\nFinally, it will add this directory to your path.";

if ((read-host "Please enter 'Y' to contontinue, or 'N' to abort") -ne "Y")
{
    Write-Output "Aborting!";
    exit;
}

# Download latest release into this dir; https://help.github.com/en/github/administering-a-repository/linking-to-releases
Write-Output "Downloading latest release to $($zip_path)..."
(New-Object Net.WebClient).DownloadFile('https://github.com/How2Compute/UAT-Tool/releases/latest/download/uattool.zip',$zip_path);

# Extract the zip file to the destination path
Write-Output "Extracting to $($destination)...";
Expand-Archive -Path $zip_path -DestinationPath $destination -Force # NOTE: Using -Force so we auomatically overide any old versions/it doesn't cause any crashes

# Update the user's Path variable so the tool can be found by the command prompt.
# See also: https://superuser.com/questions/387619/overcoming-the-1024-character-limit-with-setx
Write-Output "Updating system path...";
$additions = $uattool_path
$oldPath = [Environment]::GetEnvironmentVariable('path', 'user');
[Environment]::SetEnvironmentVariable('path', "$($destination);$($oldPath)",'user');

# Done!
Write-Output "Finished installing UATTool!";