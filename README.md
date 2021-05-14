## MineCraft Server Improved Version

### [YOUTUBE TUTORIAL](https://www.youtube.com/watch?v=Kfk_P6z-FeY)

[Youtube Channel For Tutorial](https://www.youtube.com/channel/UCrngq2SZL98AtiHBQxs3Y5g).

This project is to make installation of Minecraft server easier on any ubuntu/debian running machine. It is heavely focused to host this minecraft server to the cloud.
To make the script i have taken Inspiration from lot of people For Example [BugsWriter](https://github.com/Bugswriter) & None Other than [LukeSmith](https://github.com/LukeSmithxyz). And also the service [PLAYIT.GG](https://playit.gg).

The [Older](https://github.com/lordofwizard/minecraft_server) version is still running but not recommended to install because of some issues with disconnecting and installation of modded minecraft servers.

### Installation 
* Activate a [Google Cloud Shell](https://console.cloud.google.com/) on Google cloud.
* Clone the following repository with the below command..
```
git clone https://github.com/lordofwizard/mcserver
```
* Go inside the mcserver directory
```
cd mcserver
```
* Make everyfile execulable
```
chmod +x *
```
* Now Run the Installation Script with the following command
```
./install
```
* Then start the server with the following command. 
``` 
./startserver
```
* now stop the server {Because it won't load unless you enable the Eula Services
```
cd server
```
```
nano eula.txt
```
{TIP CTRL+o to save{DOn't forget to press enter} and CTRL+x to exit}
* Make the eula=false ---> eula=true
* If you are using like a thirdparty minecraft launcher for example [Tlauncher](https://tlauncher.org/en/) Then do the following steps.
```
nano server.properties
```
* Make the line online-mode=true ---> online-mode=false
{TIP CTRL+o to save{DOn't forget to press enter} and CTRL+x to exit}


* Now go back into the main directory ``` cd .. ```
* Stop the server 
```
./stopserver
```
* Now start the server again
```
./startserver
```
* Everything is now SET and server is running perfectly Fine
* To get the URL of the server do the following command
``` 
screen -r playit
```
* Then claim the url for your server and then you are set.
* Also if you want a different url you can do that with reinstalling the server *note this is remove your privious world*
* 

* To Detach the session you can do it using the following keyboard shortcuts 
* CTRL+a & CTRL+d
* To GO inside the console of the Minecraft server you can paste this command
```
screen -r server
```
* To Detach the session you can do it using the following keyboard shortcuts 
* CTRL+a & CTRL+d

##### TO STOP THE SERVER TYPE ```exit``` and press enter

## TO KEEP THE SERVER ALIVE 
* To keep the server alive we have created a script.py( which requires python3 and pynput installed on your computer).
For Linux and Mac-OS users
```
python3 script.py
```
For BINBOWS 10 or older versions DO the following
```
python script.py
```
### FORGE INSTALLATION
for forge installation steps are a bit different
```
when you get to choose what server you want to download paste the url link
```
```
cd server 
java -jar server.jar --installServer
rm -rf server.jar
```
After this you will get a forge-server.version.jar file you need to rename it as server.jar inorder to work with our screen sessions.
```
mv forge-.jar server.jar
```
Then you are set. Now what you need to do is configure fro the step 6th``` (./startserver)```
Every other thing is the same. 

Enjoy playing minecraft
# THANK YOU
