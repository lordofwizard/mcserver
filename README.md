#Fix of "Version" in Wiki




# To change the version of Java to java 8 to run Minecraft 1.16.5 and below:-

## First, run this command:
`sudo update-alternatives --config java`
It will give you bunch of output something like this
```
There are 2 choices for the alternative java (providing /usr/bin/java).
  Selection    Path                                            Priority   Status------------------------------------------------------------
  0            /usr/lib/jvm/java-11-openjdk-amd64/bin/java      1111      auto mode* 
  1            /usr/lib/jvm/java-11-openjdk-amd64/bin/java 1111      manual mode
  2            /usr/lib/jvm/java-8-openjdk-amd64/jre/bin/java   1081      manual mode
Press <enter> to keep the current choice[*], or type selection number:
```
## Now copy the /usr/ path of Java 8.

Then, do `nano startJavaServer` then it will open up a file like [this file](https://github.com/lordofwizard/mcserver/blob/main/startJavaServer).

Change it to like this:
```
#!/bin/bash
export MAIN_DIR=$PWD
export JAVA=$MAIN_DIR/usr/lib/jvm/java-8-openjdk-amd64/jre/bin/java
cd server

mem=$(grep MemTotal /proc/meminfo | sed -e 's/MemTotal:[ ]*//' | sed -e 's/ kB//') # some new stuff 

mem=$(($mem/1024/1024))

#uncomment this below line only if the server you are running is 1.17+ (if it is uncommented by default keep it as it is).
#$JAVA -Xmx${mem}G -Xms${mem}G -jar server.jar nogui
# for 1.16 or lower uncomment this line
java -Xmx${mem}G -Xms${mem}G -jar server.jar nogui
```
Hope this helps!
